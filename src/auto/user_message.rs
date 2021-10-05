// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/wusyong/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitUserMessage")]
    pub struct UserMessage(Object<ffi::WebKitUserMessage, ffi::WebKitUserMessageClass>);

    match fn {
        type_ => || ffi::webkit_user_message_get_type(),
    }
}

impl UserMessage {
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    #[doc(alias = "webkit_user_message_new")]
    pub fn new(name: &str, parameters: Option<&glib::Variant>) -> UserMessage {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::webkit_user_message_new(name.to_glib_none().0, parameters.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    #[doc(alias = "webkit_user_message_new_with_fd_list")]
    #[doc(alias = "new_with_fd_list")]
    pub fn with_fd_list(name: &str, parameters: Option<&glib::Variant>, fd_list: Option<&impl IsA<gio::UnixFDList>>) -> UserMessage {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::webkit_user_message_new_with_fd_list(name.to_glib_none().0, parameters.to_glib_none().0, fd_list.map(|p| p.as_ref()).to_glib_none().0))
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`UserMessage`] objects.
            ///
            /// This method returns an instance of [`UserMessageBuilder`] which can be used to create [`UserMessage`] objects.
            pub fn builder() -> UserMessageBuilder {
                UserMessageBuilder::default()
            }
        
}

#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
impl Default for UserMessage {
                     fn default() -> Self {
                         glib::object::Object::new::<Self>(&[])
                            .expect("Can't construct UserMessage object with default parameters")
                     }
                 }

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`UserMessage`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct UserMessageBuilder {
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    fd_list: Option<gio::UnixFDList>,
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    name: Option<String>,
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    parameters: Option<glib::Variant>,
}

impl UserMessageBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`UserMessageBuilder`].
    pub fn new() -> Self {
        Self::default()
    }


    // rustdoc-stripper-ignore-next
    /// Build the [`UserMessage`].
    pub fn build(self) -> UserMessage {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v2_28", feature = "dox"))]
if let Some(ref fd_list) = self.fd_list {
                properties.push(("fd-list", fd_list));
            }
        #[cfg(any(feature = "v2_28", feature = "dox"))]
if let Some(ref name) = self.name {
                properties.push(("name", name));
            }
        #[cfg(any(feature = "v2_28", feature = "dox"))]
if let Some(ref parameters) = self.parameters {
                properties.push(("parameters", parameters));
            }
        glib::Object::new::<UserMessage>(&properties)
                .expect("Failed to create an instance of UserMessage")

    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    pub fn fd_list(mut self, fd_list: &impl IsA<gio::UnixFDList>) -> Self {
        self.fd_list = Some(fd_list.clone().upcast());
        self
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    pub fn parameters(mut self, parameters: &glib::Variant) -> Self {
        self.parameters = Some(parameters.clone());
        self
    }
}

pub const NONE_USER_MESSAGE: Option<&UserMessage> = None;

pub trait UserMessageExt: 'static {
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    #[doc(alias = "webkit_user_message_get_fd_list")]
    #[doc(alias = "get_fd_list")]
    fn fd_list(&self) -> Option<gio::UnixFDList>;

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    #[doc(alias = "webkit_user_message_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    #[doc(alias = "webkit_user_message_get_parameters")]
    #[doc(alias = "get_parameters")]
    fn parameters(&self) -> Option<glib::Variant>;

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    #[doc(alias = "webkit_user_message_send_reply")]
    fn send_reply(&self, reply: &impl IsA<UserMessage>);
}

impl<O: IsA<UserMessage>> UserMessageExt for O {
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    fn fd_list(&self) -> Option<gio::UnixFDList> {
        unsafe {
            from_glib_none(ffi::webkit_user_message_get_fd_list(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_user_message_get_name(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    fn parameters(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::webkit_user_message_get_parameters(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    fn send_reply(&self, reply: &impl IsA<UserMessage>) {
        unsafe {
            ffi::webkit_user_message_send_reply(self.as_ref().to_glib_none().0, reply.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for UserMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("UserMessage")
    }
}
