// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use crate::CredentialPersistence;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Credential(Boxed<ffi::WebKitCredential>);

    match fn {
        copy => |ptr| ffi::webkit_credential_copy(mut_override(ptr)),
        free => |ptr| ffi::webkit_credential_free(ptr),
        type_ => || ffi::webkit_credential_get_type(),
    }
}

impl Credential {
    #[doc(alias = "webkit_credential_new")]
    pub fn new(username: &str, password: &str, persistence: CredentialPersistence) -> Credential {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_credential_new(
                username.to_glib_none().0,
                password.to_glib_none().0,
                persistence.to_glib(),
            ))
        }
    }

    #[doc(alias = "webkit_credential_get_password")]
    pub fn password(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_credential_get_password(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "webkit_credential_get_persistence")]
    pub fn persistence(&mut self) -> CredentialPersistence {
        unsafe {
            from_glib(ffi::webkit_credential_get_persistence(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "webkit_credential_get_username")]
    pub fn username(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_credential_get_username(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "webkit_credential_has_password")]
    pub fn has_password(&mut self) -> bool {
        unsafe {
            from_glib(ffi::webkit_credential_has_password(
                self.to_glib_none_mut().0,
            ))
        }
    }
}
