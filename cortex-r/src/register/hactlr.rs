//! Code for managing the *Hyp Auxiliary Control Register*

/// The *Hyp Auxiliary Control Register* (HACTRL)
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Hactlr(u32);

impl Hactlr {
    /// The bitmask for the CPUACTLR bit
    pub const CPUACTLR_BIT: u32 = 1 << 0;
    /// The bitmask for the CDBGDCI bit
    pub const CDBGDCI_BIT: u32 = 1 << 1;
    /// The bitmask for the FLASHIFREGIONR bit
    pub const FLASHIFREGIONR_BIT: u32 = 1 << 7;
    /// The bitmask for the PERIPHPREGIONR bit
    pub const PERIPHPREGIONR_BIT: u32 = 1 << 8;
    /// The bitmask for the QOSR bit
    pub const QOSR_BIT: u32 = 1 << 9;
    /// The bitmask for the BUSTIMEOUTR bit
    pub const BUSTIMEOUTR_BIT: u32 = 1 << 10;
    /// The bitmask for the INTMONR bit
    pub const INTMONR_BIT: u32 = 1 << 12;
    /// The bitmask for the ERR bit
    pub const ERR_BIT: u32 = 1 << 13;
    /// The bitmask for the TESTR1 bit
    pub const TESTR1_BIT: u32 = 1 << 15;

    /// Reads the *Hyp Auxiliary Control Register*
    #[inline]
    pub fn read() -> Hactlr {
        let r: u32;
        // Safety: Reading this register has no side-effects and is atomic
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("mrc p15, 4, {}, c1, c0, 1", out(reg) r, options(nomem, nostack, preserves_flags));
        }
        #[cfg(not(target_arch = "arm"))]
        {
            r = 0;
        }
        Self(r)
    }

    /// Write to the *Hyp Auxiliary Control Register*
    #[inline]
    pub fn write(_value: Self) {
        // Safety: Writing this register is atomic
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("mcr p15, 4, {}, c1, c0, 1", in(reg) _value.0, options(nomem, nostack, preserves_flags));
        };
    }

    /// Modify the *Hyp Auxiliary Control Register*
    #[inline]
    pub fn modify<F>(f: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut value = Self::read();
        f(&mut value);
        Self::write(value);
    }

    /// Is the CPUACTLR bit set?
    pub fn cpuactlr(self) -> bool {
        (self.0 & Self::CPUACTLR_BIT) != 0
    }

    /// Set the CPUACTLR bit
    pub fn set_cpuactlr(&mut self) {
        self.0 |= Self::CPUACTLR_BIT;
    }

    /// Clear the CPUACTLR bit
    pub fn clear_cpuactlr(&mut self) {
        self.0 &= !Self::CPUACTLR_BIT;
    }

    /// Is the CDBGDCI bit set?
    pub fn cdbgdci(self) -> bool {
        (self.0 & Self::CDBGDCI_BIT) != 0
    }

    /// Set the CDBGDCI bit
    pub fn set_cdbgdci(&mut self) {
        self.0 |= Self::CDBGDCI_BIT;
    }

    /// Clear the CDBGDCI bit
    pub fn clear_cdbgdci(&mut self) {
        self.0 &= !Self::CDBGDCI_BIT;
    }

    /// Is the FLASHIFREGIONR bit set?
    pub fn flashifregionr(self) -> bool {
        (self.0 & Self::FLASHIFREGIONR_BIT) != 0
    }

    /// Set the FLASHIFREGIONR bit
    pub fn set_flashifregionr(&mut self) {
        self.0 |= Self::FLASHIFREGIONR_BIT;
    }

    /// Clear the FLASHIFREGIONR bit
    pub fn clear_flashifregionr(&mut self) {
        self.0 &= !Self::FLASHIFREGIONR_BIT;
    }

    /// Is the PERIPHPREGIONR bit set?
    pub fn periphpregionr(self) -> bool {
        (self.0 & Self::PERIPHPREGIONR_BIT) != 0
    }

    /// Set the PERIPHPREGIONR bit
    pub fn set_periphpregionr(&mut self) {
        self.0 |= Self::PERIPHPREGIONR_BIT;
    }

    /// Clear the PERIPHPREGIONR bit
    pub fn clear_periphpregionr(&mut self) {
        self.0 &= !Self::PERIPHPREGIONR_BIT;
    }

    /// Is the QOSR bit set?
    pub fn qosr(self) -> bool {
        (self.0 & Self::QOSR_BIT) != 0
    }

    /// Set the QOSR bit
    pub fn set_qosr(&mut self) {
        self.0 |= Self::QOSR_BIT;
    }

    /// Clear the QOSR bit
    pub fn clear_qosr(&mut self) {
        self.0 &= !Self::QOSR_BIT;
    }

    /// Is the BUSTIMEOUTR bit set?
    pub fn bustimeoutr(self) -> bool {
        (self.0 & Self::BUSTIMEOUTR_BIT) != 0
    }

    /// Set the BUSTIMEOUTR bit
    pub fn set_bustimeoutr(&mut self) {
        self.0 |= Self::BUSTIMEOUTR_BIT;
    }

    /// Clear the BUSTIMEOUTR bit
    pub fn clear_bustimeoutr(&mut self) {
        self.0 &= !Self::BUSTIMEOUTR_BIT;
    }

    /// Is the INTMONR bit set?
    pub fn intmonr(self) -> bool {
        (self.0 & Self::INTMONR_BIT) != 0
    }

    /// Set the INTMONR bit
    pub fn set_intmonr(&mut self) {
        self.0 |= Self::INTMONR_BIT;
    }

    /// Clear the INTMONR bit
    pub fn clear_intmonr(&mut self) {
        self.0 &= !Self::INTMONR_BIT;
    }

    /// Is the ERR bit set?
    pub fn err(self) -> bool {
        (self.0 & Self::ERR_BIT) != 0
    }

    /// Set the ERR bit
    pub fn set_err(&mut self) {
        self.0 |= Self::ERR_BIT;
    }

    /// Clear the ERR bit
    pub fn clear_err(&mut self) {
        self.0 &= !Self::ERR_BIT;
    }

    /// Is the TESTR1 bit set?
    pub fn testr1(self) -> bool {
        (self.0 & Self::TESTR1_BIT) != 0
    }

    /// Set the TESTR1 bit
    pub fn set_testr1(&mut self) {
        self.0 |= Self::TESTR1_BIT;
    }

    /// Clear the TESTR1 bit
    pub fn clear_testr1(&mut self) {
        self.0 &= !Self::TESTR1_BIT;
    }
}

impl core::fmt::Debug for Hactlr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "HACTLR {{ CPUACTLR={}, CDBGDCI={}, FLASHIFREGIONR={}, PERIPHPREGIONR={}, QOSR={}, BUSTIMEOUTR={}, INTMONR={}, ERR={}, TESTR1={} }}",
            self.cpuactlr() as u8,
            self.cdbgdci() as u8,
            self.flashifregionr() as u8,
            self.periphpregionr() as u8,
            self.qosr() as u8,
            self.bustimeoutr() as u8,
            self.intmonr() as u8,
            self.err() as u8,
            self.testr1() as u8
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Hactlr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HACTLR {{ CPUACTLR={0=0..1}, CDBGDCI={0=1..2}, FLASHIFREGIONR={0=7..8}, PERIPHPREGIONR={0=8..9}, QOSR={0=9..10}, BUSTIMEOUTR={0=10..11}, INTMONR={0=12..13}, ERR={0=13..14}, TESTR1={0=15..16} }}", self.0)
    }
}
