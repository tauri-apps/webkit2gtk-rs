// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;

use glib::translate::{from_glib, mut_override, Stash, ToGlib, ToGlibPtr};
use glib_sys;
use webkit2_sys;

use super::ScriptDialogType;

glib_wrapper! {
    pub struct ScriptDialog(Boxed<webkit2_sys::WebKitScriptDialog>);

    match fn {
        copy => |ptr| webkit_script_dialog_copy(ptr),
        free => |ptr| webkit_script_dialog_free(ptr),
    }
}

impl ScriptDialog {
    pub fn confirm_set_confirmed(&self, confirmed: bool) {
        unsafe {
            webkit2_sys::webkit_script_dialog_confirm_set_confirmed(
                mut_override(self.to_glib_none().0),
                confirmed.to_glib(),
            );
        }
    }

    pub fn get_dialog_type(&self) -> ScriptDialogType {
        unsafe {
            from_glib(webkit2_sys::webkit_script_dialog_get_dialog_type(
                mut_override(self.to_glib_none().0),
            ))
        }
    }

    pub fn get_message(&self) -> &str {
        let c_str = unsafe {
            webkit2_sys::webkit_script_dialog_get_message(mut_override(self.to_glib_none().0))
        };
        let c_str = unsafe { CStr::from_ptr(c_str) };
        c_str.to_str().unwrap()
    }

    pub fn prompt_get_default_text(&self) -> &str {
        let c_str = unsafe {
            webkit2_sys::webkit_script_dialog_prompt_get_default_text(mut_override(
                self.to_glib_none().0,
            ))
        };
        let c_str = unsafe { CStr::from_ptr(c_str) };
        c_str.to_str().unwrap()
    }

    pub fn prompt_set_text(&self, text: &str) {
        let c_str = CString::new(text).unwrap();
        unsafe {
            webkit2_sys::webkit_script_dialog_prompt_set_text(
                mut_override(self.to_glib_none().0),
                c_str.as_ptr(),
            )
        };
    }
}

fn webkit_script_dialog_copy(
    dialog: *const webkit2_sys::WebKitScriptDialog,
) -> *mut webkit2_sys::WebKitScriptDialog {
    skip_assert_initialized!();
    dialog as *mut _
}

fn webkit_script_dialog_free(_dialog: *const webkit2_sys::WebKitScriptDialog) {
    skip_assert_initialized!();
}
