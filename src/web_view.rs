// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "v2_6")]
use crate::UserContentManager;
#[cfg(feature = "v2_6")]
use crate::WebContext;
use crate::WebView;

use std::error::Error;
use std::ffi::CString;
use std::ptr;

use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::{from_glib_full, FromGlibPtrNone, IntoGlib, ToGlibPtr};
use glib::StaticType;
use gobject_sys;
use gtk;
use libc::c_void;

impl WebView {
    #[cfg(feature = "v2_6")]
    pub fn new_with_context_and_user_content_manager<
        P: IsA<WebContext>,
        Q: IsA<UserContentManager>,
    >(
        context: &P,
        user_content_manager: &Q,
    ) -> Self {
        assert_initialized_main_thread!();
        let user_content_manager_property = CString::new("user-content-manager").unwrap();
        let web_context_property = CString::new("web-context").unwrap();
        let glib_user_content_manager: *mut ffi::WebKitUserContentManager =
            user_content_manager.as_ref().to_glib_none().0;
        let glib_context: *mut ffi::WebKitWebContext = context.as_ref().to_glib_none().0;
        let null: *mut gobject_sys::GObject = ptr::null_mut();
        unsafe {
            gtk::Widget::from_glib_none(gobject_sys::g_object_new(
                WebView::static_type().into_glib(),
                user_content_manager_property.as_ptr(),
                glib_user_content_manager,
                web_context_property.as_ptr(),
                glib_context,
                null,
            ) as *mut _)
            .unsafe_cast()
        }
    }
}
