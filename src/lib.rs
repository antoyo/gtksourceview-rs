extern crate atk_sys as atk_ffi;
#[macro_use]
extern crate bitflags;
extern crate cairo;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate gdk_sys as gdk_ffi;
extern crate gio;
extern crate gio_sys as gio_ffi;
#[macro_use]
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gtk;
extern crate gtk_sys as gtk_ffi;
extern crate gtksourceview_sys as ffi;
extern crate libc;
extern crate pango;

/// Asserts that this is the main thread and `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => (
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            }
            else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    )
}

macro_rules! callback_guard {
    () => (
        let _guard = ::glib::CallbackGuard::new();
        if cfg!(debug_assertions) {
            assert_initialized_main_thread!();
        }
    )
}

/// No-op.
macro_rules! skip_assert_initialized {
    () => ()
}

mod auto;

pub use auto::*;
pub use glib::Error;
