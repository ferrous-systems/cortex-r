//! Run-time support for Arm Cortex-R
//!
//! This library implements a simple Arm vector table, suitable for getting into
//! a Rust application running in System Mode.
//!
//! Transferring from System Mode to User Mode (i.e. implementing an RTOS) is
//! not handled here.
//!
//! If your processor starts in Hyp mode, this runtime will be transfer it to
//! System mode. If you wish to write a hypervisor, you will need to replace
//! this library with something more advanced.
//!
//! We assume the following global symbols exist:
//!
//! * `__start` - a Reset handler. Our linker script PROVIDEs a default function
//!   at `_default_start` but you can override it.
//! * `_stack_top` - the address of the top of some region of RAM that we can
//!   use as stack space, with eight-byte alignment. Our linker script PROVIDEs
//!   a default pointing at the top of RAM.
//! * `_fiq_stack_size` - the number of bytes to be reserved for stack space
//!   when in FIQ mode; must be a multiple of 8.
//! * `_irq_stack_size` - the number of bytes to be reserved for stack space
//!   when in FIQ mode; must be a multiple of 8.
//! * `_svc_stack_size` - the number of bytes to be reserved for stack space
//!   when in SVC mode; must be a multiple of 8.F
//! * `_svc_handler` - an `extern "C"` function to call when an SVC Exception
//!   occurs. Our linker script PROVIDEs a default function at
//!   `_default_handler` but you can override it.
//! * `_irq_handler` - an `extern "C"` function to call when an Interrupt
//!   occurs. Our linker script PROVIDEs a default function at
//!   `_default_handler` but you can override it.
//! * `_asm_fiq_handler` - a naked function to call when a Fast Interrupt
//!   Request (FIQ) occurs. Our linker script PROVIDEs a default function at
//!   `_asm_default_fiq_handler` but you can override it.
//! * `_asm_undefined_handler` - a naked function to call when an Undefined
//!   Exception occurs. Our linker script PROVIDEs a default function at
//!   `_asm_default_handler` but you can override it.
//! * `_asm_prefetch_handler` - a naked function to call when an Prefetch
//!   Exception occurs. Our linker script PROVIDEs a default function at
//!   `_asm_default_handler` but you can override it.
//! * `_asm_abort_handler` - a naked function to call when an Abort Exception
//!   occurs. Our linker script PROVIDEs a default function at
//!   `_asm_default_handler` but you can override it.
//! * `kmain` - the `extern "C"` entry point to your application.
//! * `__sdata` - the start of initialised data in RAM. Must be 4-byte aligned.
//! * `__edata` - the end of initialised data in RAM. Must be 4-byte aligned.
//! * `__sidata` - the start of the initialisation values for data, in read-only
//!   memory. Must be 4-byte aligned.
//! * `__sbss` - the start of zero-initialised data in RAM. Must be 4-byte
//!   aligned.
//! * `__ebss` - the end of zero-initialised data in RAM. Must be 4-byte
//!   aligned.
//!
//! On start-up, the memory between `__sbss` and `__ebss` is zeroed, and the
//! memory between `__sdata` and `__edata` is initialised with the data found at
//! `__sidata`.
//!
//! This library produces global symbols called:
//!
//! * `_vector_table` - the start of the interrupt vector table
//! * `_default_start` - the default Reset handler, that sets up some stacks and
//!   calls an `extern "C"` function called `kmain`.
//! * `_asm_default_fiq_handler` - an FIQ handler that just spins
//! * `_asm_default_handler` - an exception handler that just spins
//! * `_asm_svc_handler` - assembly language trampoline for SVC Exceptions that
//!   calls `_svc_handler`
//! * `_asm_irq_handler` - assembly language trampoline for Interrupts that
//!   calls `_irq_handler`
//!
//! The assembly language trampolines are required because Armv7-R (and Armv8-R)
//! processors do not save a great deal of state on entry to an exception
//! handler, unlike Armv7-M (and other M-Profile) processors. We must therefore
//! save this state to the stack using assembly language, before transferring to
//! an `extern "C"` function. We do not change modes before entering that
//! `extern "C"` function - that's for the handler to deal with as it wishes. We
//! supply a default handler that prints an error message to Semihosting so you
//! know if you hit an unexpected exception. Because FIQ is often
//! performance-sensitive, we don't supply an FIQ trampoline; if you want to use
//! FIQ, you have to write your own assembly routine, allowing you to preserve
//! only whatever state is important to you.
//!
//! If our start-up routine doesn't work for you (e.g. if you have to initialise
//! your memory controller before you touch RAM), supply your own `_start`
//! function (but feel free to call our `_default_start` as part of it).

#![no_std]

/// Our default exception handler.
///
/// We end up here if an exception fires and the weak 'PROVIDE' in the link.x
/// file hasn't been over-ridden.
#[no_mangle]
pub extern "C" fn _default_handler() {
    arm_semihosting::hprintln!("Unhandled exception!");
    arm_semihosting::debug::exit(arm_semihosting::debug::EXIT_FAILURE);
}

