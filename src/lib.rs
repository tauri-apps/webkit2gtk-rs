// Take a look at the license at the top of the repository in the LICENSE file.

/*
 * TODO: add Cargo categories.
 * TODO: add all unstable methods.
 */

#![allow(unused_imports)]

#[macro_use]
use bitflags;
use cairo;
use gdk;
use gdk_sys;
use gio;
use gio_sys;
#[macro_use]
use glib;
use glib_sys;
use gobject_sys;
use gtk;
use gtk_sys;
use javascriptcore as java_script_core;
use libc;
use webkit2gtk_sys as webkit2_sys;

macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

mod auto;
mod script_dialog;
mod web_view;

pub use glib::Error;

pub use auto::*;
pub use script_dialog::*;
pub use web_view::*;
