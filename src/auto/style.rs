// This file was generated by gir (b010d34) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use glib;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;
#[cfg(feature = "v3_22")]
use gtk;
use pango;
use std::mem::transmute;

glib_wrapper! {
    pub struct Style(Object<ffi::GtkSourceStyle>);

    match fn {
        get_type => || ffi::gtk_source_style_get_type(),
    }
}

pub trait StyleExt {
    #[cfg(feature = "v3_22")]
    fn apply<P: IsA<gtk::TextTag>>(&self, tag: &P);

    fn copy(&self) -> Option<Style>;

    fn get_property_background(&self) -> Option<String>;

    fn get_property_background_set(&self) -> bool;

    fn get_property_bold(&self) -> bool;

    fn get_property_bold_set(&self) -> bool;

    fn get_property_foreground(&self) -> Option<String>;

    fn get_property_foreground_set(&self) -> bool;

    fn get_property_italic(&self) -> bool;

    fn get_property_italic_set(&self) -> bool;

    fn get_property_line_background(&self) -> Option<String>;

    fn get_property_line_background_set(&self) -> bool;

    fn get_property_pango_underline(&self) -> pango::Underline;

    fn get_property_scale(&self) -> Option<String>;

    fn get_property_scale_set(&self) -> bool;

    fn get_property_strikethrough(&self) -> bool;

    fn get_property_strikethrough_set(&self) -> bool;

    fn get_property_underline(&self) -> bool;

    fn get_property_underline_color(&self) -> Option<String>;

    fn get_property_underline_color_set(&self) -> bool;

    fn get_property_underline_set(&self) -> bool;
}

impl<O: IsA<Style> + IsA<glib::object::Object>> StyleExt for O {
    #[cfg(feature = "v3_22")]
    fn apply<P: IsA<gtk::TextTag>>(&self, tag: &P) {
        unsafe {
            ffi::gtk_source_style_apply(self.to_glib_none().0, tag.to_glib_none().0);
        }
    }

    fn copy(&self) -> Option<Style> {
        unsafe {
            from_glib_full(ffi::gtk_source_style_copy(self.to_glib_none().0))
        }
    }

    fn get_property_background(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "background".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn get_property_background_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "background-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_bold(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "bold".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_bold_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "bold-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_foreground(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "foreground".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn get_property_foreground_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "foreground-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_italic(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "italic".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_italic_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "italic-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_line_background(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "line-background".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn get_property_line_background_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "line-background-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_pango_underline(&self) -> pango::Underline {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "pango-underline".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn get_property_scale(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "scale".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn get_property_scale_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "scale-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_strikethrough(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "strikethrough".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_strikethrough_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "strikethrough-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_underline(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "underline".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_underline_color(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "underline-color".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn get_property_underline_color_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "underline-color-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_underline_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "underline-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }
}
