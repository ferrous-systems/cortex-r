//! Run-time support for Arm Cortex-R

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

// The Interrupt Vector Table
#[cfg(any(arm_architecture = "v7-r", arm_architecture = "v8-r"))]
core::arch::global_asm!(
    r#"
.section .vector_table
.global _vector_table
.code 32
.align 0
_vector_table:
    ldr pc, =_start
    ldr pc, =_asm_undefined_handler
    ldr pc, =_asm_svc_handler
    ldr pc, =_asm_prefetch_handler
    ldr pc, =_asm_abort_handler
    nop
    ldr pc, =_asm_irq_handler
"#
);

// Our assembly language exception handlers when we don't have an FPU
#[cfg(all(
    any(arm_architecture = "v7-r", arm_architecture = "v8-r"),
    not(any(target_abi = "eabihf", feature = "eabi-fpu"))
))]
core::arch::global_asm!(
    r#"
.section .text.handlers
.code 32
.arch armv7-r
.align 0

// Called from the vector table when we have an undefined exception.
// Saves state and calls a C-compatible handler
.global _asm_undefined_handler
_asm_undefined_handler:
    // save state
    push    {{r0-r3, r12, lr}}
    // call C handler
    b       _undefined_handler
    // restore state (the ^ means copy SPSR back to CPSR)
    ldmia    sp!, {{r0-r3, r12, pc}}^

// Called from the vector table when we have an software interrupt.
// Saves state and calls a C-compatible handler
.global _asm_svc_handler
_asm_svc_handler:
    push     {{r0-r3, r12, lr}}
    mov      r1, sp                   // Set pointer to parameters
    tst      r0, #0x20                // Occurred in Thumb state?
    ldrhne   r0, [lr,#-2]             // Yes: Load halfword and...
    bicne    r0, r0, #0xFF00          // ...extract comment field
    ldreq    r0, [lr,#-4]             // No: Load word and...
    biceq    r0, r0, #0xFF000000      // ...extract comment field
    // r0 now contains SVC number
    // r1 now contains pointer to stacked register
    bl       _svc_handler
    ldmia    sp!, {{r0-r3, r12, pc}}^

// Called from the vector table when we have an prefetch exception.
// Saves state and calls a C-compatible handler
.global _asm_prefetch_handler
_asm_prefetch_handler:
    // save state
    push    {{r0-r3, r12, lr}}
    // call C handler
    b       _prefetch_handler
    // restore state (the ^ means copy SPSR back to CPSR)
    ldmia    sp!, {{r0-r3, r12, pc}}^

// Called from the vector table when we have an abort exception.
// Saves state and calls a C-compatible handler
.global _asm_abort_handler
_asm_abort_handler:
    // save state
    push    {{r0-r3, r12, lr}}
    // call C handler
    b       _abort_handler
    // restore state (the ^ means copy SPSR back to CPSR)
    ldmia    sp!, {{r0-r3, r12, pc}}^

// Called from the vector table when we have an interrupt.
// Saves state and calls a C-compatible handler
.global _asm_irq_handler
_asm_irq_handler:
    // save state
    push    {{r0-r3, r12, lr}}
    // call C handler
    b       _irq_handler
    // restore state (the ^ means copy SPSR back to CPSR)
    ldmia    sp!, {{r0-r3, r12, pc}}^
"#
);

// Our assembly language exception handlers when we do have an FPU
#[cfg(all(
    any(arm_architecture = "v7-r", arm_architecture = "v8-r"),
    any(target_abi = "eabihf", feature = "eabi-fpu")
))]
core::arch::global_asm!(
    r#"
.section .text.handlers
.code 32
.align 0
// Work around https://github.com/rust-lang/rust/issues/127269
.fpu vfp3-d16

// Called from the vector table when we have an undefined exception.
// Saves state and calls a C-compatible handler
.global _asm_undefined_handler
_asm_undefined_handler:
    // save state
    push    {{r0-r3, r12, lr}}
    vpush   {{d0-d7}}
	vmrs    r0, FPSCR
	vmrs    r1, FPEXC
    push    {{r0-r1}}
    // call C handler
    b       _undefined_handler
    // restore state (the ^ means copy SPSR back to CPSR)
    pop     {{r0-r1}}
    vmsr    FPEXC, r1
    vmsr    FPSCR, r0
    vpop    {{d0-d7}}
    ldmia    sp!, {{r0-r3, r12, pc}}^

// Called from the vector table when we have an software interrupt.
// Saves state and calls a C-compatible handler
.global _asm_svc_handler
_asm_svc_handler:
    // save state
    push    {{r0-r3, r12, lr}}
    vpush   {{d0-d7}}
	vmrs    r0, FPSCR
	vmrs    r1, FPEXC
    push    {{r0-r1}}

    mov     r1, sp                   // Set pointer to parameters
    tst     r0, #0x20                // Occurred in Thumb state?
    ldrhne  r0, [lr,#-2]             // Yes: Load halfword and...
    bicne   r0, r0, #0xFF00          // ...extract comment field
    ldreq   r0, [lr,#-4]             // No: Load word and...
    biceq   r0, r0, #0xFF000000      // ...extract comment field
    // r0 now contains SVC number
    // r1 now contains pointer to stacked register
    bl      _svc_handler

    // restore state (the ^ means copy SPSR back to CPSR)
    pop     {{r0-r1}}
    vmsr    FPEXC, r1
    vmsr    FPSCR, r0
    vpop    {{d0-d7}}
    ldmia    sp!, {{r0-r3, r12, pc}}^

// Called from the vector table when we have an prefetch exception.
// Saves state and calls a C-compatible handler
.global _asm_prefetch_handler
_asm_prefetch_handler:
    // save state
    push    {{r0-r3, r12, lr}}
    vpush   {{d0-d7}}
	vmrs    r0, FPSCR
	vmrs    r1, FPEXC
    push    {{r0-r1}}
    // call C handler
    b       _prefetch_handler
    // restore state (the ^ means copy SPSR back to CPSR)
    pop     {{r0-r1}}
    vmsr    FPEXC, r1
    vmsr    FPSCR, r0
    vpop    {{d0-d7}}
    ldmia    sp!, {{r0-r3, r12, pc}}^

// Called from the vector table when we have an abort exception.
// Saves state and calls a C-compatible handler
.global _asm_abort_handler
_asm_abort_handler:
    // save state
    push    {{r0-r3, r12, lr}}
    vpush   {{d0-d7}}
	vmrs    r0, FPSCR
	vmrs    r1, FPEXC
    push    {{r0-r1}}
    // call C handler
    b       _abort_handler
    // restore state (the ^ means copy SPSR back to CPSR)
    pop     {{r0-r1}}
    vmsr    FPEXC, r1
    vmsr    FPSCR, r0
    vpop    {{d0-d7}}
    ldmia    sp!, {{r0-r3, r12, pc}}^

// Called from the vector table when we have an interrupt.
// Saves state and calls a C-compatible handler
.global _asm_irq_handler
_asm_irq_handler:
    // save state
    push    {{r0-r3, r12, lr}}
    vpush   {{d0-d7}}
	vmrs    r0, FPSCR
	vmrs    r1, FPEXC
    push    {{r0-r1}}
    // call C handler
    b       _irq_handler
    // restore state (the ^ means copy SPSR back to CPSR)
    pop     {{r0-r1}}
    vmsr    FPEXC, r1
    vmsr    FPSCR, r0
    vpop    {{d0-d7}}
    ldmia    sp!, {{r0-r3, r12, pc}}^
"#
);

// Start-up code for Armv7-R with an FPU
//
// We boot into Supervisor mode, set up a stack pointer, and run `kmain` in supervisor mode.
#[cfg(all(
    arm_architecture = "v7-r",
    any(target_abi = "eabihf", feature = "eabi-fpu")
))]
core::arch::global_asm!(
    r#"
.section .text.startup
.global _start
.code 32
.align 0
.arch armv7-r
// Work around https://github.com/rust-lang/rust/issues/127269
.fpu vfp3-d16

_start:
    // Set stack pointer
    ldr     sp, =_stack_top
    // Allow VFP coprocessor access
    mrc     p15, 0, r0, c1, c0, 2
    orr     r0, r0, #0xF00000
    mcr     p15, 0, r0, c1, c0, 2
    // Enable VFP
    mov     r0, #0x40000000
    vmsr    fpexc, r0
    // Jump to application
    bl      kmain
    // In case the application returns, loop forever
    b       .

"#
);

