//! Code for the *Vector Base Address Register*

/// The *Vector Base Address Register* (VBAR)
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Vbar;

impl Vbar {
    /// Reads the *Vector Base Address Register*
    #[inline]
    pub fn read() -> usize {
        let r: usize;
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("mrc p15, 0, {}, c12, c0, 0", out(reg) r, options(nomem, nostack, preserves_flags));
        }
        #[cfg(not(target_arch = "arm"))]
        {
            r = 0;
        }
        r
    }

    /// Writes the *Vector Base Address Register*
    ///
    /// # Safety
    ///
    /// You must supply a correctly-aligned address of a valid Arm Cortex-R
    /// Vector Table.
    #[inline]
    pub unsafe fn write(_value: usize) {
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("mcr p15, 0, {}, c12, c0, 0", in(reg) _value, options(nomem, nostack, preserves_flags));
        }
    }
}
