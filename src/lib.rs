#![allow(deprecated)]
#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;

// Re-export gtk dependencies
pub use gio;
#[macro_use]
pub use glib;

pub use glib::Error;

#[macro_use]
use bitflags;
use cairo;
use gdk;
use gdk_sys;
use gio_sys;
use glib_sys;
use gobject_sys;
use gtk;
use gtk_sys;
use libc;

#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(useless_transmute))]
#[macro_use]
mod rt;

#[allow(unused_imports)]
mod auto;
mod credential;
mod javascript_result;
mod web_context;
mod web_view;

#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
mod website_data_manager;

pub use credential::*;
pub use javascript_result::*;
pub use web_context::*;
pub use web_view::*;

#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
pub use website_data_manager::*;

pub use crate::auto::builders::*;
pub use crate::auto::traits::*;
pub use crate::auto::*;
