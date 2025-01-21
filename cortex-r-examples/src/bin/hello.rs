//! Semihosting hello-world for Arm Cortex-R

#![no_std]
#![no_main]

// pull in our start-up code
use cortex_r as _;
use cortex_r_examples as _;

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
    cortex_r_semihosting::hprintln!("Hello, this is semihosting!");
    panic!("I am a panic");
}

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits QEMU using a semihosting
/// breakpoint.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    cortex_r_semihosting::heprintln!("PANIC: {:?}", info);
    const SYS_REPORTEXC: u32 = 0x18;
    loop {
        // Exit, using semihosting
        unsafe {
            core::arch::asm!(
                "svc 0x123456",
                in("r0") SYS_REPORTEXC,
                in("r1") 0x20026
            )
        }
    }
}
