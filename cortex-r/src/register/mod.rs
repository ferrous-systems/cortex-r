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
