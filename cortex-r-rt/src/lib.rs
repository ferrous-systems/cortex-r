//! Run-time support for Arm Cortex-R

#![no_std]

// The Interrupt Vector Table
#[cfg(any(arm_architecture = "v7-r", arm_architecture = "v8-r"))]
core::arch::global_asm!(
    r#"
.section .vector_table
.global _vector_table
.code 32
.align 0
_vector_table:
    ldr pc,=_start
    ldr pc,=_asm_undefined_handler
    ldr pc,=_asm_svc_handler
    ldr pc,=_asm_prefetch_handler
    ldr pc,=_asm_abort_handler
    nop
    ldr pc,=_asm_irq_handler
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

// Our default (c-compatible) handler crashes the CPU
.global _default_handler
_default_handler:
    udf 0
    b       _default_handler
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

// Our default (c-compatible) handler crashes the CPU
.global _default_handler
_default_handler:
    udf 0
    b       _default_handler
"#
);

#[cfg(all(
    any(arm_architecture = "v7-r", arm_architecture = "v8-r"),
    any(target_abi = "eabihf", feature = "eabi-fpu")
))]
core::arch::global_asm!(
    r#"
.section .text.startup
.global _start
.code 32
.align 0
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

#[cfg(all(
    any(arm_architecture = "v7-r", arm_architecture = "v8-r"),
    not(any(target_abi = "eabihf", feature = "eabi-fpu"))
))]
core::arch::global_asm!(
    r#"
.section .text.startup
.global _start
.code 32
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
