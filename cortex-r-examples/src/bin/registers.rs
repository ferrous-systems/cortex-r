//! Registers example for Arm Cortex-R

#![no_std]
#![no_main]

// pull in our start-up code
use cortex_r as _;
use cortex_r_examples as _;

use arm_semihosting::hprintln;

extern "C" {
    static _stack_top: u32;
}

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `cortex-m-rt`.
#[no_mangle]
pub extern "C" fn kmain() {
    hprintln!("{:?}", cortex_r::register::Midr::read());
    hprintln!("{:?}", cortex_r::register::Cpsr::read());

    hprintln!("_stack_top: {:p}", core::ptr::addr_of!(_stack_top));

    hprintln!(
        "{:?} before setting C, I and Z",
        cortex_r::register::Sctlr::read()
    );
    cortex_r::register::Sctlr::modify(|w| {
        w.set_c();
        w.set_i();
        w.set_z();
    });
    hprintln!("{:?} after", cortex_r::register::Sctlr::read());

    arm_semihosting::debug::exit(arm_semihosting::debug::EXIT_SUCCESS);
}
