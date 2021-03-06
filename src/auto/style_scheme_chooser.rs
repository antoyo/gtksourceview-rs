// This file was generated by gir (b010d34) from gir-files (71d73f0)
// DO NOT EDIT

#[cfg(feature = "v3_16")]
use StyleScheme;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct StyleSchemeChooser(Object<ffi::GtkSourceStyleSchemeChooser>);

    match fn {
        get_type => || ffi::gtk_source_style_scheme_chooser_get_type(),
    }
}

pub trait StyleSchemeChooserExt {
    #[cfg(feature = "v3_16")]
    fn get_style_scheme(&self) -> Option<StyleScheme>;

    #[cfg(feature = "v3_16")]
    fn set_style_scheme(&self, scheme: &StyleScheme);
}

impl<O: IsA<StyleSchemeChooser>> StyleSchemeChooserExt for O {
    #[cfg(feature = "v3_16")]
    fn get_style_scheme(&self) -> Option<StyleScheme> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_chooser_get_style_scheme(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    fn set_style_scheme(&self, scheme: &StyleScheme) {
        unsafe {
            ffi::gtk_source_style_scheme_chooser_set_style_scheme(self.to_glib_none().0, scheme.to_glib_none().0);
        }
    }
}
