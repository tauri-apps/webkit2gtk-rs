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
use std::mem::transmute;
use std::ptr;

use ffi;
use gio::AsyncResult;
use gio_sys::{self, GCancellable};
use glib::IsA;
use glib::error;
use glib::object::Downcast;
use glib::translate::{FromGlibPtr, ToGlibPtr, from_glib_full};
use glib_ffi::{self, GError};
use gobject_ffi;
use libc::c_void;

use super::JavascriptResult;
use super::WebView;

type AsyncCallback = Option<unsafe extern "C" fn(*mut gobject_ffi::GObject, *mut gio_sys::GAsyncResult, *mut c_void)>;

pub trait WebViewExt {
    fn run_javascript(&self, script: &str);
    fn run_javascript_with_callback<F: Fn(Result<JavascriptResult, error::Error>) + 'static>(&self, script: &str, callback: F);
}

impl<O: IsA<WebView>> WebViewExt for O {
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
    let f: &Box_<Fn(Result<JavascriptResult, error::Error>) + 'static> = transmute(f);
    f(value)
}
