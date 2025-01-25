//! Common code for all examples

#![no_std]

// Need this to bring in the start-up function

use cortex_r_rt as _;

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits QEMU using a semihosting
/// breakpoint.
#[panic_handler]
#[cfg(target_os = "none")]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use arm_semihosting::{debug, heprintln};
    heprintln!("PANIC: {:#?}", info);
    loop {
        debug::exit(debug::EXIT_FAILURE);
    }
}
