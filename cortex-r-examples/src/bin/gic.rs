//! GIC example for Arm Cortex-R52 on an MPS2-AN336

#![no_std]
#![no_main]

// pull in our start-up code
use cortex_r as _;
use cortex_r_examples as _;

use arm_gic::{
    gicv3::{Group, SgiTarget},
    IntId,
};
use arm_semihosting::{debug, hprintln};

type SingleCoreGic = arm_gic::gicv3::GicV3<1>;

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

/// Offset from PERIPHBASE for GIC Distributor
const GICD_BASE_OFFSET: usize = 0x0000_0000usize;

/// Offset from PERIPHBASE for the first GIC Redistributor
const GICR_BASE_OFFSET: usize = 0x0010_0000usize;

fn dump_cpsr() {
    let cpsr = cortex_r::register::Cpsr::read();
    hprintln!("CPSR: {:?}", cpsr);
}

/// The main function of our Rust application.
///
/// Called by [`kmain`].
fn main() -> Result<(), core::fmt::Error> {
    // Get the GIC address by reading CBAR
    let periphbase = cortex_r::register::Cbar::read().periphbase();
    hprintln!("Found PERIPHBASE {:p}", periphbase);
    let gicd_base = periphbase.wrapping_byte_add(GICD_BASE_OFFSET);
    let gicr_base = periphbase.wrapping_byte_add(GICR_BASE_OFFSET);

    // Initialise the GIC.
    hprintln!("Creating GIC driver @ {:p} / {:p}", gicd_base, gicr_base);
    let mut gic: SingleCoreGic =
        unsafe { SingleCoreGic::new(gicd_base.cast(), [gicr_base.cast()]) };
    hprintln!("Calling git.setup(0)");
    gic.setup(0);
    SingleCoreGic::set_priority_mask(0x80);

    // Configure a Software Generated Interrupt for Core 0
    hprintln!("Configure SGI...");
    let sgi_intid = IntId::sgi(3);
    gic.set_interrupt_priority(sgi_intid, Some(0), 0x31);
    gic.set_group(sgi_intid, Some(0), Group::Group1NS);

    hprintln!("gic.enable_interrupt()");
    gic.enable_interrupt(sgi_intid, Some(0), true);

    hprintln!("Enabling interrupts...");
    dump_cpsr();
    unsafe {
        cortex_r::interrupt::enable();
    }
    dump_cpsr();

    // Send it
    hprintln!("Send SGI");
    SingleCoreGic::send_sgi(
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
    while let Some(int_id) = SingleCoreGic::get_and_acknowledge_interrupt() {
        hprintln!("- IRQ handle {:?}", int_id);
        SingleCoreGic::end_interrupt(int_id);
    }
    hprintln!("< IRQ");
}
