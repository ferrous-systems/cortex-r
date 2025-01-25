//! Code for managing the *Current Program Status Register*

/// The *Current Program Status Register* (CPSR)
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Cpsr(u32);

impl Cpsr {
    /// The bitmask for Negative Result from ALU
    pub const N_BIT: u32 = 1 << 31;
    /// The bitmask for Zero Result from ALU
    pub const Z_BIT: u32 = 1 << 30;
    /// The bitmask for ALU operation Carry Out
    pub const C_BIT: u32 = 1 << 29;
    /// The bitmask for ALU operation Overflow
    pub const V_BIT: u32 = 1 << 28;
    /// The bitmask for Cumulative Saturation
    pub const Q_BIT: u32 = 1 << 27;
    /// The bitmask for Jazelle State
    pub const J_BIT: u32 = 1 << 24;
    /// The bitmask for Endianness
    pub const E_BIT: u32 = 1 << 9;
    /// The bitmask for Asynchronous Aborts
    pub const A_BIT: u32 = 1 << 8;
    /// The bitmask for Interrupts Enabled
    pub const I_BIT: u32 = 1 << 7;
    /// The bitmask for Fast Interrupts Enabled
    pub const F_BIT: u32 = 1 << 6;
    /// The bitmask for Thumb state
    pub const T_BIT: u32 = 1 << 5;
    /// The bitmask for Processor Mode
    pub const MODE_BITS: u32 = 0x1F;

    /// Reads the *Current Program Status Register*
    #[inline]
    pub fn read() -> Self {
        let r: u32;
        // Safety: Reading this register has no side-effects and is atomic
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("mrs {}, CPSR", out(reg) r, options(nomem, nostack, preserves_flags));
        }
        #[cfg(not(target_arch = "arm"))]
        {
            r = 0;
        }
        Self(r)
    }

    /// Is the N bit set?
    #[inline]
    pub fn n(self) -> bool {
        (self.0 & Self::N_BIT) != 0
    }

    /// Is the Z bit set?
    #[inline]
    pub fn z(self) -> bool {
        (self.0 & Self::Z_BIT) != 0
    }

    /// Is the C bit set?
    #[inline]
    pub fn c(self) -> bool {
        (self.0 & Self::C_BIT) != 0
    }

    /// Is the V bit set?
    #[inline]
    pub fn v(self) -> bool {
        (self.0 & Self::V_BIT) != 0
    }

    /// Is the Q bit set?
    #[inline]
    pub fn q(self) -> bool {
        (self.0 & Self::Q_BIT) != 0
    }

    /// Is the J bit set?
    #[inline]
    pub fn j(self) -> bool {
        (self.0 & Self::J_BIT) != 0
    }

    /// Is the E bit set?
    #[inline]
    pub fn e(self) -> bool {
        (self.0 & Self::E_BIT) != 0
    }

    /// Is the A bit set?
    #[inline]
    pub fn a(self) -> bool {
        (self.0 & Self::A_BIT) != 0
    }

    /// Is the I bit set?
    #[inline]
    pub fn i(self) -> bool {
        (self.0 & Self::I_BIT) != 0
    }

    /// Is the F bit set?
    #[inline]
    pub fn f(self) -> bool {
        (self.0 & Self::F_BIT) != 0
    }

    /// Is the T bit set?
    #[inline]
    pub fn t(self) -> bool {
        (self.0 & Self::T_BIT) != 0
    }

    /// Get the current mode
    #[inline]
    pub fn mode(self) -> u8 {
        (self.0 & Self::MODE_BITS) as u8
    }

    /// Are we in supervisor mode?
    #[inline]
    pub fn is_supervisor_mode(self) -> bool {
        self.mode() == 0b10011
    }
}

impl core::fmt::Debug for Cpsr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "CPSR {{ N={} Z={} C={} V={} Q={} J={} E={} A={} I={} F={} T={} MODE={:#02x} }}",
            self.n() as u8,
            self.z() as u8,
            self.c() as u8,
            self.v() as u8,
            self.q() as u8,
            self.j() as u8,
            self.e() as u8,
            self.a() as u8,
            self.i() as u8,
            self.f() as u8,
            self.t() as u8,
            self.mode(),
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Cpsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CPSR {{ N={0=31..32} Z={0=30..31} C={0=29..30} V={0=28..29} Q={0=27..28} J={0=24..25} E={0=9..10} A={0=8..9} I={0=7..8} F={0=6..7} T={0=5..6} MODE={0=0..5} }}", self.0)
    }
}
