// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::URIRequest;
use crate::URIResponse;
use crate::WebView;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitDownload")]
    pub struct Download(Object<ffi::WebKitDownload, ffi::WebKitDownloadClass>);

    match fn {
        type_ => || ffi::webkit_download_get_type(),
    }
}

impl Download {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`Download`].
    ///
    /// This method returns an instance of [`DownloadBuilder`] which can be used to create a [`Download`].
    pub fn builder() -> DownloadBuilder {
        DownloadBuilder::default()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`Download`].
pub struct DownloadBuilder {
    #[cfg(any(feature = "v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
    allow_overwrite: Option<bool>,
}

impl DownloadBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`DownloadBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Download`].
    pub fn build(self) -> Download {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v2_6", feature = "dox"))]
        if let Some(ref allow_overwrite) = self.allow_overwrite {
            properties.push(("allow-overwrite", allow_overwrite));
        }
        glib::Object::new::<Download>(&properties)
            .expect("Failed to create an instance of Download")
    }

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
    pub fn allow_overwrite(mut self, allow_overwrite: bool) -> Self {
        self.allow_overwrite = Some(allow_overwrite);
        self
    }
}

pub const NONE_DOWNLOAD: Option<&Download> = None;

pub trait DownloadExt: 'static {
    #[doc(alias = "webkit_download_cancel")]
    fn cancel(&self);

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
    #[doc(alias = "webkit_download_get_allow_overwrite")]
    #[doc(alias = "get_allow_overwrite")]
    fn allows_overwrite(&self) -> bool;

    #[doc(alias = "webkit_download_get_destination")]
    #[doc(alias = "get_destination")]
    fn destination(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_download_get_elapsed_time")]
    #[doc(alias = "get_elapsed_time")]
    fn elapsed_time(&self) -> f64;

    #[doc(alias = "webkit_download_get_estimated_progress")]
    #[doc(alias = "get_estimated_progress")]
    fn estimated_progress(&self) -> f64;

    #[doc(alias = "webkit_download_get_received_data_length")]
    #[doc(alias = "get_received_data_length")]
    fn received_data_length(&self) -> u64;

    #[doc(alias = "webkit_download_get_request")]
    #[doc(alias = "get_request")]
    fn request(&self) -> Option<URIRequest>;

    #[doc(alias = "webkit_download_get_response")]
    #[doc(alias = "get_response")]
    fn response(&self) -> Option<URIResponse>;

    #[doc(alias = "webkit_download_get_web_view")]
    #[doc(alias = "get_web_view")]
    fn web_view(&self) -> Option<WebView>;

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
    #[doc(alias = "webkit_download_set_allow_overwrite")]
    fn set_allow_overwrite(&self, allowed: bool);

    #[doc(alias = "webkit_download_set_destination")]
    fn set_destination(&self, uri: &str);

    #[doc(alias = "created-destination")]
    fn connect_created_destination<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "decide-destination")]
    fn connect_decide_destination<F: Fn(&Self, &str) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "failed")]
    fn connect_failed<F: Fn(&Self, &glib::Error) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "finished")]
    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "received-data")]
    fn connect_received_data<F: Fn(&Self, u64) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
    #[doc(alias = "allow-overwrite")]
    fn connect_allow_overwrite_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "destination")]
    fn connect_destination_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "estimated-progress")]
    fn connect_estimated_progress_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "response")]
    fn connect_response_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Download>> DownloadExt for O {
    fn cancel(&self) {
        unsafe {
            ffi::webkit_download_cancel(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
    fn allows_overwrite(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_download_get_allow_overwrite(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn destination(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_download_get_destination(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn elapsed_time(&self) -> f64 {
        unsafe { ffi::webkit_download_get_elapsed_time(self.as_ref().to_glib_none().0) }
    }

    fn estimated_progress(&self) -> f64 {
        unsafe { ffi::webkit_download_get_estimated_progress(self.as_ref().to_glib_none().0) }
    }

    fn received_data_length(&self) -> u64 {
        unsafe { ffi::webkit_download_get_received_data_length(self.as_ref().to_glib_none().0) }
    }

    fn request(&self) -> Option<URIRequest> {
        unsafe {
            from_glib_none(ffi::webkit_download_get_request(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn response(&self) -> Option<URIResponse> {
        unsafe {
            from_glib_none(ffi::webkit_download_get_response(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn web_view(&self) -> Option<WebView> {
        unsafe {
            from_glib_none(ffi::webkit_download_get_web_view(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
    fn set_allow_overwrite(&self, allowed: bool) {
        unsafe {
            ffi::webkit_download_set_allow_overwrite(
                self.as_ref().to_glib_none().0,
                allowed.into_glib(),
            );
        }
    }

    fn set_destination(&self, uri: &str) {
        unsafe {
            ffi::webkit_download_set_destination(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            );
        }
    }

    fn connect_created_destination<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn created_destination_trampoline<
            P: IsA<Download>,
            F: Fn(&P, &str) + 'static,
        >(
            this: *mut ffi::WebKitDownload,
            destination: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &Download::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(destination),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"created-destination\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    created_destination_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_decide_destination<F: Fn(&Self, &str) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn decide_destination_trampoline<
            P: IsA<Download>,
            F: Fn(&P, &str) -> bool + 'static,
        >(
            this: *mut ffi::WebKitDownload,
            suggested_filename: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &Download::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(suggested_filename),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"decide-destination\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    decide_destination_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_failed<F: Fn(&Self, &glib::Error) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn failed_trampoline<
            P: IsA<Download>,
            F: Fn(&P, &glib::Error) + 'static,
        >(
            this: *mut ffi::WebKitDownload,
            error: *mut glib::ffi::GError,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &Download::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(error),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"failed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    failed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn finished_trampoline<P: IsA<Download>, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDownload,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Download::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"finished\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    finished_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_received_data<F: Fn(&Self, u64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn received_data_trampoline<
            P: IsA<Download>,
            F: Fn(&P, u64) + 'static,
        >(
            this: *mut ffi::WebKitDownload,
            data_length: u64,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &Download::from_glib_borrow(this).unsafe_cast_ref(),
                data_length,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"received-data\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    received_data_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
    fn connect_allow_overwrite_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_allow_overwrite_trampoline<
            P: IsA<Download>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDownload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Download::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::allow-overwrite\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_allow_overwrite_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_destination_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_destination_trampoline<
            P: IsA<Download>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDownload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Download::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::destination\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_destination_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_estimated_progress_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_estimated_progress_trampoline<
            P: IsA<Download>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDownload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Download::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::estimated-progress\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_estimated_progress_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_response_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_response_trampoline<P: IsA<Download>, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDownload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Download::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::response\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_response_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Download {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Download")
    }
}
