// This file was generated by gir (d933f9a) from gir-files (469db10)
// DO NOT EDIT

use MimeInfo;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Plugin(Object<ffi::WebKitPlugin, ffi::WebKitPluginClass>);

    match fn {
        get_type => || ffi::webkit_plugin_get_type(),
    }
}

pub trait PluginExt {
    fn get_description(&self) -> Option<String>;

    fn get_mime_info_list(&self) -> Vec<MimeInfo>;

    fn get_name(&self) -> Option<String>;

    fn get_path(&self) -> Option<String>;
}

impl<O: IsA<Plugin>> PluginExt for O {
    fn get_description(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_plugin_get_description(self.to_glib_none().0))
        }
    }

    fn get_mime_info_list(&self) -> Vec<MimeInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::webkit_plugin_get_mime_info_list(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_plugin_get_name(self.to_glib_none().0))
        }
    }

    fn get_path(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_plugin_get_path(self.to_glib_none().0))
        }
    }
}
