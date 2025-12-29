#![no_std]
use core::alloc::Layout;
use core::arch::naked_asm;
use core::ffi::{c_char, c_int};
use core::time::Duration;
use core::{fmt, mem, ptr};

// Ice cream machine broke, come back another time :(

// This cannot work as it used to depend on libc, but if we fully commit to target_os = "eva" libc no longer works.
// linux_raw_sys can be an escape hatch, but it's missing some bindings, and is overall a pain to
// use, but would COMPLETELY obliterate the dependency on libc, allowing linux userspace applications to use a custom libc.

use eva_kernel::{allocator, kprintln, rt};
use linux_raw_sys::general::*;

static mut GLOBAL_TIMER: libc::timer_t = ptr::null_mut();

fn scheduler_tick_signum() -> i32 {
    libc::SIGRTMIN() + 0
}

fn timer_tick_signum() -> i32 {
    libc::SIGRTMIN() + 1
}

#[unsafe(no_mangle)]
extern "C" fn main(_argc: c_int, _argv: *const *const c_char) -> c_int {
    init_stage1();

    // Jump to the first thread
    unsafe {
        libc::setcontext(SWITCHCTX);
    }

    0
}

fn init_stage1() {
    kprintln!("--- EVA BOOTLOG ---");
    kprintln!("-> EVA logger    [online]");

    // Initialize allocator
    {
        // Simulate a good amount of RAM, 1MB
        const SIZE: usize = 1024 * 1024;

        let mem = unsafe { libc::malloc(SIZE) as *mut u8 };

        unsafe {
            allocator::init(mem, mem.add(SIZE));
        }

        kprintln!("-> EVA allocator [online]");
    }

    // Setup an alternative signal stack
    {
        const SIZE: usize = 16 * 1024;

        let mem = unsafe { libc::malloc(SIZE) };

        let new = libc::stack_t {
            ss_sp: mem,
            ss_size: SIZE,
            ss_flags: 0,
        };

        unsafe {
            libc::sigaltstack(&new, ptr::null_mut());
        }
    }

    // Install scheduler tick signal
    {
        let mut mask = unsafe { mem::zeroed() };
        unsafe {
            libc::sigemptyset(&mut mask);
        }

        let new = libc::sigaction {
            sa_sigaction: scheduler_tick as _,
            sa_restorer: None,
            sa_mask: mask,
            sa_flags: libc::SA_ONSTACK | libc::SA_SIGINFO | libc::SA_RESTART,
        };

        unsafe {
            libc::sigaction(scheduler_tick_signum(), &new, ptr::null_mut());
        }
    }

    // Initialize scheduler
    {
        // Spawn first thread
        rt::spawn(4096 * 16, 0, init_stage2, c"Main", ptr::null_mut());

        unsafe {
            // Launch the scheduler
            rt::init();
        }
    }
}

extern "C" fn init_stage2(_: *mut ()) {
    // Yay this is the first thread!

    // Install timer tick signal
    {
        let mut mask = unsafe { mem::zeroed() };
        unsafe {
            libc::sigemptyset(&mut mask);
            libc::sigaddset(&mut mask, scheduler_tick_signum());
        }

        let new = libc::sigaction {
            sa_sigaction: timer_tick as _,
            sa_restorer: None,
            sa_mask: mask,
            sa_flags: libc::SA_ONSTACK | libc::SA_SIGINFO | libc::SA_RESTART,
        };

        unsafe {
            libc::sigaction(timer_tick_signum(), &new, ptr::null_mut());
        }
    }

    // Initialize preemption timer
    {
        unsafe {
            let mut event: libc::sigevent = mem::zeroed();
            event.sigev_notify = libc::SIGEV_SIGNAL;
            event.sigev_signo = timer_tick_signum();

            libc::timer_create(
                libc::CLOCK_THREAD_CPUTIME_ID,
                &mut event,
                &raw mut GLOBAL_TIMER,
            );

            let mut period: libc::timespec = mem::zeroed();
            period.tv_sec = 0;
            period.tv_nsec = 10_000_000;

            let new = libc::itimerspec {
                it_interval: period,
                it_value: period,
            };
            libc::timer_settime(GLOBAL_TIMER, 0, &new, ptr::null_mut());
        }
    }

    kprintln!("-> EVA scheduler [online]");
    kprintln!("Pivoting control to user code, good luck!");
    kprintln!("--- EVA BOOTLOG ---");

    eva_kernel::kmain::invoke();
}

