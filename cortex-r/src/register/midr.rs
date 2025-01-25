//! Code for managing the *Main ID Register*

/// The *Main ID Register* (MIDR)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Midr(u32);

impl Midr {
    /// Reads the *Main ID Register*
    #[inline]
    pub fn read() -> Midr {
        let r: u32;
        // Safety: Reading this register has no side-effects and is atomic
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("mrc p15, 0, {}, c0, c0, 0", out(reg) r, options(nomem, nostack, preserves_flags))
        };
        #[cfg(not(target_arch = "arm"))]
        {
            r = 0;
        }
        Self(r)
    }

    /// Get the implementer field
    pub fn implementer(self) -> u32 {
        self.0 >> 24
    }

    /// Get the variant field
    pub fn variant(self) -> u32 {
        (self.0 >> 20) & 0xF
    }

    /// Get the arch field
    pub fn arch(self) -> u32 {
        (self.0 >> 16) & 0xF
    }

    /// Get the primary part number field
    pub fn part_no(self) -> u32 {
        (self.0 >> 4) & 0xFFF
    }

    /// Get the rev field
    pub fn rev(self) -> u32 {
        self.0 & 0xF
    }
}

impl core::fmt::Debug for Midr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "MIDR {{ implementer=0x{:02x} variant=0x{:x} arch=0x{:x} part_no=0x{:03x} rev=0x{:x} }}",
        self.implementer(), self.variant(), self.arch(), self.part_no(), self.rev())
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Midr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MIDR {{ implementer=0x{0=24..32:02x} variant=0x{0=20..24:x} arch=0x{0=16..20:x} part_no=0x{0=4..16:03x} rev=0x{0=0..4:x} }}", self.0)
    }
}
