#![no_std]
use core::alloc::Layout;
use core::arch::{asm, naked_asm};
use core::ffi::{c_int, c_ulong, c_void};
use core::mem::{self, offset_of, size_of};
use core::ptr::{self, addr_of_mut};
use core::time::Duration;

use eva_kernel::{allocator, kprintln, port, rt};
use linux_raw_sys::general::{
    __kernel_clockid_t, __kernel_pid_t, __kernel_timer_t, CLOCK_THREAD_CPUTIME_ID, SA_ONSTACK,
    SA_RESTART, SA_RESTORER, SA_SIGINFO, SIGEV_SIGNAL, SIGRTMIN, itimerspec, kernel_sigaction,
    kernel_sigset_t, sigevent, siginfo, stack_t, timespec, ucontext,
};

#[cfg(not(target_arch = "x86_64"))]
compile_error!("only x86_64 is supported!");

static mut GLOBAL_TIMER: __kernel_timer_t = 0;

const SCHEDULER_TICK_SIGNUM: c_int = SIGRTMIN as c_int + 0;
const TIMER_TICK_SIGNUM: c_int = SIGRTMIN as c_int + 1;

#[unsafe(no_mangle)]
#[unsafe(naked)]
unsafe extern "C" fn _start() {
    naked_asm!(
        "
        call {init_stage1}

        mov rsi, {switchctx}
        mov rcx, [rsi + 16]
        mov rdx, [rsi + 24]
        mov rbp, [rsi + 32]
        mov rsp, [rsi + 40]

        pop rdi
        pop rsi
        popf
        ret
        ",
        init_stage1 = sym init_stage1,
        switchctx = sym SWITCHCTX
    );
}

unsafe extern "C" fn init_stage1() {
    kprintln!("--- EVA BOOTLOG ---");
    kprintln!("-> EVA logger    [online]");

    // Initialize allocator
    {
        unsafe extern "C" {
            unsafe static mut __heap_start: u8;
            unsafe static mut __heap_end: u8;
        }

        kprintln!(
            "heap {:?} {:?}",
            addr_of_mut!(__heap_start),
            addr_of_mut!(__heap_end)
        );

        unsafe {
            allocator::init(addr_of_mut!(__heap_start), addr_of_mut!(__heap_end));
        }

        kprintln!("-> EVA allocator [online]");
    }

    // Setup the signal stack
    {
        unsafe extern "C" {
            unsafe static mut __irq_stack_bottom: u8;
            unsafe static mut __irq_stack_top: u8;
        }

        unsafe {
            let size = addr_of_mut!(__irq_stack_top)
                .byte_offset_from_unsigned(addr_of_mut!(__irq_stack_bottom));

            let new = stack_t {
                ss_sp: addr_of_mut!(__irq_stack_bottom) as _,
                ss_size: size as _,
                ss_flags: 0,
            };

            sys_sigalstack(&new, ptr::null_mut());
        }
    }

    // Install scheduler tick signal
    {
        unsafe {
            let new = kernel_sigaction {
                sa_handler_kernel: Some(mem::transmute(scheduler_tick as *const ())),
                sa_restorer: Some(restorer),
                sa_mask: mem::zeroed(),
                sa_flags: (SA_RESTORER | SA_ONSTACK | SA_SIGINFO | SA_RESTART) as _,
            };

            sys_sigaction(SCHEDULER_TICK_SIGNUM, &new, ptr::null_mut());
        }
    }

    // Install timer tick signal
    {
        unsafe {
            let mut mask = mem::zeroed();
            sigaddset(&mut mask, SCHEDULER_TICK_SIGNUM);

            let new = kernel_sigaction {
                sa_handler_kernel: Some(timer_tick),
                sa_restorer: Some(restorer),
                sa_mask: mask,
                sa_flags: (SA_RESTORER | SA_ONSTACK | SA_RESTART) as _,
            };

            sys_sigaction(TIMER_TICK_SIGNUM, &new, ptr::null_mut());
        }
    }

    {
        // Spawn first thread
        rt::spawn(1024 * 256, 0, init_stage2, c"Main", ptr::null_mut());

        unsafe {
            // Launch the scheduler
            rt::init();
        }
    }
}