fn kprint_fmt(args: fmt::Arguments) {
    use core::fmt::Write as _;

    let mut new = unsafe { mem::zeroed() };
    let mut old = unsafe { mem::zeroed() };

    unsafe {
        // Block all possible signals
        libc::sigfillset(&mut new);
        libc::sigprocmask(libc::SIG_BLOCK, &new, &mut old);
    }

    struct Writer;

    impl fmt::Write for Writer {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            let b = s.as_bytes();
            unsafe {
                libc::write(0, b.as_ptr() as _, b.len());
            }
            Ok(())
        }
    }

    let _ = Writer.write_fmt(args);

    unsafe {
        libc::sigprocmask(libc::SIG_SETMASK, &old, ptr::null_mut());
    }
}

struct PortabilityImpl;

impl eva_kernel::port::Impl for PortabilityImpl {
    fn switchctx_layout() -> Layout {
        Layout::new::<libc::ucontext_t>()
    }

    unsafe fn init_switchctx(
        switchctx_ptr: *mut u8,
        stack_ptr: *mut u8,
        stack_size: usize,
        entry: unsafe extern "C" fn(*mut (), *mut (), *mut (), *mut ()) -> !,
        arg1: *mut (),
        arg2: *mut (),
        arg3: *mut (),
        arg4: *mut (),
    ) {
        let context_ptr = switchctx_ptr as *mut libc::ucontext_t;

        unsafe {
            libc::getcontext(context_ptr);

            (*context_ptr).uc_link = ptr::null_mut();
            (*context_ptr).uc_stack.ss_sp = stack_ptr as _;
            (*context_ptr).uc_stack.ss_size = stack_size;
            (*context_ptr).uc_stack.ss_flags = 0;

            let entry = mem::transmute(entry);
            libc::makecontext(switchctx_ptr as _, entry, 4, arg1, arg2, arg3, arg4);
        }
    }

    unsafe fn drop_switchctx(_switchctx_ptr: *mut u8) {
        // NO-OP, linux does not need to clean up anything
    }

    unsafe fn set_global_switchctx(switchctx_ptr: *mut u8) {
        unsafe {
            SWITCHCTX = switchctx_ptr as _;
        }
    }

    fn yield_now() {
        unsafe {
            libc::kill(0, scheduler_tick_signum());
        }
    }

    fn kprint_fmt(args: fmt::Arguments) {
        kprint_fmt(args);
    }

    fn get_time() -> Duration {
        let mut time = unsafe { mem::zeroed() };
        unsafe {
            libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut time);
        }

        Duration::from_secs(time.tv_sec as _) + Duration::from_nanos(time.tv_nsec as _)
    }
}

eva_kernel::set_global_portability_impl!(PortabilityImpl);

static mut OLD_SWITCHCTX: *mut libc::ucontext_t = ptr::null_mut();
static mut SWITCHCTX: *mut libc::ucontext_t = ptr::null_mut();

unsafe fn push_call(ucontext: *mut libc::ucontext_t, func: unsafe extern "C" fn()) {
    // This is pretty much dark magic, we are modifying opaque data structures and injecting a fake return address to invoke a function before actually returning to the previous executing function.
    unsafe {
        let rip = (*ucontext).uc_mcontext.gregs[libc::REG_RIP as usize];
        let rsp = (*ucontext).uc_mcontext.gregs[libc::REG_RSP as usize];

        let rsp = rsp - 8;
        (rsp as *mut i64).write_unaligned(rip);

        (*ucontext).uc_mcontext.gregs[libc::REG_RIP as usize] = func as _;
        (*ucontext).uc_mcontext.gregs[libc::REG_RSP as usize] = rsp;
    }
}

#[unsafe(naked)]
unsafe extern "C" fn swapcontext() {
    naked_asm!(
        "push rax",
        "push rdi",
        "push rsi",
        "push rdx",
        "push rcx",
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        "mov rdi, {old_switchctx}",
        "mov rsi, {switchctx}",
        "call {swapcontext}",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rcx",
        "pop rdx",
        "pop rsi",
        "pop rdi",
        "pop rax",
        "ret",
        old_switchctx = sym OLD_SWITCHCTX,
        switchctx = sym SWITCHCTX,
        swapcontext = sym libc::swapcontext
    );
}

unsafe extern "C" fn scheduler_tick(
    _sig: c_int,
    _info: *mut libc::siginfo_t,
    ucontext: *mut libc::ucontext_t,
) {
    unsafe {
        let old_ctx = SWITCHCTX;
        *SWITCHCTX = *ucontext;
        rt::tick();

        if old_ctx != SWITCHCTX {
            OLD_SWITCHCTX = old_ctx;
            push_call(ucontext, swapcontext);
        }
    }
}

unsafe extern "C" fn timer_tick(
    _sig: c_int,
    _info: *mut libc::siginfo_t,
    _ucontext: *mut libc::ucontext_t,
) {
    unsafe {
        libc::kill(0, scheduler_tick_signum());
    }
}
