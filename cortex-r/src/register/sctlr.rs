//! Code for managing the *System Control Register*

/// The *System Control Register* (SCTLR)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sctlr(u32);

impl Sctlr {
    pub const N_BIT: u32 = 1 << 31;

    /// The bitmask for the Instruction Endianness bit
    const IE_BIT: u32 = 1 << 31;
    /// The bitmask for the Thumb Exception Enable bit
    const TE_BIT: u32 = 1 << 30;
    /// The bitmask for the Non-Maskable FIQ bit
    const NMFI_BIT: u32 = 1 << 27;
    /// The bitmask for the Exception Endianness bit
    const EE_BIT: u32 = 1 << 25;
    /// The bitmask for the U bit
    const U_BIT: u32 = 1 << 22;
    /// The bitmask for the Fast Interrupt bit
    const FI_BIT: u32 = 1 << 21;
    /// The bitmask for the Divide by Zero Fault bit
    const DZ_BIT: u32 = 1 << 18;
    /// The bitmask for the Background Region bit
    const BR_BIT: u32 = 1 << 17;
    /// The bitmask for the Round Robin bit
    const RR_BIT: u32 = 1 << 14;
    /// The bitmask for the Exception Vector Table bit
    const V_BIT: u32 = 1 << 13;
    /// The bitmask for the Instruction Cache enable bit
    const I_BIT: u32 = 1 << 12;
    /// The bitmask for the Branch Prediction enable bit
    const Z_BIT: u32 = 1 << 11;
    /// The bitmask for the SWP bit
    const SW_BIT: u32 = 1 << 10;
    /// The bitmask for the Cache enable bit
    const C_BIT: u32 = 1 << 2;
    /// The bitmask for the Alignment check bit
    const A_BIT: u32 = 1 << 1;
    /// The bitmask for the MPU bit
    const M_BIT: u32 = 1 << 0;