// Start-up code for Armv7-R without an FPU
//
// We boot into Supervisor mode, set up a stack pointer, and run `kmain` in supervisor mode.
#[cfg(all(
    arm_architecture = "v7-r",
    not(any(target_abi = "eabihf", feature = "eabi-fpu"))
))]
core::arch::global_asm!(
    r#"
.section .text.startup
.global _start
.code 32
.arch armv7-r
.align 0

_start:
    // Set stack pointer
    ldr     sp, =_stack_top
    // Jump to application
    bl      kmain
    // In case the application returns, loop forever
    b       .
"#
);

// Start-up code for Armv8-R
//
// We boot into EL2, set up a stack pointer, and run `kmain` in EL1.
#[cfg(arm_architecture = "v8-r")]
core::arch::global_asm!(
    r#"
.section .text.startup
.global _start
.code 32
.align 0
// Work around https://github.com/rust-lang/rust/issues/127269
.fpu vfp3-d16

_start:
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
    // Program the SPSR - enter supervisor mode in Arm mode with IRQ, FIQ and Async Abort masked
    mov		r0, #0x1DF
	msr		spsr_hyp, r0
	adr		r0, 1f
	msr		elr_hyp, r0
	dsb
	isb
	eret
1:
    // Set stack pointer (we can trash the EL2 stack - we're not coming back)
    ldr     sp, =_stack_top
    // Set the VBAR (for EL1) to _vector_table
    ldr     r0, =_vector_table
    mcr     p15, 0, r0, c12, c0, 0
    // Allow VFP coprocessor access
    mrc     p15, 0, r0, c1, c0, 2
    orr     r0, r0, #0xF00000
    mcr     p15, 0, r0, c1, c0, 2
    // Enable VFP
    mov     r0, #0x40000000
    vmsr    fpexc, r0
    // Jump to application
    bl      kmain
    // In case the application returns, loop forever
    b       .
"#
);
