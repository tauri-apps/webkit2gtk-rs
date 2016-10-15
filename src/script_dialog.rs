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

use std::ffi::{CStr, CString};

use ffi;
use glib::translate::{ToGlib, ToGlibPtrMut, from_glib};

use super::ScriptDialogType;

glib_wrapper! {
    pub struct ScriptDialog(Boxed<ffi::WebKitScriptDialog>);

    match fn {
        copy => |ptr| webkit_script_dialog_copy(ptr),
        free => |ptr| webkit_script_dialog_free(ptr),
    }
}

impl ScriptDialog {
    pub fn confirm_set_confirmed(&mut self, confirmed: bool) {
        unsafe { ffi::webkit_script_dialog_confirm_set_confirmed(self.to_glib_none_mut().0, confirmed.to_glib()); }
    }

    pub fn get_dialog_type(&mut self) -> ScriptDialogType {
        unsafe { from_glib(ffi::webkit_script_dialog_get_dialog_type(self.to_glib_none_mut().0)) }
    }

    pub fn get_message(&mut self) -> &str {
        let c_str = unsafe { ffi::webkit_script_dialog_get_message(self.to_glib_none_mut().0) };
        let c_str = unsafe { CStr::from_ptr(c_str) };
        c_str.to_str().unwrap()
    }

    pub fn prompt_get_default_text(&mut self) -> &str {
        let c_str = unsafe { ffi::webkit_script_dialog_prompt_get_default_text(self.to_glib_none_mut().0) };
        let c_str = unsafe { CStr::from_ptr(c_str) };
        c_str.to_str().unwrap()
    }

    pub fn prompt_set_text(&mut self, text: &str) {
        let c_str = CString::new(text).unwrap();
        unsafe { ffi::webkit_script_dialog_prompt_set_text(self.to_glib_none_mut().0, c_str.as_ptr()) };
    }
}

fn webkit_script_dialog_copy(dialog: *const ffi::WebKitScriptDialog) -> *mut ffi::WebKitScriptDialog {
    dialog as *mut _
}

fn webkit_script_dialog_free(_dialog: *const ffi::WebKitScriptDialog) {
}
