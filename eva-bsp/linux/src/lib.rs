#![no_std]
use core::alloc::Layout;
use core::arch::{asm, naked_asm};
use core::ffi::{c_int, c_ulong, c_void};
use core::mem::{self, offset_of};
use core::ptr::{self, addr_of_mut};
use core::time::Duration;

use eva_kernel::{allocator, kprintln, port, rt};
use linux_raw_sys::general::{
    __kernel_clockid_t, __kernel_pid_t, __kernel_timer_t, _fpstate_64, CLOCK_THREAD_CPUTIME_ID,
    SA_ONSTACK, SA_RESTART, SA_RESTORER, SA_SIGINFO, SIGEV_SIGNAL, SIGRTMIN, itimerspec,
    kernel_sigaction, kernel_sigset_t, sigevent, siginfo, stack_t, timespec, ucontext,
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

        mov rax, {switchctx}
        mov rdi, [rax + {offset_rdi}]
        mov rsi, [rax + {offset_rsi}]
        mov rdx, [rax + {offset_rdx}]
        mov rcx, [rax + {offset_rcx}]
        mov rsp, [rax + {offset_rsp}]
        mov rbp, [rax + {offset_rbp}]
        mov rax, [rax + {offset_rip}]
        jmp rax
        ",
        init_stage1 = sym init_stage1,
        switchctx = sym SWITCHCTX,
        offset_rdi = const offset_of!(SwitchCtx, rdi),
        offset_rsi = const offset_of!(SwitchCtx, rsi),
        offset_rdx = const offset_of!(SwitchCtx, rdx),
        offset_rcx = const offset_of!(SwitchCtx, rcx),
        offset_rsp = const offset_of!(SwitchCtx, rsp),
        offset_rbp = const offset_of!(SwitchCtx, rbp),
        offset_rip = const offset_of!(SwitchCtx, rip)
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
        rt::spawn(64 * 1024, 0, init_stage2, c"Main", ptr::null_mut());

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

const DEFAULT_RFLAGS: u64 = 0x202;

#[derive(Debug)]
#[repr(C, align(16))]
struct FxSave([u8; 512]);

// Check that _fpstate_64 is actually containable in FxSave
const _: () = {
    assert!(mem::size_of::<FxSave>() >= mem::size_of::<_fpstate_64>());
};

#[repr(C)]
#[derive(Debug)]
struct SwitchCtx {
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    rdi: u64,
    rsi: u64,
    rbp: u64,
    rbx: u64,
    rdx: u64,
    rax: u64,
    rcx: u64,
    rsp: u64,
    rip: u64,
    eflags: u64,
    fxsave: FxSave,
}

impl SwitchCtx {
    pub fn init(
        stack_ptr: *mut u8,
        entry: unsafe extern "C" fn(*mut (), *mut (), *mut (), *mut ()) -> !,
        arg1: *mut (),
        arg2: *mut (),
        arg3: *mut (),
        arg4: *mut (),
    ) -> Self {
        Self {
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rdi: arg1 as _,
            rsi: arg2 as _,
            rbp: stack_ptr as _,
            rbx: 0,
            rdx: arg3 as _,
            rax: 0,
            rcx: arg4 as _,
            rsp: stack_ptr as _,
            rip: entry as _,
            eflags: DEFAULT_RFLAGS,
            fxsave: FxSave([0; 512]),
        }
    }

    pub unsafe fn save_from(&mut self, ucontext: *mut ucontext) {
        unsafe {
            self.r8 = (*ucontext).uc_mcontext.r8;
            self.r9 = (*ucontext).uc_mcontext.r9;
            self.r10 = (*ucontext).uc_mcontext.r10;
            self.r11 = (*ucontext).uc_mcontext.r11;
            self.r12 = (*ucontext).uc_mcontext.r12;
            self.r13 = (*ucontext).uc_mcontext.r13;
            self.r14 = (*ucontext).uc_mcontext.r14;
            self.r15 = (*ucontext).uc_mcontext.r15;
            self.rdi = (*ucontext).uc_mcontext.rdi;
            self.rsi = (*ucontext).uc_mcontext.rsi;
            self.rbp = (*ucontext).uc_mcontext.rbp;
            self.rbx = (*ucontext).uc_mcontext.rbx;
            self.rdx = (*ucontext).uc_mcontext.rdx;
            self.rax = (*ucontext).uc_mcontext.rax;
            self.rcx = (*ucontext).uc_mcontext.rcx;
            self.rsp = (*ucontext).uc_mcontext.rsp;
            self.rip = (*ucontext).uc_mcontext.rip;
            self.eflags = (*ucontext).uc_mcontext.eflags;

            ptr::copy_nonoverlapping(
                (*ucontext).uc_mcontext.fpstate as *const u8,
                self.fxsave.0.as_mut_ptr(),
                mem::size_of::<_fpstate_64>(),
            );
        }
    }

    pub unsafe fn restore_to(&self, ucontext: *mut ucontext) {
        unsafe {
            (*ucontext).uc_mcontext.r8 = self.r8;
            (*ucontext).uc_mcontext.r9 = self.r9;
            (*ucontext).uc_mcontext.r10 = self.r10;
            (*ucontext).uc_mcontext.r11 = self.r11;
            (*ucontext).uc_mcontext.r12 = self.r12;
            (*ucontext).uc_mcontext.r13 = self.r13;
            (*ucontext).uc_mcontext.r14 = self.r14;
            (*ucontext).uc_mcontext.r15 = self.r15;
            (*ucontext).uc_mcontext.rdi = self.rdi;
            (*ucontext).uc_mcontext.rsi = self.rsi;
            (*ucontext).uc_mcontext.rbp = self.rbp;
            (*ucontext).uc_mcontext.rbx = self.rbx;
            (*ucontext).uc_mcontext.rdx = self.rdx;
            (*ucontext).uc_mcontext.rax = self.rax;
            (*ucontext).uc_mcontext.rcx = self.rcx;
            (*ucontext).uc_mcontext.rsp = self.rsp;
            (*ucontext).uc_mcontext.rip = self.rip;
            (*ucontext).uc_mcontext.eflags = self.eflags;

            ptr::copy_nonoverlapping(
                self.fxsave.0.as_ptr(),
                (*ucontext).uc_mcontext.fpstate as *mut u8,
                mem::size_of::<_fpstate_64>(),
            );
        }
    }
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
            stack_ptr.cast::<u64>().write(u64::MAX);
            // Intentionally misalign the stack, this is what x64 expects on function enter
            let stack_ptr = stack_ptr.sub(8);

            switchctx_ptr
                .cast::<SwitchCtx>()
                .write(SwitchCtx::init(stack_ptr, entry, arg1, arg2, arg3, arg4))
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

static mut SWITCHCTX: *mut SwitchCtx = ptr::null_mut();

unsafe extern "C" fn scheduler_tick(_sig: c_int, _info: *mut siginfo, ucontext: *mut ucontext) {
    unsafe {
        let old_ctx = SWITCHCTX;
        rt::tick();

        if old_ctx != SWITCHCTX {
            (*old_ctx).save_from(ucontext);
            (*SWITCHCTX).restore_to(ucontext);
        }
    }
}

unsafe extern "C" fn timer_tick(_sig: c_int) {
    // Trigger an immediate yield
    unsafe {
        sys_kill(0, SCHEDULER_TICK_SIGNUM);
    }
}
