// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use cairo;
use gio;
use gio_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gobject_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;
use webkit2_sys;

glib_wrapper! {
    pub struct FaviconDatabase(Object<webkit2_sys::WebKitFaviconDatabase, webkit2_sys::WebKitFaviconDatabaseClass, FaviconDatabaseClass>);

    match fn {
        get_type => || webkit2_sys::webkit_favicon_database_get_type(),
    }
}

pub const NONE_FAVICON_DATABASE: Option<&FaviconDatabase> = None;

pub trait FaviconDatabaseExt: 'static {
    fn clear(&self);

    fn get_favicon<P: IsA<gio::Cancellable>, Q: FnOnce(Result<cairo::Surface, glib::Error>) + Send + 'static>(&self, page_uri: &str, cancellable: Option<&P>, callback: Q);

    
    fn get_favicon_future(&self, page_uri: &str) -> Pin<Box_<dyn std::future::Future<Output = Result<cairo::Surface, glib::Error>> + 'static>>;

    fn get_favicon_uri(&self, page_uri: &str) -> Option<GString>;

    fn connect_favicon_changed<F: Fn(&Self, &str, &str) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FaviconDatabase>> FaviconDatabaseExt for O {
    fn clear(&self) {
        unsafe {
            webkit2_sys::webkit_favicon_database_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn get_favicon<P: IsA<gio::Cancellable>, Q: FnOnce(Result<cairo::Surface, glib::Error>) + Send + 'static>(&self, page_uri: &str, cancellable: Option<&P>, callback: Q) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn get_favicon_trampoline<Q: FnOnce(Result<cairo::Surface, glib::Error>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer) {
            let mut error = ptr::null_mut();
            let ret = webkit2_sys::webkit_favicon_database_get_favicon_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = get_favicon_trampoline::<Q>;
        unsafe {
            webkit2_sys::webkit_favicon_database_get_favicon(self.as_ref().to_glib_none().0, page_uri.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    fn get_favicon_future(&self, page_uri: &str) -> Pin<Box_<dyn std::future::Future<Output = Result<cairo::Surface, glib::Error>> + 'static>> {

        let page_uri = String::from(page_uri);
        Box_::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.get_favicon(
                &page_uri,
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    fn get_favicon_uri(&self, page_uri: &str) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_sys::webkit_favicon_database_get_favicon_uri(self.as_ref().to_glib_none().0, page_uri.to_glib_none().0))
        }
    }

    fn connect_favicon_changed<F: Fn(&Self, &str, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn favicon_changed_trampoline<P, F: Fn(&P, &str, &str) + 'static>(this: *mut webkit2_sys::WebKitFaviconDatabase, page_uri: *mut libc::c_char, favicon_uri: *mut libc::c_char, f: glib_sys::gpointer)
            where P: IsA<FaviconDatabase>
        {
            let f: &F = &*(f as *const F);
            f(&FaviconDatabase::from_glib_borrow(this).unsafe_cast(), &GString::from_glib_borrow(page_uri), &GString::from_glib_borrow(favicon_uri))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"favicon-changed\0".as_ptr() as *const _,
                Some(transmute(favicon_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for FaviconDatabase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FaviconDatabase")
    }
}
