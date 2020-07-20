#[cfg(unix)]
fn main() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;
    use std::time::{Duration, Instant};

    use libc::{sighandler_t, SIGINT};

    // The time between ticks of our counter.
    let duration = Duration::from_secs(1) / 2;

    // "SIGINT received" global variable.
    static mut GOT_SIGINT: AtomicBool = AtomicBool::new(false);

    unsafe {
        // Initially, "SIGINT received" is false.
        GOT_SIGINT.store(false, Ordering::Release);
        // Interrupt handler that handles the SIGINT signal
        unsafe fn handle_sigint() {
            // It is dangerous to perform any system calls in interrupts, so just set the atomic
            // "SIGINT received" global to true when it arrives.
            GOT_SIGINT.store(true, Ordering::Release);
        }
        // Make handle_sigint the signal handler for SIGINT.
        libc::signal(SIGINT, handle_sigint as sighandler_t);
    }

    // Get the start time...
    let start = Instant::now();

    // Integer counter
    let mut i = 0u32;

    // Every `duration`...
    loop {
        thread::sleep(duration);

        // Break if SIGINT was handled
        if unsafe { GOT_SIGINT.load(Ordering::Acquire) } {
            break;
        }

        // Otherwise, increment and display the integer and continue the loop.
        i += 1;
        println!("{}", i);
    }

    // Get the elapsed time.
    let elapsed = start.elapsed();

    // Print the difference and exit
    println!("Program has run for {} seconds", elapsed.as_secs());
}

#[cfg(not(unix))]
fn main() {
    println!("Not supported on this platform");
}
