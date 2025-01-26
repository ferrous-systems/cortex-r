// Copyright 2023 The arm-gic Authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

#[cfg(test)]
#[macro_use]
pub mod fake;

#[cfg(target_arch = "arm")]
use core::arch::asm;

pub fn read_icc_iar1_el1() -> u64 {
    let r: usize;
    #[cfg(target_arch = "arm")]
    // Safety: This loads from a co-processor register to a main register
    unsafe {
        asm!("mrc p15, 0, {}, c12, c12, 0", out(reg) r, options(nomem, nostack, preserves_flags));
    }
    #[cfg(not(target_arch = "arm"))]
    {
        r = 0;
    }
    r as u64
}

pub fn write_icc_ctlr_el1(_value: u64) {
    #[cfg(target_arch = "arm")]
    // Safety: This copies from a main register to a co-processor register
    unsafe {
        asm!("mcr p15, 0, {}, c12, c12, 4", in(reg) _value as u32, options(nomem, nostack, preserves_flags));
    }
}

pub fn write_icc_eoir1_el1(_value: u64) {
    #[cfg(target_arch = "arm")]
    // Safety: This copies from a main register to a co-processor register
    unsafe {
        asm!("mcr p15, 0, {}, c12, c12, 1", in(reg) _value as u32, options(nomem, nostack, preserves_flags));
    }
}

pub fn write_icc_igrpen1_el1(_value: u64) {
    #[cfg(target_arch = "arm")]
    // Safety: This copies from a main register to a co-processor register
    unsafe {
        asm!("mcr p15, 0, {}, c12, c12, 7", in(reg) _value as u32, options(nomem, nostack, preserves_flags));
    }
}

pub fn write_icc_pmr_el1(_value: u64) {
    #[cfg(target_arch = "arm")]
    // Safety: This copies from a main register to a co-processor register
    unsafe {
        asm!("mcr p15, 0, {}, c4, c6, 0", in(reg) _value as u32, options(nomem, nostack, preserves_flags));
    }
}

pub fn write_icc_sgi1r_el1(_value: u64) {
    #[cfg(target_arch = "arm")]
    // Safety: This copies from a main register to a co-processor register
    unsafe {
        let value_hi = (_value >> 32) as u32;
        let value_lo = _value as u32;
        asm!("mcrr p15, 0, {rt}, {rt2}, c12", rt = in(reg) value_lo, rt2 = in(reg) value_hi, options(nomem, nostack, preserves_flags));
    }
}

pub fn write_icc_sre_el1(_value: u64) {
    #[cfg(target_arch = "arm")]
    // Safety: This copies from a main register to a co-processor register
    unsafe {
        asm!("mcr p15, 0, {}, c12, c12, 5", in(reg) _value as u32, options(nomem, nostack, preserves_flags));
    }
}
