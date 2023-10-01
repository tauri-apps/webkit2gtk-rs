// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use glib::{prelude::*,translate::*};

glib::wrapper! {
    #[doc(alias = "WebKitBackForwardListItem")]
    pub struct BackForwardListItem(Object<ffi::WebKitBackForwardListItem, ffi::WebKitBackForwardListItemClass>);

    match fn {
        type_ => || ffi::webkit_back_forward_list_item_get_type(),
    }
}

impl BackForwardListItem {
        pub const NONE: Option<&'static BackForwardListItem> = None;
    
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::BackForwardListItem>> Sealed for T {}
}

pub trait BackForwardListItemExt: IsA<BackForwardListItem> + sealed::Sealed + 'static {
    #[doc(alias = "webkit_back_forward_list_item_get_original_uri")]
    #[doc(alias = "get_original_uri")]
    fn original_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_item_get_original_uri(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "webkit_back_forward_list_item_get_title")]
    #[doc(alias = "get_title")]
    fn title(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_item_get_title(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "webkit_back_forward_list_item_get_uri")]
    #[doc(alias = "get_uri")]
    fn uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_item_get_uri(self.as_ref().to_glib_none().0))
        }
    }
}

impl<O: IsA<BackForwardListItem>> BackForwardListItemExt for O {}
