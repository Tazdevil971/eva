#![no_std]
use core::alloc::Layout;
use core::ffi::{c_char, c_int, c_void};
use core::time::Duration;
use core::{fmt, mem, ptr};

use eva_kernel::scheduler::thread::{self, Priority};
use eva_kernel::{allocator, kprintln, scheduler};

fn scheduler_tick_signum() -> i32 {
    libc::SIGRTMIN() + 0
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
            sa_flags: libc::SA_ONSTACK | libc::SA_SIGINFO,
        };

        unsafe {
            libc::sigaction(scheduler_tick_signum(), &new, ptr::null_mut());
        }
    }

    // Initialize scheduler
    {
        unsafe {
            // Spawn first thread
            thread::spawn(4096 * 4, Priority::MIN, init_stage2, 0 as _);

            // Launch the scheduler
            scheduler::init();
        }
    }
}

unsafe extern "C" fn init_stage2(_: *mut ()) {
    // Yay this is the first thread!
    // TODO: Initialize preemption through timer_create

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

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    kprintln!("{}", info);

    loop {}
}

struct PortabilityImpl;

impl eva_kernel::portability::Impl for PortabilityImpl {
    fn switchctx_layout() -> Layout {
        Layout::new::<libc::ucontext_t>()
    }

    unsafe fn init_switchctx(
        switchctx_ptr: *mut u8,
        stack_ptr: *mut u8,
        stack_size: usize,
        entry: unsafe extern "C" fn(*mut (), *mut ()) -> !,
        arg1: *mut (),
        arg2: *mut (),
    ) {
        let context_ptr = switchctx_ptr as *mut libc::ucontext_t;

        unsafe {
            libc::getcontext(context_ptr);

            (*context_ptr).uc_link = ptr::null_mut();
            (*context_ptr).uc_stack.ss_sp = stack_ptr as _;
            (*context_ptr).uc_stack.ss_size = stack_size;
            (*context_ptr).uc_stack.ss_flags = 0;

            let entry = mem::transmute(entry);
            libc::makecontext(switchctx_ptr as _, entry, 2, arg1, arg2);
        }
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

static mut SWITCHCTX: *mut libc::ucontext_t = ptr::null_mut();

unsafe extern "C" fn scheduler_tick(
    _sig: c_int,
    _info: *mut libc::siginfo_t,
    ucontext: *mut c_void,
) {
    let context_ptr = ucontext as *mut libc::ucontext_t;

    unsafe {
        let old_ctx = SWITCHCTX;
        *SWITCHCTX = *context_ptr;
        scheduler::tick();

        // For some reason, setcontext with the same ucontext that got as a parameter of this function segfaults
        if old_ctx != SWITCHCTX {
            libc::setcontext(SWITCHCTX);
        }
    }
}
