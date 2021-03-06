// This file was generated by gir (b010d34) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use glib;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    pub struct StyleScheme(Object<ffi::GtkSourceStyleScheme>);

    match fn {
        get_type => || ffi::gtk_source_style_scheme_get_type(),
    }
}

pub trait StyleSchemeExt {
    fn get_authors(&self) -> Vec<String>;

    fn get_description(&self) -> Option<String>;

    fn get_filename(&self) -> Option<String>;

    fn get_id(&self) -> Option<String>;

    fn get_name(&self) -> Option<String>;

    //fn get_style(&self, style_id: &str) -> /*Ignored*/Option<Style>;

    fn get_property_description(&self) -> Option<String>;

    fn get_property_filename(&self) -> Option<String>;

    fn get_property_id(&self) -> Option<String>;

    fn get_property_name(&self) -> Option<String>;
}

impl<O: IsA<StyleScheme> + IsA<glib::object::Object>> StyleSchemeExt for O {
    fn get_authors(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_source_style_scheme_get_authors(self.to_glib_none().0))
        }
    }

    fn get_description(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_description(self.to_glib_none().0))
        }
    }

    fn get_filename(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_filename(self.to_glib_none().0))
        }
    }

    fn get_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_id(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_name(self.to_glib_none().0))
        }
    }

    //fn get_style(&self, style_id: &str) -> /*Ignored*/Option<Style> {
    //    unsafe { TODO: call ffi::gtk_source_style_scheme_get_style() }
    //}

    fn get_property_description(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "description".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn get_property_filename(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "filename".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn get_property_id(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "id".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn get_property_name(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "name".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }
}
