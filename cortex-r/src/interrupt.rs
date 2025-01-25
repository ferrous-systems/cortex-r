//! Interrupts on Arm Cortex-R

/// Enable interrupts
///
/// Doesn't work in User mode
///
/// # Safety
///
/// Do not call this function inside an interrupt-based critical section
#[inline]
pub unsafe fn enable() {
    // Safety: We're atomically setting a bit in a special register, and we're
    // in an unsafe function that places restrictions on when you can call it
    #[cfg(target_arch = "arm")]
    unsafe {
        core::arch::asm!("cpsie i", options(nomem, nostack, preserves_flags));
    };
}

/// Disable interrupts
///
/// Doesn't work in User mode
#[inline]
pub fn disable() {
    // Safety: We're atomically clearing a bit in a special register
    #[cfg(target_arch = "arm")]
    unsafe {
        core::arch::asm!("cpsid i", options(nomem, nostack, preserves_flags));
    };
}

/// Run with interrupts disabled
///
/// Doesn't work in User mode
#[inline]
pub fn free<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let cpsr = crate::register::Cpsr::read();
    disable();
    let result = f();
    if cpsr.i() {
        // Safety: We're only turning them back on if they were on previously
        unsafe {
            enable();
        }
    }
    result
}
