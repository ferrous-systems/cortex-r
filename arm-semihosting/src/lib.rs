//! Semihosting for ARM Cortex-R processors
//!
//! This is a port of [`cortex-m-semihosting`] to Arm Cortex-R.
//!
//! [`cortex-m-semihosting`]: https://crates.io/crates/cortex-m-semihosting
//!
//! # What is semihosting?
//!
//! "Semihosting is a mechanism that enables code running on an ARM target to
//! communicate and use the Input/Output facilities on a host computer that is
//! running a debugger." - ARM
//!
//! # Interface
//!
//! This crate provides implementations of
//! [`core::fmt::Write`](https://doc.rust-lang.org/core/fmt/trait.Write.html),
//! so you can use it, in conjunction with
//! [`core::format_args!`](https://doc.rust-lang.org/core/macro.format_args.html)
//! or the [`write!` macro](https://doc.rust-lang.org/core/macro.write.html),
//! for user-friendly construction and printing of formatted strings.
//!
//! Since semihosting operations are modeled as [system calls][sc], this crate
//! exposes an untyped `syscall!` interface just like the [`sc`] crate does.
//!
//! [sc]: https://en.wikipedia.org/wiki/System_call
//! [`sc`]: https://crates.io/crates/sc
//!
//! # Forewarning
//!
//! Semihosting operations are *very* slow. Like, each WRITE operation can take
//! hundreds of milliseconds.
//!
//! # References
//!
//! * Arm "ARM DS-5 Debugger User Guide Version 5.26"
//!   * <https://developer.arm.com/documentation/dui0446/z/controlling-target-execution/using-semihosting-to-access-resources-on-the-host-computer>
//!
//! # Example
//!
//! ## Using `hio::hstdout`
//!
//! This example will demonstrate how to print formatted strings.
//!
//! ```no_run
//! use arm_semihosting::hio;
//! use core::fmt::Write;
//!
//! // This function will be called by the application
//! fn print() -> Result<(), core::fmt::Error> {
//!     let mut stdout = hio::hstdout().map_err(|_| core::fmt::Error)?;
//!     let language = "Rust";
//!     let ranking = 1;
//!
//!     write!(stdout, "{} on embedded is #{}!", language, ranking)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! On the host side, use `-semihosting` with `qemu-system-arm`, or your
//! favourite semi-hosting capable debugger.
//!
//! ## The `dbg!` macro
//!
//! Analogous to [`std::dbg`](https://doc.rust-lang.org/std/macro.dbg.html) the
//! macro `dbg!` returns a given expression and prints it using `heprintln!`
//! including context for quick and dirty debugging.
//!
//! Panics if `heprintln!` returns an error.
//!
//! Example:
//!
//! ```no_run
//! const UUID: *mut u32 = 0x0009_FC70 as *mut u32;
//! dbg!(UUID);
//! let mut uuid: [u32; 4] = [0; 4];
//! for i in 0..4 {
//!     dbg!(i);
//!     uuid[i] = unsafe { dbg!(UUID.offset(i as isize).read_volatile()) };
//! }
//! ```
//! outputs
//! ```text
//! [examples/semihosting.rs:37] UUID = 0x0009fc70
//! [examples/semihosting.rs:40] i = 0
//! [examples/semihosting.rs:41] UUID.offset(i as isize).read_volatile() = 3370045464
//! [examples/semihosting.rs:40] i = 1
//! [examples/semihosting.rs:41] UUID.offset(i as isize).read_volatile() = 1426218275
//! [examples/semihosting.rs:40] i = 2
//! [examples/semihosting.rs:41] UUID.offset(i as isize).read_volatile() = 2422621116
//! [examples/semihosting.rs:40] i = 3
//! [examples/semihosting.rs:41] UUID.offset(i as isize).read_volatile() = 1044138593
//! ```
//!
//! # Optional features
//!
//! ## `jlink-quirks`
//!
//! When this feature is enabled, return values above `0xfffffff0` from
//! semihosting operation `SYS_WRITE` (0x05) are interpreted as if the entire
//! buffer had been written. The current latest version 6.48b of J-Link exhibits
//! such behaviour, causing a panic if this feature is not enabled.
//!
//! ## `no-semihosting`
//!
//! When this feature is enabled, the underlying system calls to `SVC` are
//! patched out.
//!
//! # Reference
//!
//! For documentation about the semihosting operations, check:
//!
//! 'Chapter 8 - Semihosting' of the ['ARM Compiler toolchain Version 5.0'][pdf]
//! manual.
//!
//! [pdf]:
//!     http://infocenter.arm.com/help/topic/com.arm.doc.dui0471e/DUI0471E_developing_for_arm_processors.pdf

#![deny(missing_docs)]
#![no_std]

#[macro_use]
mod macros;

pub mod debug;
#[doc(hidden)]
pub mod export;
pub mod hio;
pub mod nr;

/// Performs a semihosting operation, takes a pointer to an argument block
///
/// # Safety
///
/// The syscall number must be a valid [semihosting operation],
/// and the arguments must be valid for the associated operation.
///
/// [semihosting operation]: https://developer.arm.com/documentation/dui0471/i/semihosting/semihosting-operations?lang=en
#[inline(always)]
pub unsafe fn syscall<T>(nr: usize, arg: &T) -> usize {
    syscall1(nr, arg as *const T as usize)
}

/// Performs a semihosting operation, takes one integer as an argument
///
/// # Safety
///
/// Same as [`syscall()`].
#[inline(always)]
pub unsafe fn syscall1(_nr: usize, _arg: usize) -> usize {
    if cfg!(all(target_arch = "arm", not(feature = "no-semihosting"))) {
        let mut nr = _nr as u32;
        let arg = _arg as u32;
        #[cfg(arm_isa = "a64")]
        unsafe {
            core::arch::asm!("HLT 0xF000", inout("r0") nr, in("r1") arg, options(nostack, preserves_flags));
        }

        #[cfg(arm_isa = "a32")]
        unsafe {
            // We have observed some systems that accepted the HLT instruction
            // but not SVC instruction, even though both should be equivalent.
            // So prefer SVC but let the user pick HLT.
            if cfg!(feature = "use-hlt") {
                core::arch::asm!("HLT 0xF000", inout("r0") nr, in("r1") arg, options(nostack, preserves_flags));
            } else {
                core::arch::asm!("SVC 0x123456", inout("r0") nr, in("r1") arg, options(nostack, preserves_flags));
            }
        }

        #[cfg(all(arm_isa = "t32", not(arm_profile = "m")))]
        unsafe {
            // See note about about SVC vs HLT
            if cfg!(feature = "use-hlt") {
                core::arch::asm!("HLT 0x3C", inout("r0") nr, in("r1") arg, options(nostack, preserves_flags));
            } else {
                core::arch::asm!("SVC 0xAB", inout("r0") nr, in("r1") arg, options(nostack, preserves_flags));
            }
        }

        #[cfg(all(arm_isa = "t32", arm_profile = "m"))]
        unsafe {
            core::arch::asm!("BKPT 0xAB", inout("r0") nr, in("r1") arg, options(nostack, preserves_flags));
        }

        return nr as usize;
    }

    0
}
