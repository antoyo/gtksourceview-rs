// This file was generated by gir (b010d34) from gir-files (71d73f0)
// DO NOT EDIT

use Buffer;
use View;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use gtk;

glib_wrapper! {
    pub struct PrintCompositor(Object<ffi::GtkSourcePrintCompositor>);

    match fn {
        get_type => || ffi::gtk_source_print_compositor_get_type(),
    }
}

impl PrintCompositor {
    pub fn new(buffer: &Buffer) -> PrintCompositor {
        unsafe {
            from_glib_full(ffi::gtk_source_print_compositor_new(buffer.to_glib_none().0))
        }
    }

    pub fn new_from_view<P: IsA<View>>(view: &P) -> PrintCompositor {
        unsafe {
            from_glib_full(ffi::gtk_source_print_compositor_new_from_view(view.to_glib_none().0))
        }
    }
}

pub trait PrintCompositorExt {
    //fn draw_page(&self, context: /*Ignored*/&gtk::PrintContext, page_nr: i32);

    fn get_body_font_name(&self) -> Option<String>;

    fn get_bottom_margin(&self, unit: gtk::Unit) -> f64;

    fn get_buffer(&self) -> Option<Buffer>;

    fn get_footer_font_name(&self) -> Option<String>;

    fn get_header_font_name(&self) -> Option<String>;

    fn get_highlight_syntax(&self) -> bool;

    fn get_left_margin(&self, unit: gtk::Unit) -> f64;

    fn get_line_numbers_font_name(&self) -> Option<String>;

    fn get_n_pages(&self) -> i32;

    fn get_pagination_progress(&self) -> f64;

    fn get_print_footer(&self) -> bool;

    fn get_print_header(&self) -> bool;

    fn get_print_line_numbers(&self) -> u32;

    fn get_right_margin(&self, unit: gtk::Unit) -> f64;

    fn get_tab_width(&self) -> u32;

    fn get_top_margin(&self, unit: gtk::Unit) -> f64;

    fn get_wrap_mode(&self) -> gtk::WrapMode;

    //fn paginate(&self, context: /*Ignored*/&gtk::PrintContext) -> bool;

    fn set_body_font_name(&self, font_name: &str);

    fn set_bottom_margin(&self, margin: f64, unit: gtk::Unit);

    fn set_footer_font_name<'a, P: Into<Option<&'a str>>>(&self, font_name: P);

    fn set_footer_format<'a, 'b, 'c, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>, R: Into<Option<&'c str>>>(&self, separator: bool, left: P, center: Q, right: R);

    fn set_header_font_name<'a, P: Into<Option<&'a str>>>(&self, font_name: P);

    fn set_header_format<'a, 'b, 'c, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>, R: Into<Option<&'c str>>>(&self, separator: bool, left: P, center: Q, right: R);

    fn set_highlight_syntax(&self, highlight: bool);

    fn set_left_margin(&self, margin: f64, unit: gtk::Unit);

    fn set_line_numbers_font_name<'a, P: Into<Option<&'a str>>>(&self, font_name: P);

    fn set_print_footer(&self, print: bool);

    fn set_print_header(&self, print: bool);

    fn set_print_line_numbers(&self, interval: u32);

    fn set_right_margin(&self, margin: f64, unit: gtk::Unit);

    fn set_tab_width(&self, width: u32);

    fn set_top_margin(&self, margin: f64, unit: gtk::Unit);

    fn set_wrap_mode(&self, wrap_mode: gtk::WrapMode);
}

impl<O: IsA<PrintCompositor>> PrintCompositorExt for O {
    //fn draw_page(&self, context: /*Ignored*/&gtk::PrintContext, page_nr: i32) {
    //    unsafe { TODO: call ffi::gtk_source_print_compositor_draw_page() }
    //}

