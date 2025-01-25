//! SVC (software interrupt) example for Arm Cortex-R

#![no_std]
#![no_main]

// pull in our start-up code
use cortex_r as _;
use cortex_r_examples as _;

use arm_semihosting::hprintln;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `cortex-m-rt`.
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
    let x = 1;
    let y = x + 1;
    let z = (y as f64) * 1.5;
    hprintln!("x = {}, y = {}, z = {:0.3}", x, y, z);
    cortex_r::svc!(0xABCDEF);
    hprintln!("x = {}, y = {}, z = {:0.3}", x, y, z);
    panic!("I am an example panic");
}

/// This is our SVC exception handler
#[no_mangle]
unsafe extern "C" fn _svc_handler(arg: u32, state: *const [u32; 6]) {
    hprintln!(
        "SVC interrupt, with arg={:#06x}, state={:08x?}",
        arg,
        state.read()
    );
}
