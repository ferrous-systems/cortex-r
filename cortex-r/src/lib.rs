//! CPU/peripheral support for Arm Cortex-R

#![no_std]

#[cfg(feature = "critical-section-single-core")]
mod critical_section;

pub mod register;

pub mod interrupt;

pub mod asm;

/// Generate an SVC call with the given argument
///
/// Safe to call in Supervisor mode because it pushes LR and SPSR to the stack
/// before the call, and restores them afterwards.
#[macro_export]
macro_rules! svc {
    ($r0:expr) => {
        unsafe {
            core::arch::asm!(r#"
                // save lr
                push    {{lr}}
                // Get spsr
                mrs     lr, spsr           
                // save spsr
                push    {{lr}}                                    
                // call software interrupt
                svc {}                     
                // Get spsr from stack
                pop     {{lr}}          
                // Restore spsr
                msr     spsr, lr      
                // restore
                pop     {{lr}}          
            "#, const $r0);
        }
    }
}

/// Generate an SVC call with the given argument
///
/// Only works in User mode. Do not call in Supervisor mode (the default mode)
/// because it will trash your LR and processor status.
#[macro_export]
macro_rules! user_svc {
    ($r0:expr) => {
        unsafe {
            core::arch::asm!("svc {}", const $r0);
        }
    }
}