extern "C" fn init_stage2(_: *mut ()) {
    // Yay this is the first thread!

    // Initialize the preemption timer
    {
        unsafe {
            let mut event: sigevent = mem::zeroed();
            event.sigev_notify = SIGEV_SIGNAL as _;
            event.sigev_signo = TIMER_TICK_SIGNUM;

            sys_timer_create(
                CLOCK_THREAD_CPUTIME_ID as _,
                &mut event,
                &raw mut GLOBAL_TIMER,
            );

            let mut period: timespec = mem::zeroed();
            period.tv_sec = 0;
            period.tv_nsec = 10_000_000;

            let new = itimerspec {
                it_interval: period,
                it_value: period,
            };

            sys_timer_settime(GLOBAL_TIMER, 0, &new, ptr::null_mut());
        }
    }

    kprintln!("-> EVA scheduler [online]");
    kprintln!("Pivoting control to user code, good luck!");
    kprintln!("--- EVA BOOTLOG ---");

    eva_kernel::kmain::invoke();
}

#[unsafe(naked)]
unsafe extern "C" fn restorer() {
    naked_asm!(
        "
        mov rax, {}
        syscall
        ",
        const linux_raw_sys::general::__NR_rt_sigreturn
    )
}

fn sigaddset(set: &mut kernel_sigset_t, signo: c_int) -> c_int {
    let word = (signo as usize - 1) / (mem::size_of::<c_ulong>() * 8);
    let mask = 1 << ((signo as usize - 1) % (mem::size_of::<c_ulong>() * 8));
    set.sig[word] |= mask;
    0
}

#[inline(always)]
unsafe fn sys_read(fd: c_int, buf: *mut c_void, count: usize) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") linux_raw_sys::general::__NR_write as usize => ret,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
}

#[inline(always)]
unsafe fn sys_write(fd: c_int, buf: *const c_void, count: usize) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") linux_raw_sys::general::__NR_write as usize => ret,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags, readonly)
        );
        ret
    }
}

