//! Semihosting hello-world for Arm Cortex-R

#![no_std]
#![no_main]

// pull in our start-up code
use cortex_r as _;
use cortex_r_examples as _;

use arm_semihosting::{debug, heprintln, hprintln};

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `lib.rs`.
#[no_mangle]
pub extern "C" fn kmain() {
    if let Err(e) = main() {
        panic!("main returned {:?}", e);
    }
}

/// The main function of our Rust application.
///
/// Called by [`kmain`].
fn main() -> Result<(), core::fmt::Error> {
    let x = 1.0f64;
    let y = x * 2.0;
    hprintln!("Hello, this is semihosting! x = {:0.3}, y = {:0.3}", x, y);
    panic!("I am a panic");
}

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits QEMU using a semihosting
/// breakpoint.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    heprintln!("PANIC: {:?}", info);
    loop {
        debug::exit(debug::EXIT_FAILURE);
    }
}
