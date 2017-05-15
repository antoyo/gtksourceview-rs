// This file was generated by gir (8343e00) from gir-files (71d73f0)
// DO NOT EDIT

use GutterRenderer;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;
use std::mem;

glib_wrapper! {
    pub struct GutterRendererText(Object<ffi::GtkSourceGutterRendererText>): GutterRenderer;

    match fn {
        get_type => || ffi::gtk_source_gutter_renderer_text_get_type(),
    }
}

impl GutterRendererText {
    pub fn new() -> GutterRendererText {
        unsafe {
            GutterRenderer::from_glib_full(ffi::gtk_source_gutter_renderer_text_new()).downcast_unchecked()
        }
    }
}

pub trait GutterRendererTextExt {
    fn measure(&self, text: &str) -> (i32, i32);

    fn measure_markup(&self, markup: &str) -> (i32, i32);

    fn set_markup(&self, markup: &str, length: i32);

    fn set_text(&self, text: &str, length: i32);

    fn get_property_markup(&self) -> Option<String>;

    fn get_property_text(&self) -> Option<String>;
}

impl<O: IsA<GutterRendererText> + IsA<glib::object::Object>> GutterRendererTextExt for O {
    fn measure(&self, text: &str) -> (i32, i32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gtk_source_gutter_renderer_text_measure(self.to_glib_none().0, text.to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    fn measure_markup(&self, markup: &str) -> (i32, i32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gtk_source_gutter_renderer_text_measure_markup(self.to_glib_none().0, markup.to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    fn set_markup(&self, markup: &str, length: i32) {
        unsafe {
            ffi::gtk_source_gutter_renderer_text_set_markup(self.to_glib_none().0, markup.to_glib_none().0, length);
        }
    }

    fn set_text(&self, text: &str, length: i32) {
        unsafe {
            ffi::gtk_source_gutter_renderer_text_set_text(self.to_glib_none().0, text.to_glib_none().0, length);
        }
    }

    fn get_property_markup(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "markup".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn get_property_text(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "text".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }
}
