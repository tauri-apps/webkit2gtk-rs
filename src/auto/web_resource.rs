// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use URIRequest;
use URIResponse;
use gio;
use gio_sys;
use glib;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;
use webkit2_sys;

glib_wrapper! {
    pub struct WebResource(Object<webkit2_sys::WebKitWebResource, webkit2_sys::WebKitWebResourceClass, WebResourceClass>);

    match fn {
        get_type => || webkit2_sys::webkit_web_resource_get_type(),
    }
}

pub const NONE_WEB_RESOURCE: Option<&WebResource> = None;

pub trait WebResourceExt: 'static {
    fn get_data<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(Vec<u8>, usize), glib::Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q);

    
    fn get_data_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<(Vec<u8>, usize), glib::Error>> + 'static>>;

    fn get_response(&self) -> Option<URIResponse>;

    fn get_uri(&self) -> Option<GString>;

    fn connect_failed<F: Fn(&Self, &glib::Error) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn connect_failed_with_tls_errors<F: Fn(&Self, &gio::TlsCertificate, gio::TlsCertificateFlags) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_received_data<F: Fn(&Self, u64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_sent_request<F: Fn(&Self, &URIRequest, &URIResponse) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_response_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<WebResource>> WebResourceExt for O {
    fn get_data<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(Vec<u8>, usize), glib::Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn get_data_trampoline<Q: FnOnce(Result<(Vec<u8>, usize), glib::Error>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer) {
            let mut error = ptr::null_mut();
            let mut length = mem::MaybeUninit::uninit();
            let ret = webkit2_sys::webkit_web_resource_get_data_finish(_source_object as *mut _, res, length.as_mut_ptr(), &mut error);
            let result = if error.is_null() { Ok((FromGlibContainer::from_glib_full_num(ret, length.assume_init() as usize), length.assume_init() as usize)) } else { Err(from_glib_full(error)) };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = get_data_trampoline::<Q>;
        unsafe {
            webkit2_sys::webkit_web_resource_get_data(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    fn get_data_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<(Vec<u8>, usize), glib::Error>> + 'static>> {
        use gio::GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            obj.get_data(
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    fn get_response(&self) -> Option<URIResponse> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_web_resource_get_response(self.as_ref().to_glib_none().0))
        }
    }

    fn get_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_web_resource_get_uri(self.as_ref().to_glib_none().0))
        }
    }

    fn connect_failed<F: Fn(&Self, &glib::Error) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn failed_trampoline<P, F: Fn(&P, &glib::Error) + 'static>(this: *mut webkit2_sys::WebKitWebResource, error: *mut glib_sys::GError, f: glib_sys::gpointer)
            where P: IsA<WebResource>
        {
            let f: &F = &*(f as *const F);
            f(&WebResource::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(error))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"failed\0".as_ptr() as *const _,
                Some(transmute(failed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn connect_failed_with_tls_errors<F: Fn(&Self, &gio::TlsCertificate, gio::TlsCertificateFlags) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn failed_with_tls_errors_trampoline<P, F: Fn(&P, &gio::TlsCertificate, gio::TlsCertificateFlags) + 'static>(this: *mut webkit2_sys::WebKitWebResource, certificate: *mut gio_sys::GTlsCertificate, errors: gio_sys::GTlsCertificateFlags, f: glib_sys::gpointer)
            where P: IsA<WebResource>
        {
            let f: &F = &*(f as *const F);
            f(&WebResource::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(certificate), from_glib(errors))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"failed-with-tls-errors\0".as_ptr() as *const _,
                Some(transmute(failed_with_tls_errors_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn finished_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_sys::WebKitWebResource, f: glib_sys::gpointer)
            where P: IsA<WebResource>
        {
            let f: &F = &*(f as *const F);
            f(&WebResource::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"finished\0".as_ptr() as *const _,
                Some(transmute(finished_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_received_data<F: Fn(&Self, u64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn received_data_trampoline<P, F: Fn(&P, u64) + 'static>(this: *mut webkit2_sys::WebKitWebResource, data_length: u64, f: glib_sys::gpointer)
            where P: IsA<WebResource>
        {
            let f: &F = &*(f as *const F);
            f(&WebResource::from_glib_borrow(this).unsafe_cast(), data_length)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"received-data\0".as_ptr() as *const _,
                Some(transmute(received_data_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_sent_request<F: Fn(&Self, &URIRequest, &URIResponse) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn sent_request_trampoline<P, F: Fn(&P, &URIRequest, &URIResponse) + 'static>(this: *mut webkit2_sys::WebKitWebResource, request: *mut webkit2_sys::WebKitURIRequest, redirected_response: *mut webkit2_sys::WebKitURIResponse, f: glib_sys::gpointer)
            where P: IsA<WebResource>
        {
            let f: &F = &*(f as *const F);
            f(&WebResource::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(request), &from_glib_borrow(redirected_response))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"sent-request\0".as_ptr() as *const _,
                Some(transmute(sent_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_response_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_response_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_sys::WebKitWebResource, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<WebResource>
        {
            let f: &F = &*(f as *const F);
            f(&WebResource::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::response\0".as_ptr() as *const _,
                Some(transmute(notify_response_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_uri_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_sys::WebKitWebResource, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<WebResource>
        {
            let f: &F = &*(f as *const F);
            f(&WebResource::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::uri\0".as_ptr() as *const _,
                Some(transmute(notify_uri_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for WebResource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WebResource")
    }
}
