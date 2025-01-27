//! Code for the *Configuration Base Address Register*

/// The *Configuration Base Address Register* (CBAR)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cbar(u32);

impl Cbar {
    /// Reads the *Configuration Base Address Register*
    #[inline]
    pub fn read() -> Cbar {
        let r: u32;
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("mrc p15, 1, {}, c15, c3, 0", out(reg) r, options(nomem, nostack, preserves_flags));
        }
        #[cfg(not(target_arch = "arm"))]
        {
            r = 0;
        }
        Self(r)
    }

    /// Get the periphbase address
    pub fn periphbase(self) -> *mut u64 {
        (self.0 & 0xFFFFF) as *mut u64
    }
}

impl core::fmt::Debug for Cbar {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CBAR {{ PERIPHBASE={:p} }}", self.periphbase())
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Cbar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CBAR {{ PERIPHBASE=0x{:08x} }}", self.0)
    }
}