    /// Reads the *System Control Register*
    #[inline]
    pub fn read() -> Self {
        let r: u32;
        // Safety: Reading this register has no side-effects and is atomic
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("mrc p15, 0, {}, c1, c0, 0", out(reg) r, options(nomem, nostack, preserves_flags))
        };
        #[cfg(not(target_arch = "arm"))]
        {
            r = 0;
        }
        Self(r)
    }

    /// Write to the *System Control Register*
    #[inline]
    pub fn write(_value: Self) {
        // Safety: Writing this register is atomic
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("mcr p15, 0, {}, c1, c0, 0", in(reg) _value.0, options(nomem, nostack, preserves_flags))
        };
    }

    /// Modify the *System Control Register*
    #[inline]
    pub fn modify<F>(f: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut value = Self::read();
        f(&mut value);
        Self::write(value);
    }

    /// Is the IE bit set?
    pub fn ie(self) -> bool {
        (self.0 & Self::IE_BIT) != 0
    }

    /// Set the IE bit
    pub fn set_ie(&mut self) {
        self.0 |= Self::IE_BIT;
    }

    /// Clear the IE bit
    pub fn clear_ie(&mut self) {
        self.0 &= !Self::IE_BIT;
    }

    /// Is the TE bit set?
    pub fn te(self) -> bool {
        (self.0 & Self::TE_BIT) != 0
    }

    /// Set the TE bit
    pub fn set_te(&mut self) {
        self.0 |= Self::TE_BIT;
    }

    /// Clear the TE bit
    pub fn clear_te(&mut self) {
        self.0 &= !Self::TE_BIT;
    }

    /// Is the NMFI bit set?
    pub fn nmfi(self) -> bool {
        (self.0 & Self::NMFI_BIT) != 0
    }

    /// Set the NMFI bit
    pub fn set_nmfi(&mut self) {
        self.0 |= Self::NMFI_BIT;
    }

    /// Clear the NMFI bit
    pub fn clear_nmfi(&mut self) {
        self.0 &= !Self::NMFI_BIT;
    }

    /// Is the EE bit set?
    pub fn ee(self) -> bool {
        (self.0 & Self::EE_BIT) != 0
    }

    /// Set the EE bit
    pub fn set_ee(&mut self) {
        self.0 |= Self::EE_BIT;
    }

    /// Clear the EE bit
    pub fn clear_ee(&mut self) {
        self.0 &= !Self::EE_BIT;
    }

    /// Is the U bit set?
    pub fn u(self) -> bool {
        (self.0 & Self::U_BIT) != 0
    }

    /// Set the U bit
    pub fn set_u(&mut self) {
        self.0 |= Self::U_BIT;
    }

    /// Clear the U bit
    pub fn clear_u(&mut self) {
        self.0 &= !Self::U_BIT;
    }

    /// Is the FI bit set?
    pub fn fi(self) -> bool {
        (self.0 & Self::FI_BIT) != 0
    }

    /// Set the FI bit
    pub fn set_fi(&mut self) {
        self.0 |= Self::FI_BIT;
    }

    /// Clear the FI bit
    pub fn clear_fi(&mut self) {
        self.0 &= !Self::FI_BIT;
    }

    /// Is the DZ bit set?
    pub fn dz(self) -> bool {
        (self.0 & Self::DZ_BIT) != 0
    }

    /// Set the DZ bit
    pub fn set_dz(&mut self) {
        self.0 |= Self::DZ_BIT;
    }

    /// Clear the DZ bit
    pub fn clear_dz(&mut self) {
        self.0 &= !Self::DZ_BIT;
    }

    /// Is the BR bit set?
    pub fn br(self) -> bool {
        (self.0 & Self::BR_BIT) != 0
    }

    /// Set the BR bit
    pub fn set_br(&mut self) {
        self.0 |= Self::BR_BIT;
    }

    /// Clear the BR bit
    pub fn clear_br(&mut self) {
        self.0 &= !Self::BR_BIT;
    }

    /// Is the RR bit set?
    pub fn rr(self) -> bool {
        (self.0 & Self::RR_BIT) != 0
    }

    /// Set the RR bit
    pub fn set_rr(&mut self) {
        self.0 |= Self::RR_BIT;
    }

    /// Clear the RR bit
    pub fn clear_rr(&mut self) {
        self.0 &= !Self::RR_BIT;
    }

    /// Is the V bit set?
    pub fn v(self) -> bool {
        (self.0 & Self::V_BIT) != 0
    }

    /// Set the V bit
    pub fn set_v(&mut self) {
        self.0 |= Self::V_BIT;
    }

    /// Clear the V bit
    pub fn clear_v(&mut self) {
        self.0 &= !Self::V_BIT;
    }

    /// Is the I bit set?
    pub fn i(self) -> bool {
        (self.0 & Self::I_BIT) != 0
    }

    /// Set the I bit
    pub fn set_i(&mut self) {
        self.0 |= Self::I_BIT;
    }

    /// Clear the I bit
    pub fn clear_i(&mut self) {
        self.0 &= !Self::I_BIT;
    }

    /// Is the Z bit set?
    pub fn z(self) -> bool {
        (self.0 & Self::Z_BIT) != 0
    }

    /// Set the Z bit
    pub fn set_z(&mut self) {
        self.0 |= Self::Z_BIT;
    }

    /// Clear the Z bit
    pub fn clear_z(&mut self) {
        self.0 &= !Self::Z_BIT;
    }

    /// Is the SW bit set?
    pub fn sw(self) -> bool {
        (self.0 & Self::SW_BIT) != 0
    }

    /// Set the SW bit
    pub fn set_sw(&mut self) {
        self.0 |= Self::SW_BIT;
    }

    /// Clear the SW bit
    pub fn clear_sw(&mut self) {
        self.0 &= !Self::SW_BIT;
    }

    /// Is the C bit set?
    pub fn c(self) -> bool {
        (self.0 & Self::C_BIT) != 0
    }

    /// Set the C bit
    pub fn set_c(&mut self) {
        self.0 |= Self::C_BIT;
    }

    /// Clear the C bit
    pub fn clear_c(&mut self) {
        self.0 &= !Self::C_BIT;
    }

    /// Is the A bit set?
    pub fn a(self) -> bool {
        (self.0 & Self::A_BIT) != 0
    }

    /// Set the A bit
    pub fn set_a(&mut self) {
        self.0 |= Self::A_BIT;
    }

    /// Clear the A bit
    pub fn clear_a(&mut self) {
        self.0 &= !Self::A_BIT;
    }

    /// Is the M bit set?
    pub fn m(self) -> bool {
        (self.0 & Self::M_BIT) != 0
    }

    /// Set the M bit
    pub fn set_m(&mut self) {
        self.0 |= Self::M_BIT;
    }

    /// Clear the M bit
    pub fn clear_m(&mut self) {
        self.0 &= !Self::M_BIT;
    }
}

impl core::fmt::Debug for Sctlr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "SCTLR {{ IE={} TE={} NMFI={} EE={} U={} FI={} DZ={} BR={} RR={} V={} I={} Z={} SW={} C={} A={} M={} }}",
            self.ie() as u8,
            self.te() as u8,
            self.nmfi() as u8,
            self.ee() as u8,
            self.u() as u8,
            self.fi() as u8,
            self.dz() as u8,
            self.br() as u8,
            self.rr() as u8,
            self.v() as u8,
            self.i() as u8,
            self.z() as u8,
            self.sw() as u8,
            self.c() as u8,
            self.a() as u8,
            self.m() as u8,
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Sctlr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SCTLR {{ IE={0=31..32} TE={0=30..31} NMFI={0=27..28} EE={0=25..26} U={0=22..23} FI={0=21..22} DZ={0=18..19} BR={0=17..18} RR={0=14..15} V={0=13..14} I={0=12..13} Z={0=11..12} SW={0=10..11} C={0=2..3} A={0=1..2} M={0=0..1} }}", self.0)
    }
}
