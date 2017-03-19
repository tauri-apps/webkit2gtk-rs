/*
 * Copyright (c) 2016 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

use std::boxed::Box as Box_;
use std::error::Error;
use std::ffi::CString;
use std::mem::transmute;
use std::ptr;

use ffi;
use gio_sys::{self, GCancellable};
use glib::{IsA, StaticType, error};
use glib::object::Downcast;
use glib::signal::connect;
use glib::translate::{FromGlibPtrNone, ToGlib, ToGlibPtr, from_glib_full};
use glib_ffi::{self, GError};
use gobject_ffi;
use gtk;
use libc::c_void;

use super::{JavascriptResult, WebContext, WebView};
#[cfg(feature = "v2_6")]
use super::UserContentManager;

type AsyncCallback = Option<unsafe extern "C" fn(*mut gobject_ffi::GObject, *mut gio_sys::GAsyncResult, *mut c_void)>;

fn connect_notify<F: Fn() + 'static, O: IsA<WebView>>(property_name: &str, view: &O, callback: F) {
    let callback: Box<Box<Fn() + 'static>> = Box::new(Box::new(callback));
    let this: *mut ffi::WebKitWebView = view.to_glib_none().0;
    unsafe {
        connect(this as *mut _, &format!("notify::{}", property_name), transmute(notify_trampoline as usize), Box::into_raw(callback) as *mut _);
    }
}

pub trait WebViewExt {
    #[cfg(feature = "v2_6")]
    fn new_with_context_and_user_content_manager(context: &WebContext, user_content_manager: &UserContentManager) -> Self;
    fn connect_title_changed<F: Fn() + 'static>(&self, callback: F);
    fn connect_uri_changed<F: Fn() + 'static>(&self, callback: F);
    fn run_javascript(&self, script: &str);
    fn run_javascript_with_callback<F: Fn(Result<JavascriptResult, error::Error>) + 'static>(&self, script: &str, callback: F);
}

impl<O> WebViewExt for O
    where O: IsA<gtk::Widget> + IsA<WebView>
{
    #[cfg(feature = "v2_6")]
    fn new_with_context_and_user_content_manager(context: &WebContext, user_content_manager: &UserContentManager) -> Self {
        let user_content_manager_property = CString::new("user-content-manager").unwrap();
        let web_context_property = CString::new("web-context").unwrap();
        let glib_user_content_manager: *mut gobject_ffi::GObject = user_content_manager.to_glib_none().0;
        let glib_context: *mut gobject_ffi::GObject = context.to_glib_none().0;
        let null: *mut gobject_ffi::GObject = ptr::null_mut();
        unsafe {
            gtk::Widget::from_glib_none(gobject_ffi::g_object_new(WebView::static_type().to_glib(), user_content_manager_property.as_ptr(), glib_user_content_manager, web_context_property.as_ptr(), glib_context, null) as *mut _).downcast_unchecked()
        }
    }

    fn connect_title_changed<F: Fn() + 'static>(&self, callback: F) {
        connect_notify("title", self, callback);
    }

    fn connect_uri_changed<F: Fn() + 'static>(&self, callback: F) {
        connect_notify("uri", self, callback);
    }

    fn run_javascript(&self, script: &str) {
        unsafe { ffi::webkit_web_view_run_javascript(self.to_glib_none().0, script.to_glib_none().0, ptr::null_mut::<GCancellable>(), None, ptr::null_mut::<c_void>()) }
    }

    fn run_javascript_with_callback<F: Fn(Result<JavascriptResult, error::Error>) + 'static>(&self, script: &str, callback: F) {
        let trampoline: AsyncCallback = unsafe { transmute(async_ready_trampoline as usize) };
        let f: Box_<Box_<Fn(Result<JavascriptResult, error::Error>) + 'static>> = Box_::new(Box_::new(callback));
        let user_data: *mut c_void = Box_::into_raw(f) as *mut _;
        unsafe { ffi::webkit_web_view_run_javascript(self.to_glib_none().0, script.to_glib_none().0, ptr::null_mut::<GCancellable>(), trampoline, user_data) }
    }
}

unsafe extern "C" fn async_ready_trampoline(this: *mut gobject_ffi::GObject, result: *mut gio_sys::GAsyncResult, f: glib_ffi::gpointer) {
    callback_guard!();
    let mut error = ptr::null_mut();
    let javascript_result = ffi::webkit_web_view_run_javascript_finish(this as *mut _, result, &mut error);
    let value =
        if javascript_result.is_null() {
            Err(from_glib_full(error))
        }
        else {
            Ok(JavascriptResult::from_glib_none(javascript_result))
        };
    let f: &Box_<Fn(Result<JavascriptResult, error::Error>) + 'static> = &*(f as *const _);
    f(value)
}

unsafe extern "C" fn notify_trampoline(_gobject: *mut ::gobject_ffi::GObject, _pspec: *mut ::gobject_ffi::GParamSpec, user_data: ::glib_ffi::gpointer) {
    let callback: &Box<Fn() + 'static> = transmute(user_data);
    callback();
}
