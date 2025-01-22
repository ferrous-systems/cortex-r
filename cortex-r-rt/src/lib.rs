//! Run-time support for Arm Cortex-R

#![no_std]

#[cfg(
    all(
        any(
            cmr_architecture = "v7-r",
            cmr_architecture = "v8-r"
        ),
        any(
            cmr_abi="hard-float",
            feature="eabi-fpu"
        )
    )
)]
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
    ldr sp, =stack_top
    // Allow VFP coprocessor access
    mrc p15, 0, r0, c1, c0, 2
    orr r0, r0, #0xF00000
    mcr p15, 0, r0, c1, c0, 2
    // Enable VFP
    mov r0, #0x40000000
    vmsr fpexc, r0
    // Jump to application
    bl kmain
    // In case the application returns, loop forever
    b .

"#
);

#[cfg(
    all(
        any(
            cmr_architecture = "v7-r",
            cmr_architecture = "v8-r"
        ),
        not(
            any(
                cmr_abi="hard-float",
                feature="eabi-fpu"
            )
        )
    )
)]
core::arch::global_asm!(
    r#"

.section .text.startup
.global _start
.code 32
.align 0

_start:
    // Set stack pointer
    ldr sp, =stack_top
    // Jump to application
    bl kmain
    // In case the application returns, loop forever
    b .

"#
);
