//! Access registers in Armv7-R and Armv8-R

mod cpsr;
#[doc(inline)]
pub use cpsr::Cpsr;

mod midr;
#[doc(inline)]
pub use midr::Midr;

mod sctlr;
#[doc(inline)]
pub use sctlr::Sctlr;

#[cfg(arm_architecture = "v8-r")]
mod hactlr;
#[doc(inline)]
#[cfg(arm_architecture = "v8-r")]
pub use hactlr::Hactlr;

#[cfg(arm_architecture = "v8-r")]
mod hvbar;
#[doc(inline)]
#[cfg(arm_architecture = "v8-r")]
pub use hvbar::Hvbar;

#[cfg(arm_architecture = "v8-r")]
mod vbar;
#[doc(inline)]
#[cfg(arm_architecture = "v8-r")]
pub use vbar::Vbar;

// TODO:

// Multiprocessor Affinity Register (MPIDR)

// System Control Register

// Auxilliary Control Register

// Coprocessor Access Control Register

// Data Fault Status Register

// Instruction Fault Status Register

// Data Fault Address Register

// Instruction Fault Address Register

// MPU Region Base Address Register

// MPU Region Size and Enable Register

// MPU Region Access Control Register

// MPU Region Number Register

// Context ID Register

// Software Thread ID Register

// Configuration Base Address Register
