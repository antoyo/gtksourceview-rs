// This file was generated by gir (b010d34) from gir-files (71d73f0)
// DO NOT EDIT

#[cfg(feature = "v3_16")]
use BackgroundPatternType;
use Buffer;
#[cfg(feature = "v3_16")]
use ChangeCaseType;
use Completion;
use DrawSpacesFlags;
use Gutter;
use MarkAttributes;
use SmartHomeEndType;
#[cfg(feature = "v3_24")]
use SpaceDrawer;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gtk;
use gtk_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct View(Object<ffi::GtkSourceView>): [
        gtk::TextView => gtk_ffi::GtkTextView,
        gtk::Widget => gtk_ffi::GtkWidget,
        gtk::Scrollable => gtk_ffi::GtkScrollable,
    ];

    match fn {
        get_type => || ffi::gtk_source_view_get_type(),
    }
}

impl View {
    pub fn new() -> View {
        unsafe {
            gtk::Widget::from_glib_none(ffi::gtk_source_view_new()).downcast_unchecked()
        }
    }

    pub fn new_with_buffer(buffer: &Buffer) -> View {
        unsafe {
            gtk::Widget::from_glib_none(ffi::gtk_source_view_new_with_buffer(buffer.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait ViewExt {
    fn get_auto_indent(&self) -> bool;

    #[cfg(feature = "v3_16")]
    fn get_background_pattern(&self) -> BackgroundPatternType;

    fn get_completion(&self) -> Option<Completion>;

    fn get_draw_spaces(&self) -> DrawSpacesFlags;

    fn get_gutter(&self, window_type: gtk::TextWindowType) -> Option<Gutter>;

    fn get_highlight_current_line(&self) -> bool;

    fn get_indent_on_tab(&self) -> bool;

    fn get_indent_width(&self) -> i32;

    fn get_insert_spaces_instead_of_tabs(&self) -> bool;

    fn get_right_margin_position(&self) -> u32;

    fn get_show_line_marks(&self) -> bool;

    fn get_show_line_numbers(&self) -> bool;

    fn get_show_right_margin(&self) -> bool;

    #[cfg(feature = "v3_18")]
    fn get_smart_backspace(&self) -> bool;

    fn get_smart_home_end(&self) -> SmartHomeEndType;

    #[cfg(feature = "v3_24")]
    fn get_space_drawer(&self) -> Option<SpaceDrawer>;

    fn get_tab_width(&self) -> u32;

    //fn get_visual_column(&self, iter: /*Ignored*/&gtk::TextIter) -> u32;

    //#[cfg(feature = "v3_16")]
    //fn indent_lines(&self, start: /*Ignored*/&mut gtk::TextIter, end: /*Ignored*/&mut gtk::TextIter);

    fn set_auto_indent(&self, enable: bool);

    #[cfg(feature = "v3_16")]
    fn set_background_pattern(&self, background_pattern: BackgroundPatternType);

    fn set_draw_spaces(&self, flags: DrawSpacesFlags);

    fn set_highlight_current_line(&self, highlight: bool);

    fn set_indent_on_tab(&self, enable: bool);

    fn set_indent_width(&self, width: i32);

    fn set_insert_spaces_instead_of_tabs(&self, enable: bool);

    fn set_mark_attributes(&self, category: &str, attributes: &MarkAttributes, priority: i32);

    fn set_right_margin_position(&self, pos: u32);

    fn set_show_line_marks(&self, show: bool);

    fn set_show_line_numbers(&self, show: bool);

    fn set_show_right_margin(&self, show: bool);

    #[cfg(feature = "v3_18")]
    fn set_smart_backspace(&self, smart_backspace: bool);

    fn set_smart_home_end(&self, smart_home_end: SmartHomeEndType);

    fn set_tab_width(&self, width: u32);

    //#[cfg(feature = "v3_16")]
    //fn unindent_lines(&self, start: /*Ignored*/&mut gtk::TextIter, end: /*Ignored*/&mut gtk::TextIter);

    fn get_property_show_line_marks(&self) -> bool;

    fn set_property_show_line_marks(&self, show_line_marks: bool);

    #[cfg(feature = "v3_16")]
    fn connect_change_case<F: Fn(&Self, ChangeCaseType) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_16")]
    fn connect_change_number<F: Fn(&Self, i32) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_16")]
    fn connect_join_lines<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    //fn connect_line_mark_activated<Unsupported or ignored types>(&self, f: F) -> u64;

    fn connect_move_lines<F: Fn(&Self, bool, i32) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_16")]
    fn connect_move_to_matching_bracket<F: Fn(&Self, bool) + 'static>(&self, f: F) -> u64;

    fn connect_move_words<F: Fn(&Self, i32) + 'static>(&self, f: F) -> u64;

    fn connect_redo<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_show_completion<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    //fn connect_smart_home_end<Unsupported or ignored types>(&self, f: F) -> u64;

    fn connect_undo<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<View> + IsA<glib::object::Object>> ViewExt for O {
    fn get_auto_indent(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_view_get_auto_indent(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    fn get_background_pattern(&self) -> BackgroundPatternType {
        unsafe {
            from_glib(ffi::gtk_source_view_get_background_pattern(self.to_glib_none().0))
        }
    }

    fn get_completion(&self) -> Option<Completion> {
        unsafe {
            from_glib_none(ffi::gtk_source_view_get_completion(self.to_glib_none().0))
        }
    }

    fn get_draw_spaces(&self) -> DrawSpacesFlags {
        unsafe {
            from_glib(ffi::gtk_source_view_get_draw_spaces(self.to_glib_none().0))
        }
    }

    fn get_gutter(&self, window_type: gtk::TextWindowType) -> Option<Gutter> {
        unsafe {
            from_glib_none(ffi::gtk_source_view_get_gutter(self.to_glib_none().0, window_type.to_glib()))
        }
    }

    fn get_highlight_current_line(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_view_get_highlight_current_line(self.to_glib_none().0))
        }
    }

    fn get_indent_on_tab(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_view_get_indent_on_tab(self.to_glib_none().0))
        }
    }

    fn get_indent_width(&self) -> i32 {
        unsafe {
            ffi::gtk_source_view_get_indent_width(self.to_glib_none().0)
        }
    }

    fn get_insert_spaces_instead_of_tabs(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_view_get_insert_spaces_instead_of_tabs(self.to_glib_none().0))
        }
    }

    fn get_right_margin_position(&self) -> u32 {
        unsafe {
            ffi::gtk_source_view_get_right_margin_position(self.to_glib_none().0)
        }
    }

    fn get_show_line_marks(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_view_get_show_line_marks(self.to_glib_none().0))
        }
    }

