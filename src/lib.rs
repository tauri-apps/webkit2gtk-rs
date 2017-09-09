/*
 * TODO: add Cargo categories.
 * TODO: add all unstable methods.
 */

#![allow(unused_imports)]

#[macro_use]
extern crate bitflags;
extern crate gio_sys;
#[macro_use]
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gtk;
extern crate gtk_sys as gtk_ffi;
extern crate javascriptcore as java_script_core;
extern crate libc;

extern crate webkit2gtk_sys as ffi;

macro_rules! assert_initialized_main_thread {
    () => (
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            }
            else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    )
}

macro_rules! skip_assert_initialized {
    () => ()
}

macro_rules! callback_guard {
    () => (
        let _guard = ::glib::CallbackGuard::new();
    )
}

mod auto;
mod script_dialog;
mod web_view;

pub use glib::Error;

pub use auto::*;
pub use script_dialog::*;
pub use web_view::*;
