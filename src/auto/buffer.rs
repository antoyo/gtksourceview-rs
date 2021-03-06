// This file was generated by gir (b010d34) from gir-files (71d73f0)
// DO NOT EDIT

use Language;
use Mark;
use StyleScheme;
use UndoManager;
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
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct Buffer(Object<ffi::GtkSourceBuffer>): [
        gtk::TextBuffer => gtk_ffi::GtkTextBuffer,
    ];

    match fn {
        get_type => || ffi::gtk_source_buffer_get_type(),
    }
}

impl Buffer {
    pub fn new<'a, P: Into<Option<&'a gtk::TextTagTable>>>(table: P) -> Buffer {
        let table = table.into();
        let table = table.to_glib_none();
        unsafe {
            from_glib_full(ffi::gtk_source_buffer_new(table.0))
        }
    }

    pub fn new_with_language(language: &Language) -> Buffer {
        unsafe {
            from_glib_full(ffi::gtk_source_buffer_new_with_language(language.to_glib_none().0))
        }
    }
}

pub trait BufferExt {
    //fn backward_iter_to_source_mark<'a, P: Into<Option<&'a str>>>(&self, iter: /*Ignored*/&mut gtk::TextIter, category: P) -> bool;

    fn begin_not_undoable_action(&self);

    fn can_redo(&self) -> bool;

    fn can_undo(&self) -> bool;

    //#[cfg(feature = "v3_12")]
    //fn change_case(&self, case_type: ChangeCaseType, start: /*Ignored*/&mut gtk::TextIter, end: /*Ignored*/&mut gtk::TextIter);

    //fn create_source_mark<'a, P: Into<Option<&'a str>>>(&self, name: P, category: &str, where_: /*Ignored*/&gtk::TextIter) -> Option<Mark>;

    //#[cfg(feature = "v3_20")]
    //fn create_source_tag<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, tag_name: P, first_property_name: Q, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<gtk::TextTag>;

    fn end_not_undoable_action(&self);

    //fn ensure_highlight(&self, start: /*Ignored*/&gtk::TextIter, end: /*Ignored*/&gtk::TextIter);

    //fn forward_iter_to_source_mark<'a, P: Into<Option<&'a str>>>(&self, iter: /*Ignored*/&mut gtk::TextIter, category: P) -> bool;

    //fn get_context_classes_at_iter(&self, iter: /*Ignored*/&gtk::TextIter) -> Vec<String>;

    fn get_highlight_matching_brackets(&self) -> bool;

    fn get_highlight_syntax(&self) -> bool;

    #[cfg(feature = "v3_14")]
    fn get_implicit_trailing_newline(&self) -> bool;

    fn get_language(&self) -> Option<Language>;

    fn get_max_undo_levels(&self) -> i32;

    //fn get_source_marks_at_iter<'a, P: Into<Option<&'a str>>>(&self, iter: /*Ignored*/&mut gtk::TextIter, category: P) -> Vec<Mark>;

    fn get_source_marks_at_line<'a, P: Into<Option<&'a str>>>(&self, line: i32, category: P) -> Vec<Mark>;

    fn get_style_scheme(&self) -> Option<StyleScheme>;

    fn get_undo_manager(&self) -> Option<UndoManager>;

    //fn iter_backward_to_context_class_toggle(&self, iter: /*Ignored*/&mut gtk::TextIter, context_class: &str) -> bool;

    //fn iter_forward_to_context_class_toggle(&self, iter: /*Ignored*/&mut gtk::TextIter, context_class: &str) -> bool;

    //fn iter_has_context_class(&self, iter: /*Ignored*/&gtk::TextIter, context_class: &str) -> bool;

    //#[cfg(feature = "v3_16")]
    //fn join_lines(&self, start: /*Ignored*/&mut gtk::TextIter, end: /*Ignored*/&mut gtk::TextIter);

    fn redo(&self);

    //fn remove_source_marks<'a, P: Into<Option<&'a str>>>(&self, start: /*Ignored*/&gtk::TextIter, end: /*Ignored*/&gtk::TextIter, category: P);

    fn set_highlight_matching_brackets(&self, highlight: bool);

    fn set_highlight_syntax(&self, highlight: bool);

    #[cfg(feature = "v3_14")]
    fn set_implicit_trailing_newline(&self, implicit_trailing_newline: bool);

    fn set_language<'a, P: Into<Option<&'a Language>>>(&self, language: P);

    fn set_max_undo_levels(&self, max_undo_levels: i32);

    fn set_style_scheme<'a, P: Into<Option<&'a StyleScheme>>>(&self, scheme: P);

    fn set_undo_manager<'a, P: IsA<UndoManager> + 'a, Q: Into<Option<&'a P>>>(&self, manager: Q);

    //#[cfg(feature = "v3_18")]
    //fn sort_lines(&self, start: /*Ignored*/&mut gtk::TextIter, end: /*Ignored*/&mut gtk::TextIter, flags: SortFlags, column: i32);

    fn undo(&self);

