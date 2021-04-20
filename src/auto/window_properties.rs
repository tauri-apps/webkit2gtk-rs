// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    pub struct WindowProperties(Object<ffi::WebKitWindowProperties, ffi::WebKitWindowPropertiesClass>);

    match fn {
        type_ => || ffi::webkit_window_properties_get_type(),
    }
}

#[derive(Clone, Default)]
pub struct WindowPropertiesBuilder {
    fullscreen: Option<bool>,
    geometry: Option<gdk::Rectangle>,
    locationbar_visible: Option<bool>,
    menubar_visible: Option<bool>,
    resizable: Option<bool>,
    scrollbars_visible: Option<bool>,
    statusbar_visible: Option<bool>,
    toolbar_visible: Option<bool>,
}

impl WindowPropertiesBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> WindowProperties {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref fullscreen) = self.fullscreen {
            properties.push(("fullscreen", fullscreen));
        }
        if let Some(ref geometry) = self.geometry {
            properties.push(("geometry", geometry));
        }
        if let Some(ref locationbar_visible) = self.locationbar_visible {
            properties.push(("locationbar-visible", locationbar_visible));
        }
        if let Some(ref menubar_visible) = self.menubar_visible {
            properties.push(("menubar-visible", menubar_visible));
        }
        if let Some(ref resizable) = self.resizable {
            properties.push(("resizable", resizable));
        }
        if let Some(ref scrollbars_visible) = self.scrollbars_visible {
            properties.push(("scrollbars-visible", scrollbars_visible));
        }
        if let Some(ref statusbar_visible) = self.statusbar_visible {
            properties.push(("statusbar-visible", statusbar_visible));
        }
        if let Some(ref toolbar_visible) = self.toolbar_visible {
            properties.push(("toolbar-visible", toolbar_visible));
        }
        let ret = glib::Object::new::<WindowProperties>(&properties).expect("object new");
        ret
    }

    pub fn fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = Some(fullscreen);
        self
    }

    pub fn geometry(mut self, geometry: &gdk::Rectangle) -> Self {
        self.geometry = Some(geometry.clone());
        self
    }

    pub fn locationbar_visible(mut self, locationbar_visible: bool) -> Self {
        self.locationbar_visible = Some(locationbar_visible);
        self
    }

    pub fn menubar_visible(mut self, menubar_visible: bool) -> Self {
        self.menubar_visible = Some(menubar_visible);
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = Some(resizable);
        self
    }

    pub fn scrollbars_visible(mut self, scrollbars_visible: bool) -> Self {
        self.scrollbars_visible = Some(scrollbars_visible);
        self
    }

    pub fn statusbar_visible(mut self, statusbar_visible: bool) -> Self {
        self.statusbar_visible = Some(statusbar_visible);
        self
    }

    pub fn toolbar_visible(mut self, toolbar_visible: bool) -> Self {
        self.toolbar_visible = Some(toolbar_visible);
        self
    }
}

pub const NONE_WINDOW_PROPERTIES: Option<&WindowProperties> = None;

pub trait WindowPropertiesExt: 'static {
    #[doc(alias = "webkit_window_properties_get_fullscreen")]
    fn is_fullscreen(&self) -> bool;

    #[doc(alias = "webkit_window_properties_get_geometry")]
    fn geometry(&self) -> gdk::Rectangle;

    #[doc(alias = "webkit_window_properties_get_locationbar_visible")]
    fn is_locationbar_visible(&self) -> bool;

    #[doc(alias = "webkit_window_properties_get_menubar_visible")]
    fn is_menubar_visible(&self) -> bool;

    #[doc(alias = "webkit_window_properties_get_resizable")]
    fn is_resizable(&self) -> bool;

    #[doc(alias = "webkit_window_properties_get_scrollbars_visible")]
    fn is_scrollbars_visible(&self) -> bool;

    #[doc(alias = "webkit_window_properties_get_statusbar_visible")]
    fn is_statusbar_visible(&self) -> bool;

    #[doc(alias = "webkit_window_properties_get_toolbar_visible")]
    fn is_toolbar_visible(&self) -> bool;
}

impl<O: IsA<WindowProperties>> WindowPropertiesExt for O {
    fn is_fullscreen(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_fullscreen(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn geometry(&self) -> gdk::Rectangle {
        unsafe {
            let mut geometry = gdk::Rectangle::uninitialized();
            ffi::webkit_window_properties_get_geometry(
                self.as_ref().to_glib_none().0,
                geometry.to_glib_none_mut().0,
            );
            geometry
        }
    }

    fn is_locationbar_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_locationbar_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_menubar_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_menubar_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_resizable(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_resizable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_scrollbars_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_scrollbars_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_statusbar_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_statusbar_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_toolbar_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_toolbar_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for WindowProperties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WindowProperties")
    }
}