    fn get_body_font_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_source_print_compositor_get_body_font_name(self.to_glib_none().0))
        }
    }

    fn get_bottom_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe {
            ffi::gtk_source_print_compositor_get_bottom_margin(self.to_glib_none().0, unit.to_glib())
        }
    }

    fn get_buffer(&self) -> Option<Buffer> {
        unsafe {
            from_glib_none(ffi::gtk_source_print_compositor_get_buffer(self.to_glib_none().0))
        }
    }

    fn get_footer_font_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_source_print_compositor_get_footer_font_name(self.to_glib_none().0))
        }
    }

    fn get_header_font_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_source_print_compositor_get_header_font_name(self.to_glib_none().0))
        }
    }

    fn get_highlight_syntax(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_print_compositor_get_highlight_syntax(self.to_glib_none().0))
        }
    }

    fn get_left_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe {
            ffi::gtk_source_print_compositor_get_left_margin(self.to_glib_none().0, unit.to_glib())
        }
    }

    fn get_line_numbers_font_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_source_print_compositor_get_line_numbers_font_name(self.to_glib_none().0))
        }
    }

    fn get_n_pages(&self) -> i32 {
        unsafe {
            ffi::gtk_source_print_compositor_get_n_pages(self.to_glib_none().0)
        }
    }

    fn get_pagination_progress(&self) -> f64 {
        unsafe {
            ffi::gtk_source_print_compositor_get_pagination_progress(self.to_glib_none().0)
        }
    }

    fn get_print_footer(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_print_compositor_get_print_footer(self.to_glib_none().0))
        }
    }

    fn get_print_header(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_print_compositor_get_print_header(self.to_glib_none().0))
        }
    }

    fn get_print_line_numbers(&self) -> u32 {
        unsafe {
            ffi::gtk_source_print_compositor_get_print_line_numbers(self.to_glib_none().0)
        }
    }

    fn get_right_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe {
            ffi::gtk_source_print_compositor_get_right_margin(self.to_glib_none().0, unit.to_glib())
        }
    }

    fn get_tab_width(&self) -> u32 {
        unsafe {
            ffi::gtk_source_print_compositor_get_tab_width(self.to_glib_none().0)
        }
    }

    fn get_top_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe {
            ffi::gtk_source_print_compositor_get_top_margin(self.to_glib_none().0, unit.to_glib())
        }
    }

    fn get_wrap_mode(&self) -> gtk::WrapMode {
        unsafe {
            from_glib(ffi::gtk_source_print_compositor_get_wrap_mode(self.to_glib_none().0))
        }
    }

    //fn paginate(&self, context: /*Ignored*/&gtk::PrintContext) -> bool {
    //    unsafe { TODO: call ffi::gtk_source_print_compositor_paginate() }
    //}

    fn set_body_font_name(&self, font_name: &str) {
        unsafe {
            ffi::gtk_source_print_compositor_set_body_font_name(self.to_glib_none().0, font_name.to_glib_none().0);
        }
    }

    fn set_bottom_margin(&self, margin: f64, unit: gtk::Unit) {
        unsafe {
            ffi::gtk_source_print_compositor_set_bottom_margin(self.to_glib_none().0, margin, unit.to_glib());
        }
    }

    fn set_footer_font_name<'a, P: Into<Option<&'a str>>>(&self, font_name: P) {
        let font_name = font_name.into();
        let font_name = font_name.to_glib_none();
        unsafe {
            ffi::gtk_source_print_compositor_set_footer_font_name(self.to_glib_none().0, font_name.0);
        }
    }

    fn set_footer_format<'a, 'b, 'c, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>, R: Into<Option<&'c str>>>(&self, separator: bool, left: P, center: Q, right: R) {
        let left = left.into();
        let left = left.to_glib_none();
        let center = center.into();
        let center = center.to_glib_none();
        let right = right.into();
        let right = right.to_glib_none();
        unsafe {
            ffi::gtk_source_print_compositor_set_footer_format(self.to_glib_none().0, separator.to_glib(), left.0, center.0, right.0);
        }
    }

    fn set_header_font_name<'a, P: Into<Option<&'a str>>>(&self, font_name: P) {
        let font_name = font_name.into();
        let font_name = font_name.to_glib_none();
        unsafe {
            ffi::gtk_source_print_compositor_set_header_font_name(self.to_glib_none().0, font_name.0);
        }
    }

    fn set_header_format<'a, 'b, 'c, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>, R: Into<Option<&'c str>>>(&self, separator: bool, left: P, center: Q, right: R) {
        let left = left.into();
        let left = left.to_glib_none();
        let center = center.into();
        let center = center.to_glib_none();
        let right = right.into();
        let right = right.to_glib_none();
        unsafe {
            ffi::gtk_source_print_compositor_set_header_format(self.to_glib_none().0, separator.to_glib(), left.0, center.0, right.0);
        }
    }

    fn set_highlight_syntax(&self, highlight: bool) {
        unsafe {
            ffi::gtk_source_print_compositor_set_highlight_syntax(self.to_glib_none().0, highlight.to_glib());
        }
    }

    fn set_left_margin(&self, margin: f64, unit: gtk::Unit) {
        unsafe {
            ffi::gtk_source_print_compositor_set_left_margin(self.to_glib_none().0, margin, unit.to_glib());
        }
    }

    fn set_line_numbers_font_name<'a, P: Into<Option<&'a str>>>(&self, font_name: P) {
        let font_name = font_name.into();
        let font_name = font_name.to_glib_none();
        unsafe {
            ffi::gtk_source_print_compositor_set_line_numbers_font_name(self.to_glib_none().0, font_name.0);
        }
    }

    fn set_print_footer(&self, print: bool) {
        unsafe {
            ffi::gtk_source_print_compositor_set_print_footer(self.to_glib_none().0, print.to_glib());
        }
    }

    fn set_print_header(&self, print: bool) {
        unsafe {
            ffi::gtk_source_print_compositor_set_print_header(self.to_glib_none().0, print.to_glib());
        }
    }

    fn set_print_line_numbers(&self, interval: u32) {
        unsafe {
            ffi::gtk_source_print_compositor_set_print_line_numbers(self.to_glib_none().0, interval);
        }
    }

    fn set_right_margin(&self, margin: f64, unit: gtk::Unit) {
        unsafe {
            ffi::gtk_source_print_compositor_set_right_margin(self.to_glib_none().0, margin, unit.to_glib());
        }
    }

    fn set_tab_width(&self, width: u32) {
        unsafe {
            ffi::gtk_source_print_compositor_set_tab_width(self.to_glib_none().0, width);
        }
    }

    fn set_top_margin(&self, margin: f64, unit: gtk::Unit) {
        unsafe {
            ffi::gtk_source_print_compositor_set_top_margin(self.to_glib_none().0, margin, unit.to_glib());
        }
    }

    fn set_wrap_mode(&self, wrap_mode: gtk::WrapMode) {
        unsafe {
            ffi::gtk_source_print_compositor_set_wrap_mode(self.to_glib_none().0, wrap_mode.to_glib());
        }
    }
}
