//! CPU/peripheral support for Arm Cortex-R

#![no_std]

#[cfg(feature = "critical-section-single-core")]
mod critical_section;

pub mod register;

pub mod interrupt;
