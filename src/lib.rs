#![allow(unused_imports)]

#[macro_use]
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gtk;
extern crate gtk_sys as gtk_ffi;
extern crate libc;

extern crate webkit2_sys as ffi;

macro_rules! callback_guard {
    () => (
        let _guard = ::glib::CallbackGuard::new();
    )
}

mod auto;

pub use auto::*;
