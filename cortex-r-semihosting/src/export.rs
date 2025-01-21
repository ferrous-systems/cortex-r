//! IMPLEMENTATION DETAILS USED BY MACROS

use core::{
    cell::RefCell,
    fmt::{self, Write},
};

use crate::hio::{self, HostStream};

static HSTDOUT: critical_section::Mutex<RefCell<Option<HostStream>>> =
    critical_section::Mutex::new(RefCell::new(None));

pub fn hstdout_str(s: &str) {
    let _result = critical_section::with(|cs| {
        let mut guard = HSTDOUT.borrow_ref_mut(cs);
        if guard.is_none() {
            *guard = Some(hio::hstdout()?);
        }
        guard.as_mut().unwrap().write_str(s).map_err(drop)
    });
}

pub fn hstdout_fmt(args: fmt::Arguments) {
    let _result = critical_section::with(|cs| {
        let mut guard = HSTDOUT.borrow_ref_mut(cs);
        if guard.is_none() {
            *guard = Some(hio::hstdout()?);
        }
        guard.as_mut().unwrap().write_fmt(args).map_err(drop)
    });
}

static HSTDERR: critical_section::Mutex<RefCell<Option<HostStream>>> =
    critical_section::Mutex::new(RefCell::new(None));

pub fn hstderr_str(s: &str) {
    let _result = critical_section::with(|cs| {
        let mut guard = HSTDERR.borrow_ref_mut(cs);
        if guard.is_none() {
            *guard = Some(hio::hstderr()?);
        }
        guard.as_mut().unwrap().write_str(s).map_err(drop)
    });
}

pub fn hstderr_fmt(args: fmt::Arguments) {
    let _result = critical_section::with(|cs| {
        let mut guard = HSTDERR.borrow_ref_mut(cs);
        if guard.is_none() {
            *guard = Some(hio::hstderr()?);
        }
        guard.as_mut().unwrap().write_fmt(args).map_err(drop)
    });
}
