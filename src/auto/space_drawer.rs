// This file was generated by gir (8343e00) from gir-files (71d73f0)
// DO NOT EDIT

#[cfg(feature = "v3_24")]
use SpaceLocationFlags;
#[cfg(feature = "v3_24")]
use SpaceTypeFlags;
use ffi;
#[cfg(feature = "v3_24")]
use gio;
#[cfg(feature = "v3_24")]
use glib;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct SpaceDrawer(Object<ffi::GtkSourceSpaceDrawer>);

    match fn {
        get_type => || ffi::gtk_source_space_drawer_get_type(),
    }
}

impl SpaceDrawer {
    #[cfg(feature = "v3_24")]
    pub fn new() -> SpaceDrawer {
        unsafe {
            from_glib_full(ffi::gtk_source_space_drawer_new())
        }
    }
}

pub trait SpaceDrawerExt {
    #[cfg(feature = "v3_24")]
    fn bind_matrix_setting(&self, settings: &gio::Settings, key: &str, flags: gio::SettingsBindFlags);

    #[cfg(feature = "v3_24")]
    fn get_enable_matrix(&self) -> bool;

    #[cfg(feature = "v3_24")]
    fn get_matrix(&self) -> Option<glib::Variant>;

    #[cfg(feature = "v3_24")]
    fn get_types_for_locations(&self, locations: SpaceLocationFlags) -> SpaceTypeFlags;

    #[cfg(feature = "v3_24")]
    fn set_enable_matrix(&self, enable_matrix: bool);

    #[cfg(feature = "v3_24")]
    fn set_matrix<'a, P: Into<Option<&'a glib::Variant>>>(&self, matrix: P);

    #[cfg(feature = "v3_24")]
    fn set_types_for_locations(&self, locations: SpaceLocationFlags, types: SpaceTypeFlags);
}

impl<O: IsA<SpaceDrawer>> SpaceDrawerExt for O {
    #[cfg(feature = "v3_24")]
    fn bind_matrix_setting(&self, settings: &gio::Settings, key: &str, flags: gio::SettingsBindFlags) {
        unsafe {
            ffi::gtk_source_space_drawer_bind_matrix_setting(self.to_glib_none().0, settings.to_glib_none().0, key.to_glib_none().0, flags.to_glib());
        }
    }

    #[cfg(feature = "v3_24")]
    fn get_enable_matrix(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_space_drawer_get_enable_matrix(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_24")]
    fn get_matrix(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::gtk_source_space_drawer_get_matrix(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_24")]
    fn get_types_for_locations(&self, locations: SpaceLocationFlags) -> SpaceTypeFlags {
        unsafe {
            from_glib(ffi::gtk_source_space_drawer_get_types_for_locations(self.to_glib_none().0, locations.to_glib()))
        }
    }

    #[cfg(feature = "v3_24")]
    fn set_enable_matrix(&self, enable_matrix: bool) {
        unsafe {
            ffi::gtk_source_space_drawer_set_enable_matrix(self.to_glib_none().0, enable_matrix.to_glib());
        }
    }

    #[cfg(feature = "v3_24")]
    fn set_matrix<'a, P: Into<Option<&'a glib::Variant>>>(&self, matrix: P) {
        let matrix = matrix.into();
        let matrix = matrix.to_glib_none();
        unsafe {
            ffi::gtk_source_space_drawer_set_matrix(self.to_glib_none().0, matrix.0);
        }
    }

    #[cfg(feature = "v3_24")]
    fn set_types_for_locations(&self, locations: SpaceLocationFlags, types: SpaceTypeFlags) {
        unsafe {
            ffi::gtk_source_space_drawer_set_types_for_locations(self.to_glib_none().0, locations.to_glib(), types.to_glib());
        }
    }
}