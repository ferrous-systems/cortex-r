//! GIC example for Arm Cortex-R52 on an MPS2-AN336

#![no_std]
#![no_main]

// pull in our start-up code
use cortex_r as _;
use cortex_r_examples as _;

use arm_gic::{
    gicv3::{GicV3, Group, SgiTarget},
    IntId,
};
use arm_semihosting::{debug, hprintln};

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `cortex-m-rt`.
#[no_mangle]
pub extern "C" fn kmain() {
    if let Err(e) = main() {
        panic!("main returned {:?}", e);
    }
    debug::exit(debug::EXIT_SUCCESS);
}

// Base addresses of the GICv3 distributor and redistributor.
const GICD_BASE_ADDRESS: *mut u64 = 0xF000_0000usize as _;
const GICR_BASE_ADDRESS: *mut u64 = 0xF010_0000usize as _;

fn dump_cpsr() {
    let cpsr = cortex_r::register::Cpsr::read();
    hprintln!("CPSR: {:?}", cpsr);
}

/// The main function of our Rust application.
///
/// Called by [`kmain`].
fn main() -> Result<(), core::fmt::Error> {
    // Initialise the GIC.
    hprintln!("Creating GIC driver...");
    let mut gic = unsafe { GicV3::new(GICD_BASE_ADDRESS, GICR_BASE_ADDRESS) };
    hprintln!("Calling git.setup()");
    gic.setup();
    hprintln!("Configure SGI");
    // Configure an SGI and then send it to ourself.
    let sgi_intid = IntId::sgi(3);
    GicV3::set_priority_mask(0xFF);
    gic.set_interrupt_priority(sgi_intid, 0x31);
    gic.set_group(sgi_intid, Group::Group1NS);
    hprintln!("gic.enable_interrupt()");
    gic.enable_interrupt(sgi_intid, true);
    hprintln!("Enabling interrupts...");
    dump_cpsr();
    unsafe {
        cortex_r::interrupt::enable();
    }
    dump_cpsr();
    hprintln!("Send SGI");
    GicV3::send_sgi(
        sgi_intid,
        SgiTarget::List {
            affinity3: 0,
            affinity2: 0,
            affinity1: 0,
            target_list: 0b1,
        },
    );

    for _ in 0..1_000_000 {
        cortex_r::asm::nop();
    }

    Ok(())
}

#[no_mangle]
unsafe extern "C" fn _irq_handler() {
    hprintln!("> IRQ");
    while let Some(int_id) = GicV3::get_and_acknowledge_interrupt() {
        hprintln!("- IRQ handle {:?}", int_id);
        GicV3::end_interrupt(int_id);
    }
    hprintln!("< IRQ");
}
