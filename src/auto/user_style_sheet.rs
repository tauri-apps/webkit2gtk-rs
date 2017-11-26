// This file was generated by gir (d933f9a) from gir-files (469db10)
// DO NOT EDIT

#[cfg(any(feature = "v2_6", feature = "dox"))]
use UserContentInjectedFrames;
#[cfg(any(feature = "v2_6", feature = "dox"))]
use UserStyleLevel;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct UserStyleSheet(Shared<ffi::WebKitUserStyleSheet>);

    match fn {
        ref => |ptr| ffi::webkit_user_style_sheet_ref(ptr),
        unref => |ptr| ffi::webkit_user_style_sheet_unref(ptr),
        get_type => || ffi::webkit_user_style_sheet_get_type(),
    }
}

impl UserStyleSheet {
    #[cfg(any(feature = "v2_6", feature = "dox"))]
    pub fn new(source: &str, injected_frames: UserContentInjectedFrames, level: UserStyleLevel, whitelist: &[&str], blacklist: &[&str]) -> UserStyleSheet {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_user_style_sheet_new(source.to_glib_none().0, injected_frames.to_glib(), level.to_glib(), whitelist.to_glib_none().0, blacklist.to_glib_none().0))
        }
    }
}