    fn get_property_can_redo(&self) -> bool;

    fn get_property_can_undo(&self) -> bool;

    //fn connect_bracket_matched<Unsupported or ignored types>(&self, f: F) -> u64;

    //fn connect_highlight_updated<Unsupported or ignored types>(&self, f: F) -> u64;

    fn connect_redo<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_source_mark_updated<F: Fn(&Self, &gtk::TextMark) + 'static>(&self, f: F) -> u64;

    fn connect_undo<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Buffer> + IsA<glib::object::Object>> BufferExt for O {
    //fn backward_iter_to_source_mark<'a, P: Into<Option<&'a str>>>(&self, iter: /*Ignored*/&mut gtk::TextIter, category: P) -> bool {
    //    unsafe { TODO: call ffi::gtk_source_buffer_backward_iter_to_source_mark() }
    //}

    fn begin_not_undoable_action(&self) {
        unsafe {
            ffi::gtk_source_buffer_begin_not_undoable_action(self.to_glib_none().0);
        }
    }

    fn can_redo(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_buffer_can_redo(self.to_glib_none().0))
        }
    }

    fn can_undo(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_buffer_can_undo(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v3_12")]
    //fn change_case(&self, case_type: ChangeCaseType, start: /*Ignored*/&mut gtk::TextIter, end: /*Ignored*/&mut gtk::TextIter) {
    //    unsafe { TODO: call ffi::gtk_source_buffer_change_case() }
    //}

    //fn create_source_mark<'a, P: Into<Option<&'a str>>>(&self, name: P, category: &str, where_: /*Ignored*/&gtk::TextIter) -> Option<Mark> {
    //    unsafe { TODO: call ffi::gtk_source_buffer_create_source_mark() }
    //}

    //#[cfg(feature = "v3_20")]
    //fn create_source_tag<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, tag_name: P, first_property_name: Q, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<gtk::TextTag> {
    //    unsafe { TODO: call ffi::gtk_source_buffer_create_source_tag() }
    //}

    fn end_not_undoable_action(&self) {
        unsafe {
            ffi::gtk_source_buffer_end_not_undoable_action(self.to_glib_none().0);
        }
    }

    //fn ensure_highlight(&self, start: /*Ignored*/&gtk::TextIter, end: /*Ignored*/&gtk::TextIter) {
    //    unsafe { TODO: call ffi::gtk_source_buffer_ensure_highlight() }
    //}

    //fn forward_iter_to_source_mark<'a, P: Into<Option<&'a str>>>(&self, iter: /*Ignored*/&mut gtk::TextIter, category: P) -> bool {
    //    unsafe { TODO: call ffi::gtk_source_buffer_forward_iter_to_source_mark() }
    //}

    //fn get_context_classes_at_iter(&self, iter: /*Ignored*/&gtk::TextIter) -> Vec<String> {
    //    unsafe { TODO: call ffi::gtk_source_buffer_get_context_classes_at_iter() }
    //}

    fn get_highlight_matching_brackets(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_buffer_get_highlight_matching_brackets(self.to_glib_none().0))
        }
    }

    fn get_highlight_syntax(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_buffer_get_highlight_syntax(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_implicit_trailing_newline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_buffer_get_implicit_trailing_newline(self.to_glib_none().0))
        }
    }

    fn get_language(&self) -> Option<Language> {
        unsafe {
            from_glib_none(ffi::gtk_source_buffer_get_language(self.to_glib_none().0))
        }
    }

    fn get_max_undo_levels(&self) -> i32 {
        unsafe {
            ffi::gtk_source_buffer_get_max_undo_levels(self.to_glib_none().0)
        }
    }

    //fn get_source_marks_at_iter<'a, P: Into<Option<&'a str>>>(&self, iter: /*Ignored*/&mut gtk::TextIter, category: P) -> Vec<Mark> {
    //    unsafe { TODO: call ffi::gtk_source_buffer_get_source_marks_at_iter() }
    //}

    fn get_source_marks_at_line<'a, P: Into<Option<&'a str>>>(&self, line: i32, category: P) -> Vec<Mark> {
        let category = category.into();
        let category = category.to_glib_none();
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_source_buffer_get_source_marks_at_line(self.to_glib_none().0, line, category.0))
        }
    }

    fn get_style_scheme(&self) -> Option<StyleScheme> {
        unsafe {
            from_glib_none(ffi::gtk_source_buffer_get_style_scheme(self.to_glib_none().0))
        }
    }

    fn get_undo_manager(&self) -> Option<UndoManager> {
        unsafe {
            from_glib_none(ffi::gtk_source_buffer_get_undo_manager(self.to_glib_none().0))
        }
    }

    //fn iter_backward_to_context_class_toggle(&self, iter: /*Ignored*/&mut gtk::TextIter, context_class: &str) -> bool {
    //    unsafe { TODO: call ffi::gtk_source_buffer_iter_backward_to_context_class_toggle() }
    //}

    //fn iter_forward_to_context_class_toggle(&self, iter: /*Ignored*/&mut gtk::TextIter, context_class: &str) -> bool {
    //    unsafe { TODO: call ffi::gtk_source_buffer_iter_forward_to_context_class_toggle() }
    //}

    //fn iter_has_context_class(&self, iter: /*Ignored*/&gtk::TextIter, context_class: &str) -> bool {
    //    unsafe { TODO: call ffi::gtk_source_buffer_iter_has_context_class() }
    //}

    //#[cfg(feature = "v3_16")]
    //fn join_lines(&self, start: /*Ignored*/&mut gtk::TextIter, end: /*Ignored*/&mut gtk::TextIter) {
    //    unsafe { TODO: call ffi::gtk_source_buffer_join_lines() }
    //}

    fn redo(&self) {
        unsafe {
            ffi::gtk_source_buffer_redo(self.to_glib_none().0);
        }
    }

    //fn remove_source_marks<'a, P: Into<Option<&'a str>>>(&self, start: /*Ignored*/&gtk::TextIter, end: /*Ignored*/&gtk::TextIter, category: P) {
    //    unsafe { TODO: call ffi::gtk_source_buffer_remove_source_marks() }
    //}

    fn set_highlight_matching_brackets(&self, highlight: bool) {
        unsafe {
            ffi::gtk_source_buffer_set_highlight_matching_brackets(self.to_glib_none().0, highlight.to_glib());
        }
    }

    fn set_highlight_syntax(&self, highlight: bool) {
        unsafe {
            ffi::gtk_source_buffer_set_highlight_syntax(self.to_glib_none().0, highlight.to_glib());
        }
    }

    #[cfg(feature = "v3_14")]
    fn set_implicit_trailing_newline(&self, implicit_trailing_newline: bool) {
        unsafe {
            ffi::gtk_source_buffer_set_implicit_trailing_newline(self.to_glib_none().0, implicit_trailing_newline.to_glib());
        }
    }

    fn set_language<'a, P: Into<Option<&'a Language>>>(&self, language: P) {
        let language = language.into();
        let language = language.to_glib_none();
        unsafe {
            ffi::gtk_source_buffer_set_language(self.to_glib_none().0, language.0);
        }
    }

    fn set_max_undo_levels(&self, max_undo_levels: i32) {
        unsafe {
            ffi::gtk_source_buffer_set_max_undo_levels(self.to_glib_none().0, max_undo_levels);
        }
    }

    fn set_style_scheme<'a, P: Into<Option<&'a StyleScheme>>>(&self, scheme: P) {
        let scheme = scheme.into();
        let scheme = scheme.to_glib_none();
        unsafe {
            ffi::gtk_source_buffer_set_style_scheme(self.to_glib_none().0, scheme.0);
        }
    }

    fn set_undo_manager<'a, P: IsA<UndoManager> + 'a, Q: Into<Option<&'a P>>>(&self, manager: Q) {
        let manager = manager.into();
        let manager = manager.to_glib_none();
        unsafe {
            ffi::gtk_source_buffer_set_undo_manager(self.to_glib_none().0, manager.0);
        }
    }

    //#[cfg(feature = "v3_18")]
    //fn sort_lines(&self, start: /*Ignored*/&mut gtk::TextIter, end: /*Ignored*/&mut gtk::TextIter, flags: SortFlags, column: i32) {
    //    unsafe { TODO: call ffi::gtk_source_buffer_sort_lines() }
    //}

    fn undo(&self) {
        unsafe {
            ffi::gtk_source_buffer_undo(self.to_glib_none().0);
        }
    }

    fn get_property_can_redo(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "can-redo".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_can_undo(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "can-undo".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    //fn connect_bracket_matched<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored iter: Gtk.TextIter
    //}

    //fn connect_highlight_updated<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored start: Gtk.TextIter
    //    Ignored end: Gtk.TextIter
    //}

    fn connect_redo<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "redo",
                transmute(redo_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_source_mark_updated<F: Fn(&Self, &gtk::TextMark) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gtk::TextMark) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "source-mark-updated",
                transmute(source_mark_updated_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_undo<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "undo",
                transmute(undo_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn redo_trampoline<P>(this: *mut ffi::GtkSourceBuffer, f: glib_ffi::gpointer)
where P: IsA<Buffer> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Buffer::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn source_mark_updated_trampoline<P>(this: *mut ffi::GtkSourceBuffer, mark: *mut gtk_ffi::GtkTextMark, f: glib_ffi::gpointer)
where P: IsA<Buffer> {
    callback_guard!();
    let f: &Box_<Fn(&P, &gtk::TextMark) + 'static> = transmute(f);
    f(&Buffer::from_glib_none(this).downcast_unchecked(), &from_glib_none(mark))
}

unsafe extern "C" fn undo_trampoline<P>(this: *mut ffi::GtkSourceBuffer, f: glib_ffi::gpointer)
where P: IsA<Buffer> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Buffer::from_glib_none(this).downcast_unchecked())
}
