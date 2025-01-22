//! IMPLEMENTATION DETAILS USED BY MACROS

use core::{
    cell::RefCell,
    fmt::{self, Write},
};

use crate::hio::{self, HostStream};

static HSTDOUT: critical_section::Mutex<RefCell<Option<HostStream>>> =
    critical_section::Mutex::new(RefCell::new(None));

pub fn hstdout_str(s: &str) {
    let _result: Result<(), hio::Error> = critical_section::with(|cs| {
        let mut guard = HSTDOUT.borrow_ref_mut(cs);
        if guard.is_none() {
            let handle = hio::hstdout()?;
            *guard = Some(handle);
        }
        guard.as_mut().unwrap().write_all(s.as_bytes())
    });
}

pub fn hstdout_fmt(args: fmt::Arguments) {
    let _result: Result<(), hio::Error> = critical_section::with(|cs| {
        let mut guard = HSTDOUT.borrow_ref_mut(cs);
        if guard.is_none() {
            let handle = hio::hstdout()?;
            *guard = Some(handle);
        }
        let _ = guard.as_mut().unwrap().write_fmt(args);
        Ok(())
    });
}

static HSTDERR: critical_section::Mutex<RefCell<Option<HostStream>>> =
    critical_section::Mutex::new(RefCell::new(None));

pub fn hstderr_str(s: &str) {
    let _result: Result<(), hio::Error> = critical_section::with(|cs| {
        let mut guard = HSTDERR.borrow_ref_mut(cs);
        if guard.is_none() {
            let handle = hio::hstderr()?;
            *guard = Some(handle);
        }
        guard.as_mut().unwrap().write_all(s.as_bytes())
    });
}

pub fn hstderr_fmt(args: fmt::Arguments) {
    let _result: Result<(), hio::Error> = critical_section::with(|cs| {
        let mut guard = HSTDERR.borrow_ref_mut(cs);
        if guard.is_none() {
            let handle = hio::hstderr()?;
            *guard = Some(handle);
        }
        let _ = guard.as_mut().unwrap().write_fmt(args);
        Ok(())
    });
}