// The Interrupt Vector Table, and some default assembly-language handler.
#[cfg(any(arm_architecture = "v7-r", arm_architecture = "v8-r"))]
core::arch::global_asm!(
    r#"
    .section .vector_table
    .code 32
    .align 0

    .global _vector_table
    _vector_table:
        ldr     pc, =_start
        ldr     pc, =_asm_undefined_handler
        ldr     pc, =_asm_svc_handler
        ldr     pc, =_asm_prefetch_handler
        ldr     pc, =_asm_abort_handler
        nop
        ldr     pc, =_asm_irq_handler
        ldr     pc, =_asm_fiq_handler

    .section .text.handlers

    .global _asm_default_fiq_handler
    _asm_default_fiq_handler:
        b       _asm_default_fiq_handler

    .global _asm_default_handler
    _asm_default_handler:
        b       _asm_default_handler
    "#
);

/// This macro expands to code for saving context on entry to an exception
/// handler.
///
/// It should match `restore_context!`.
///
/// On entry to this block, we assume that we are in exception context.
#[cfg(all(
    any(arm_architecture = "v7-r", arm_architecture = "v8-r"),
    not(any(target_abi = "eabihf", feature = "eabi-fpu"))
))]
macro_rules! save_context {
    () => {
        r#"
        // save preserved registers (and gives us some working area)
        push    {{r0-r3}}
        // align SP down to eight byte boundary
        mov     r0, sp
        and     r0, r0, 7
        sub     sp, r0
        // push alignment amount, and final preserved register
        push    {{r0, r12}}
        "#
    };
}

/// This macro expands to code for restoring context on exit from an exception
/// handler.
///
/// It should match `save_context!`.
#[cfg(all(
    any(arm_architecture = "v7-r", arm_architecture = "v8-r"),
    not(any(target_abi = "eabihf", feature = "eabi-fpu"))
))]
macro_rules! restore_context {
    () => {
        r#"
        // restore alignment amount, and preserved register
        pop     {{r0, r12}}
        // restore pre-alignment SP
        add     sp, r0
        // restore more preserved registers
        pop     {{r0-r3}}
        "#
    };
}

/// This macro expands to code for saving context on entry to an exception
/// handler.
///
/// It should match `restore_context!`.
#[cfg(all(
    any(arm_architecture = "v7-r", arm_architecture = "v8-r"),
    any(target_abi = "eabihf", feature = "eabi-fpu")
))]
macro_rules! save_context {
    () => {
        r#"
        // save preserved registers (and gives us some working area)
        push    {{r0-r3}}
        // save FPU context
        vpush   {{d0-d7}}
        vmrs    r0, FPSCR
        vmrs    r1, FPEXC
        push    {{r0-r1}}
        // align SP down to eight byte boundary
        mov     r0, sp
        and     r0, r0, 7
        sub     sp, r0
        // push alignment amount, and final preserved register
        push    {{r0, r12}}
        "#
    };
}

/// This macro expands to code for restoring context on exit from an exception
/// handler.
///
/// It should match `save_context!`.
#[cfg(all(
    any(arm_architecture = "v7-r", arm_architecture = "v8-r"),
    any(target_abi = "eabihf", feature = "eabi-fpu")
))]
macro_rules! restore_context {
    () => {
        r#"
        // restore alignment amount, and preserved register
        pop     {{r0, r12}}
        // restore pre-alignment SP
        add     sp, r0
        // pop FPU state
        pop     {{r0-r1}}
        vmsr    FPEXC, r1
        vmsr    FPSCR, r0
        vpop    {{d0-d7}}
        // restore more preserved registers
        pop     {{r0-r3}}
        "#
    };
}

// Our assembly language exception handlers when we don't have an FPU
#[cfg(any(arm_architecture = "v7-r", arm_architecture = "v8-r"))]
core::arch::global_asm!(
    r#"
    .section .text.handlers
    .code 32
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp3-d16
    .align 0

    // Called from the vector table when we have an software interrupt.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn svc_handler(svc: u32, context: *const u32);`
    .global _asm_svc_handler
    _asm_svc_handler:
        srsfd   sp!, #0x13
    "#,
    save_context!(),
    r#"
        tst      r0, #0x20                // Occurred in Thumb state?
        ldrhne   r0, [lr,#-2]             // Yes: Load halfword and...
        bicne    r0, r0, #0xFF00          // ...extract comment field
        ldreq    r0, [lr,#-4]             // No: Load word and...
        biceq    r0, r0, #0xFF000000      // ...extract comment field
        // r0 now contains SVC number
        bl       _svc_handler
    "#,
    restore_context!(),
    r#"
        rfefd   sp!

    // Called from the vector table when we have an interrupt.
    // Saves state and calls a C-compatible handler like
    // `extern "C" fn irq_handler();`
    .global _asm_irq_handler
    _asm_irq_handler:
        srsfd   sp!, #0x12
    "#,
    save_context!(),
    r#"
        // call C handler
        bl      _irq_handler
    "#,
    restore_context!(),
    r#"
        rfefd   sp!
    "#
);

#[cfg(all(
    any(arm_architecture = "v7-r", arm_architecture = "v8-r"),
    any(target_abi = "eabihf", feature = "eabi-fpu")
))]
macro_rules! fpu_enable {
    () => {
        r#"
        // Allow VFP coprocessor access
        mrc     p15, 0, r0, c1, c0, 2
        orr     r0, r0, #0xF00000
        mcr     p15, 0, r0, c1, c0, 2
        // Enable VFP
        mov     r0, #0x40000000
        vmsr    fpexc, r0
        "#
    };
}