#[inline(always)]
unsafe fn sys_sigalstack(ss: *const stack_t, old_ss: *mut stack_t) -> c_int {
    unsafe {
        let ret: usize;
        asm!(
            "syscall",
            inlateout("rax") linux_raw_sys::general::__NR_sigaltstack as usize => ret,
            in("rdi") ss,
            in("rsi") old_ss,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
        ret as _
    }
}

#[inline(always)]
unsafe fn sys_sigaction(
    signum: c_int,
    act: *const kernel_sigaction,
    oldact: *mut kernel_sigaction,
) -> c_int {
    unsafe {
        let ret: usize;
        asm!(
            "syscall",
            inlateout("rax") linux_raw_sys::general::__NR_rt_sigaction as usize => ret,
            in("rdi") signum,
            in("rsi") act,
            in("rdx") oldact,
            in("r10") mem::size_of::<kernel_sigset_t>(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
        ret as _
    }
}

#[inline(always)]
unsafe fn sys_kill(pid: __kernel_pid_t, sig: c_int) -> c_int {
    unsafe {
        let ret: usize;
        asm!(
            "syscall",
            inlateout("rax") linux_raw_sys::general::__NR_kill as usize => ret,
            in("rdi") pid,
            in("rsi") sig,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags, readonly)
        );
        ret as _
    }
}

#[inline(always)]
unsafe fn sys_timer_create(
    clockid: __kernel_clockid_t,
    sevp: *mut sigevent,
    timerid: *mut __kernel_timer_t,
) -> c_int {
    unsafe {
        let ret: usize;
        asm!(
            "syscall",
            inlateout("rax") linux_raw_sys::general::__NR_timer_create as usize => ret,
            in("rdi") clockid,
            in("rsi") sevp,
            in("rdx") timerid,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
        ret as _
    }
}

#[inline(always)]
unsafe fn sys_timer_settime(
    timerid: __kernel_timer_t,
    flags: c_int,
    new_value: *const itimerspec,
    old_value: *mut itimerspec,
) -> c_int {
    unsafe {
        let ret: usize;
        asm!(
            "syscall",
            inlateout("rax") linux_raw_sys::general::__NR_timer_settime as usize => ret,
            in("rdi") timerid,
            in("rsi") flags,
            in("rdx") new_value,
            in("r10") old_value,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
        ret as _
    }
}

#[inline(always)]
unsafe fn sys_clock_gettime(clockid: __kernel_clockid_t, tp: *mut timespec) -> c_int {
    unsafe {
        let ret: usize;
        asm!(
            "syscall",
            inlateout("rax") linux_raw_sys::general::__NR_clock_gettime as usize => ret,
            in("rdi") clockid,
            in("rsi") tp,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
        ret as _
    }
}

#[repr(C)]
struct SwitchStack {
    rdi: u64,
    rsi: u64,
    eflags: u64,
    rip: u64,
}

const DEFAULT_RFLAGS: u64 = 0x202;

#[derive(Debug)]
#[repr(C, align(16))]
struct FxSave([u8; 512]);

#[derive(Debug)]
#[repr(C, align(64))]
struct XSave([u8; 1024]);

#[repr(C)]
#[derive(Debug)]
struct SwitchCtx {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rbp: u64,
    rsp: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    fxsave: FxSave,
    xsave: XSave,
}

struct PortabilityImpl;

impl port::Impl for PortabilityImpl {
    fn switchctx_layout() -> Layout {
        Layout::new::<SwitchCtx>()
    }

    unsafe fn init_switchctx(
        switchctx_ptr: *mut u8,
        stack_ptr: *mut u8,
        _stack_size: usize,
        entry: unsafe extern "C" fn(*mut (), *mut (), *mut (), *mut ()) -> !,
        arg1: *mut (),
        arg2: *mut (),
        arg3: *mut (),
        arg4: *mut (),
    ) {
        unsafe {
            let stack_ptr = stack_ptr.sub(size_of::<SwitchStack>()).sub(8);

            stack_ptr.cast::<SwitchStack>().write(SwitchStack {
                rdi: arg1 as _,
                rsi: arg2 as _,
                eflags: DEFAULT_RFLAGS,
                rip: entry as _,
            });

            switchctx_ptr.cast::<SwitchCtx>().write(SwitchCtx {
                rax: 0,
                rbx: 0,
                rcx: arg4 as _,
                rdx: arg3 as _,
                rbp: stack_ptr as _,
                rsp: stack_ptr as _,
                r8: 0,
                r9: 0,
                r10: 0,
                r11: 0,
                r12: 0,
                r13: 0,
                r14: 0,
                r15: 0,
                fxsave: FxSave([0; 512]),
                xsave: XSave([0; 1024]),
            })
        }
    }

    unsafe fn drop_switchctx(switchctx_ptr: *mut u8) {
        // Actually a NO-OP, included for correctness
        unsafe {
            switchctx_ptr.cast::<SwitchCtx>().drop_in_place();
        }
    }

    unsafe fn set_global_switchctx(switchctx_ptr: *mut u8) {
        unsafe {
            SWITCHCTX = switchctx_ptr as _;
        }
    }

    fn yield_now() {
        unsafe {
            sys_kill(0, SCHEDULER_TICK_SIGNUM);
        }
    }

    fn kwrite(data: &[u8]) -> usize {
        unsafe { sys_write(0, data.as_ptr() as _, data.len()) }
    }

    fn kread(data: &mut [u8]) -> usize {
        unsafe { sys_read(0, data.as_ptr() as _, data.len()) }
    }

    fn get_time() -> Duration {
        let mut time = unsafe { mem::zeroed() };
        unsafe {
            sys_clock_gettime(CLOCK_THREAD_CPUTIME_ID as _, &mut time);
        }

        Duration::new(time.tv_sec as _, time.tv_nsec as _)
    }
}

eva_kernel::set_global_portability_impl!(PortabilityImpl);

#[unsafe(naked)]
unsafe extern "C" fn swap_context() {
    naked_asm!(
        "
        mov rdi, {old_switchctx}
        mov rsi, {switchctx}

        mov [rdi + 0], rax
        mov [rdi + 8], rbx
        mov [rdi + 16], rcx
        mov [rdi + 24], rdx
        mov [rdi + 32], rbp
        mov [rdi + 40], rsp
        mov [rdi + 48], r8
        mov [rdi + 56], r9
        mov [rdi + 64], r10
        mov [rdi + 72], r11
        mov [rdi + 80], r12
        mov [rdi + 88], r13
        mov [rdi + 96], r14
        mov [rdi + 104], r15

        fxsave [rdi + {fxsave_offset}]
        xsave [rdi + {xsave_offset}]

        mov rax, [rsi + 0]
        mov rbx, [rsi + 8]
        mov rcx, [rsi + 16]
        mov rdx, [rsi + 24]
        mov rbp, [rsi + 32]
        mov rsp, [rsi + 40]
        mov r8, [rsi + 48]
        mov r9, [rsi + 56]
        mov r10, [rsi + 64]
        mov r11, [rsi + 72]
        mov r12, [rsi + 80]
        mov r13, [rsi + 88]
        mov r14, [rsi + 96]
        mov r15, [rsi + 104]

        fxrstor [rsi + {fxsave_offset}]
        xrstor [rsi + {xsave_offset}]
        
        pop rdi
        pop rsi
        popf
        ret
        ",
        old_switchctx = sym OLD_SWITCHCTX,
        switchctx = sym SWITCHCTX,
        fxsave_offset = const offset_of!(SwitchCtx, fxsave),
        xsave_offset = const offset_of!(SwitchCtx, xsave)
    );
}

static mut OLD_SWITCHCTX: *mut SwitchCtx = ptr::null_mut();
static mut SWITCHCTX: *mut SwitchCtx = ptr::null_mut();

unsafe fn push_swap_context(ucontext: *mut ucontext) {
    // This is pretty much dark magic, we are modifying opaque data structures and injecting a fake return address to invoke a function before actually returning to the previous executing function.
    unsafe {
        let rip = (*ucontext).uc_mcontext.rip;
        let rsp = (*ucontext).uc_mcontext.rsp;
        let rdi = (*ucontext).uc_mcontext.rdi;
        let rsi = (*ucontext).uc_mcontext.rsi;
        let eflags = (*ucontext).uc_mcontext.eflags;

        let rsp = rsp as *mut u8;
        let rsp = rsp.sub(size_of::<SwitchStack>());
        rsp.cast::<SwitchStack>().write(SwitchStack {
            rdi,
            rsi,
            eflags,
            rip,
        });

        (*ucontext).uc_mcontext.rip = swap_context as *const () as _;
        (*ucontext).uc_mcontext.rsp = rsp as _;
    }
}

unsafe extern "C" fn scheduler_tick(_sig: c_int, _info: *mut siginfo, ucontext: *mut ucontext) {
    unsafe {
        let old_ctx = SWITCHCTX;
        rt::tick();

        if old_ctx != SWITCHCTX {
            OLD_SWITCHCTX = old_ctx;
            push_swap_context(ucontext);
        }
    }
}

unsafe extern "C" fn timer_tick(_sig: c_int) {
    // Trigger an immediate yield
    unsafe {
        sys_kill(0, SCHEDULER_TICK_SIGNUM);
    }
}