    fn get_show_line_numbers(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_view_get_show_line_numbers(self.to_glib_none().0))
        }
    }

    fn get_show_right_margin(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_view_get_show_right_margin(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_18")]
    fn get_smart_backspace(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_view_get_smart_backspace(self.to_glib_none().0))
        }
    }

    fn get_smart_home_end(&self) -> SmartHomeEndType {
        unsafe {
            from_glib(ffi::gtk_source_view_get_smart_home_end(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_24")]
    fn get_space_drawer(&self) -> Option<SpaceDrawer> {
        unsafe {
            from_glib_none(ffi::gtk_source_view_get_space_drawer(self.to_glib_none().0))
        }
    }

    fn get_tab_width(&self) -> u32 {
        unsafe {
            ffi::gtk_source_view_get_tab_width(self.to_glib_none().0)
        }
    }

    //fn get_visual_column(&self, iter: /*Ignored*/&gtk::TextIter) -> u32 {
    //    unsafe { TODO: call ffi::gtk_source_view_get_visual_column() }
    //}

    //#[cfg(feature = "v3_16")]
    //fn indent_lines(&self, start: /*Ignored*/&mut gtk::TextIter, end: /*Ignored*/&mut gtk::TextIter) {
    //    unsafe { TODO: call ffi::gtk_source_view_indent_lines() }
    //}

    fn set_auto_indent(&self, enable: bool) {
        unsafe {
            ffi::gtk_source_view_set_auto_indent(self.to_glib_none().0, enable.to_glib());
        }
    }

    #[cfg(feature = "v3_16")]
    fn set_background_pattern(&self, background_pattern: BackgroundPatternType) {
        unsafe {
            ffi::gtk_source_view_set_background_pattern(self.to_glib_none().0, background_pattern.to_glib());
        }
    }

    fn set_draw_spaces(&self, flags: DrawSpacesFlags) {
        unsafe {
            ffi::gtk_source_view_set_draw_spaces(self.to_glib_none().0, flags.to_glib());
        }
    }

    fn set_highlight_current_line(&self, highlight: bool) {
        unsafe {
            ffi::gtk_source_view_set_highlight_current_line(self.to_glib_none().0, highlight.to_glib());
        }
    }

    fn set_indent_on_tab(&self, enable: bool) {
        unsafe {
            ffi::gtk_source_view_set_indent_on_tab(self.to_glib_none().0, enable.to_glib());
        }
    }

    fn set_indent_width(&self, width: i32) {
        unsafe {
            ffi::gtk_source_view_set_indent_width(self.to_glib_none().0, width);
        }
    }

    fn set_insert_spaces_instead_of_tabs(&self, enable: bool) {
        unsafe {
            ffi::gtk_source_view_set_insert_spaces_instead_of_tabs(self.to_glib_none().0, enable.to_glib());
        }
    }

    fn set_mark_attributes(&self, category: &str, attributes: &MarkAttributes, priority: i32) {
        unsafe {
            ffi::gtk_source_view_set_mark_attributes(self.to_glib_none().0, category.to_glib_none().0, attributes.to_glib_none().0, priority);
        }
    }

    fn set_right_margin_position(&self, pos: u32) {
        unsafe {
            ffi::gtk_source_view_set_right_margin_position(self.to_glib_none().0, pos);
        }
    }

    fn set_show_line_marks(&self, show: bool) {
        unsafe {
            ffi::gtk_source_view_set_show_line_marks(self.to_glib_none().0, show.to_glib());
        }
    }

    fn set_show_line_numbers(&self, show: bool) {
        unsafe {
            ffi::gtk_source_view_set_show_line_numbers(self.to_glib_none().0, show.to_glib());
        }
    }

    fn set_show_right_margin(&self, show: bool) {
        unsafe {
            ffi::gtk_source_view_set_show_right_margin(self.to_glib_none().0, show.to_glib());
        }
    }

    #[cfg(feature = "v3_18")]
    fn set_smart_backspace(&self, smart_backspace: bool) {
        unsafe {
            ffi::gtk_source_view_set_smart_backspace(self.to_glib_none().0, smart_backspace.to_glib());
        }
    }

    fn set_smart_home_end(&self, smart_home_end: SmartHomeEndType) {
        unsafe {
            ffi::gtk_source_view_set_smart_home_end(self.to_glib_none().0, smart_home_end.to_glib());
        }
    }

    fn set_tab_width(&self, width: u32) {
        unsafe {
            ffi::gtk_source_view_set_tab_width(self.to_glib_none().0, width);
        }
    }

    //#[cfg(feature = "v3_16")]
    //fn unindent_lines(&self, start: /*Ignored*/&mut gtk::TextIter, end: /*Ignored*/&mut gtk::TextIter) {
    //    unsafe { TODO: call ffi::gtk_source_view_unindent_lines() }
    //}

    fn get_property_show_line_marks(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-line-marks".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_show_line_marks(&self, show_line_marks: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-line-marks".to_glib_none().0, Value::from(&show_line_marks).to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_16")]
    fn connect_change_case<F: Fn(&Self, ChangeCaseType) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, ChangeCaseType) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "change-case",
                transmute(change_case_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_16")]
    fn connect_change_number<F: Fn(&Self, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "change-number",
                transmute(change_number_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_16")]
    fn connect_join_lines<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "join-lines",
                transmute(join_lines_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //fn connect_line_mark_activated<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored iter: Gtk.TextIter
    //}

    fn connect_move_lines<F: Fn(&Self, bool, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, bool, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-lines",
                transmute(move_lines_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_16")]
    fn connect_move_to_matching_bracket<F: Fn(&Self, bool) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, bool) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-to-matching-bracket",
                transmute(move_to_matching_bracket_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_move_words<F: Fn(&Self, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-words",
                transmute(move_words_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_redo<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "redo",
                transmute(redo_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_show_completion<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "show-completion",
                transmute(show_completion_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //fn connect_smart_home_end<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored iter: Gtk.TextIter
    //}

    fn connect_undo<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "undo",
                transmute(undo_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_16")]
unsafe extern "C" fn change_case_trampoline<P>(this: *mut ffi::GtkSourceView, case_type: ffi::GtkSourceChangeCaseType, f: glib_ffi::gpointer)
where P: IsA<View> {
    callback_guard!();
    let f: &Box_<Fn(&P, ChangeCaseType) + 'static> = transmute(f);
    f(&View::from_glib_none(this).downcast_unchecked(), from_glib(case_type))
}

#[cfg(feature = "v3_16")]
unsafe extern "C" fn change_number_trampoline<P>(this: *mut ffi::GtkSourceView, count: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<View> {
    callback_guard!();
    let f: &Box_<Fn(&P, i32) + 'static> = transmute(f);
    f(&View::from_glib_none(this).downcast_unchecked(), count)
}

#[cfg(feature = "v3_16")]
unsafe extern "C" fn join_lines_trampoline<P>(this: *mut ffi::GtkSourceView, f: glib_ffi::gpointer)
where P: IsA<View> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&View::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn move_lines_trampoline<P>(this: *mut ffi::GtkSourceView, copy: glib_ffi::gboolean, count: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<View> {
    callback_guard!();
    let f: &Box_<Fn(&P, bool, i32) + 'static> = transmute(f);
    f(&View::from_glib_none(this).downcast_unchecked(), from_glib(copy), count)
}

#[cfg(feature = "v3_16")]
unsafe extern "C" fn move_to_matching_bracket_trampoline<P>(this: *mut ffi::GtkSourceView, extend_selection: glib_ffi::gboolean, f: glib_ffi::gpointer)
where P: IsA<View> {
    callback_guard!();
    let f: &Box_<Fn(&P, bool) + 'static> = transmute(f);
    f(&View::from_glib_none(this).downcast_unchecked(), from_glib(extend_selection))
}

unsafe extern "C" fn move_words_trampoline<P>(this: *mut ffi::GtkSourceView, count: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<View> {
    callback_guard!();
    let f: &Box_<Fn(&P, i32) + 'static> = transmute(f);
    f(&View::from_glib_none(this).downcast_unchecked(), count)
}

unsafe extern "C" fn redo_trampoline<P>(this: *mut ffi::GtkSourceView, f: glib_ffi::gpointer)
where P: IsA<View> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&View::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn show_completion_trampoline<P>(this: *mut ffi::GtkSourceView, f: glib_ffi::gpointer)
where P: IsA<View> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&View::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn undo_trampoline<P>(this: *mut ffi::GtkSourceView, f: glib_ffi::gpointer)
where P: IsA<View> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&View::from_glib_none(this).downcast_unchecked())
}