#[cfg(all(
    any(arm_architecture = "v7-r", arm_architecture = "v8-r"),
    not(any(target_abi = "eabihf", feature = "eabi-fpu"))
))]
macro_rules! fpu_enable {
    () => {
        r#"
        // no FPU - do nothing
        "#
    };
}

// Start-up code for Armv7-R (and Armv8-R once we've left EL2)
//
// We set up our stacks and `kmain` in system mode.
#[cfg(any(arm_architecture = "v7-r", arm_architecture = "v8-r"))]
core::arch::global_asm!(
    r#"
    .section .text.startup
    .code 32
    .align 0
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp3-d16

    _el1_start:
        // Set stack pointer (as the top) and mask interrupts for for FIQ mode (Mode 0x11)
        ldr     r0, =_stack_top
        msr     cpsr, #0xD1
        mov     sp, r0
        ldr     r1, =_fiq_stack_size
        sub     r0, r0, r1
        // Set stack pointer (right after) and mask interrupts for for IRQ mode (Mode 0x12)
        msr     cpsr, #0xD2
        mov     sp, r0
        ldr     r1, =_irq_stack_size
        sub     r0, r0, r1
        // Set stack pointer (right after) and mask interrupts for for SVC mode (Mode 0x13)
        msr     cpsr, #0xD3
        mov     sp, r0
        ldr     r1, =_svc_stack_size
        sub     r0, r0, r1
        // Set stack pointer (right after) and mask interrupts for for System mode (Mode 0x1F)
        msr     cpsr, #0xDF
        mov     sp, r0
    "#,
    fpu_enable!(),
    r#"
        // Initialise .bss
        ldr     r0, =__sbss
        ldr     r1, =__ebss
        mov     r2, 0
    0:
        cmp     r1, r0
        beq     1f
        stm     r0!, {{r2}}
        b       0b
    1:
        // Initialise .data
        ldr     r0, =__sdata
        ldr     r1, =__edata
        ldr     r2, =__sidata
    0:
        cmp     r1, r0
        beq     1f
        ldm     r2!, {{r3}}
        stm     r0!, {{r3}}
        b       0b
    1:
        // Jump to application
        bl      kmain
        // In case the application returns, loop forever
        b       .
    "#
);

// Start-up code for Armv7-R.
//
// Go straight to our default routine
#[cfg(arm_architecture = "v7-r")]
core::arch::global_asm!(
    r#"
    .section .text.startup
    .code 32
    .align 0

    .global _default_start
    _default_start:
        ldr     pc, =_el1_start
    "#
);

// Start-up code for Armv8-R.
//
// There's only one Armv8-R CPU (the Cortex-R52) and the FPU is mandatory, so we
// always enable it.
//
// We boot into EL2, set up a stack pointer, and run `kmain` in EL1.
#[cfg(arm_architecture = "v8-r")]
core::arch::global_asm!(
    r#"
    .section .text.startup
    .code 32
    .align 0
    
    .global _default_start
    _default_start:
        // Are we in EL2? If not, skip the EL2 setup portion
        mrs     r0, cpsr
        and     r0, r0, #0x1f
        cmp     r0, #0x1a
        bne     1f
        // Set stack pointer
        ldr     sp, =_stack_top
        // Set the HVBAR (for EL2) to _vector_table
        ldr     r0, =_vector_table
        mcr     p15, 4, r0, c12, c0, 0
        // Configure HACTLR to let us enter EL1
        mrc     p15, 4, r0, c1, c0, 1
        orr     r0, r0, #0x7700
        orr     r0, r0, #0x0083
        mcr     p15, 4, r0, c1, c0, 1
        // Program the SPSR - enter sytem mode (0x1F) in Arm mode with IRQ, FIQ masked
        mov		r0, #0xDF
        msr		spsr_hyp, r0
        adr		r0, 1f
        msr		elr_hyp, r0
        dsb
        isb
        eret
    1:
        // Set the VBAR (for EL1) to _vector_table. NB: This isn't required on
        // Armv7-R because that only supports 'low' (default) or 'high'.
        ldr     r0, =_vector_table
        mcr     p15, 0, r0, c12, c0, 0
        // go do the rest of the EL1 init
        ldr     pc, =_el1_start
    "#
);
