// This file was generated by gir (b010d34) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use glib_ffi;
use glib::error::ErrorDomain;
use glib::translate::*;

#[cfg(feature = "v3_16")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BackgroundPatternType {
    None,
    Grid,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[cfg(feature = "v3_16")]
#[doc(hidden)]
impl ToGlib for BackgroundPatternType {
    type GlibType = ffi::GtkSourceBackgroundPatternType;

    fn to_glib(&self) -> ffi::GtkSourceBackgroundPatternType {
        match *self {
            BackgroundPatternType::None => ffi::GTK_SOURCE_BACKGROUND_PATTERN_TYPE_NONE,
            BackgroundPatternType::Grid => ffi::GTK_SOURCE_BACKGROUND_PATTERN_TYPE_GRID,
            BackgroundPatternType::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[cfg(feature = "v3_16")]
#[doc(hidden)]
impl FromGlib<ffi::GtkSourceBackgroundPatternType> for BackgroundPatternType {
    fn from_glib(value: ffi::GtkSourceBackgroundPatternType) -> Self {
        match value {
            ffi::GTK_SOURCE_BACKGROUND_PATTERN_TYPE_NONE => BackgroundPatternType::None,
            ffi::GTK_SOURCE_BACKGROUND_PATTERN_TYPE_GRID => BackgroundPatternType::Grid,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BracketMatchType {
    None,
    OutOfRange,
    NotFound,
    Found,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for BracketMatchType {
    type GlibType = ffi::GtkSourceBracketMatchType;

    fn to_glib(&self) -> ffi::GtkSourceBracketMatchType {
        match *self {
            BracketMatchType::None => ffi::GTK_SOURCE_BRACKET_MATCH_NONE,
            BracketMatchType::OutOfRange => ffi::GTK_SOURCE_BRACKET_MATCH_OUT_OF_RANGE,
            BracketMatchType::NotFound => ffi::GTK_SOURCE_BRACKET_MATCH_NOT_FOUND,
            BracketMatchType::Found => ffi::GTK_SOURCE_BRACKET_MATCH_FOUND,
            BracketMatchType::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceBracketMatchType> for BracketMatchType {
    fn from_glib(value: ffi::GtkSourceBracketMatchType) -> Self {
        match value {
            ffi::GTK_SOURCE_BRACKET_MATCH_NONE => BracketMatchType::None,
            ffi::GTK_SOURCE_BRACKET_MATCH_OUT_OF_RANGE => BracketMatchType::OutOfRange,
            ffi::GTK_SOURCE_BRACKET_MATCH_NOT_FOUND => BracketMatchType::NotFound,
            ffi::GTK_SOURCE_BRACKET_MATCH_FOUND => BracketMatchType::Found,
        }
    }
}

#[cfg(feature = "v3_12")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ChangeCaseType {
    Lower,
    Upper,
    Toggle,
    Title,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[cfg(feature = "v3_12")]
#[doc(hidden)]
impl ToGlib for ChangeCaseType {
    type GlibType = ffi::GtkSourceChangeCaseType;

    fn to_glib(&self) -> ffi::GtkSourceChangeCaseType {
        match *self {
            ChangeCaseType::Lower => ffi::GTK_SOURCE_CHANGE_CASE_LOWER,
            ChangeCaseType::Upper => ffi::GTK_SOURCE_CHANGE_CASE_UPPER,
            ChangeCaseType::Toggle => ffi::GTK_SOURCE_CHANGE_CASE_TOGGLE,
            ChangeCaseType::Title => ffi::GTK_SOURCE_CHANGE_CASE_TITLE,
            ChangeCaseType::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[cfg(feature = "v3_12")]
#[doc(hidden)]
impl FromGlib<ffi::GtkSourceChangeCaseType> for ChangeCaseType {
    fn from_glib(value: ffi::GtkSourceChangeCaseType) -> Self {
        match value {
            ffi::GTK_SOURCE_CHANGE_CASE_LOWER => ChangeCaseType::Lower,
            ffi::GTK_SOURCE_CHANGE_CASE_UPPER => ChangeCaseType::Upper,
            ffi::GTK_SOURCE_CHANGE_CASE_TOGGLE => ChangeCaseType::Toggle,
            ffi::GTK_SOURCE_CHANGE_CASE_TITLE => ChangeCaseType::Title,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CompletionError {
    AlreadyBound,
    NotBound,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for CompletionError {
    type GlibType = ffi::GtkSourceCompletionError;

    fn to_glib(&self) -> ffi::GtkSourceCompletionError {
        match *self {
            CompletionError::AlreadyBound => ffi::GTK_SOURCE_COMPLETION_ERROR_ALREADY_BOUND,
            CompletionError::NotBound => ffi::GTK_SOURCE_COMPLETION_ERROR_NOT_BOUND,
            CompletionError::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceCompletionError> for CompletionError {
    fn from_glib(value: ffi::GtkSourceCompletionError) -> Self {
        match value {
            ffi::GTK_SOURCE_COMPLETION_ERROR_ALREADY_BOUND => CompletionError::AlreadyBound,
            ffi::GTK_SOURCE_COMPLETION_ERROR_NOT_BOUND => CompletionError::NotBound,
        }
    }
}

impl ErrorDomain for CompletionError {
    fn domain() -> glib_ffi::GQuark {
        unsafe { ffi::gtk_source_completion_error_quark() }
    }

    fn code(self) -> i32 {
        self.to_glib() as i32
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            x if x == ffi::GTK_SOURCE_COMPLETION_ERROR_ALREADY_BOUND as i32 => Some(CompletionError::AlreadyBound),
            x if x == ffi::GTK_SOURCE_COMPLETION_ERROR_NOT_BOUND as i32 => Some(CompletionError::NotBound),
            _ => Some(CompletionError::__Nonexhaustive(())),
        }
    }
}

#[cfg(feature = "v3_14")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CompressionType {
    None,
    Gzip,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[cfg(feature = "v3_14")]
#[doc(hidden)]
impl ToGlib for CompressionType {
    type GlibType = ffi::GtkSourceCompressionType;

    fn to_glib(&self) -> ffi::GtkSourceCompressionType {
        match *self {
            CompressionType::None => ffi::GTK_SOURCE_COMPRESSION_TYPE_NONE,
            CompressionType::Gzip => ffi::GTK_SOURCE_COMPRESSION_TYPE_GZIP,
            CompressionType::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[cfg(feature = "v3_14")]
#[doc(hidden)]
impl FromGlib<ffi::GtkSourceCompressionType> for CompressionType {
    fn from_glib(value: ffi::GtkSourceCompressionType) -> Self {
        match value {
            ffi::GTK_SOURCE_COMPRESSION_TYPE_NONE => CompressionType::None,
            ffi::GTK_SOURCE_COMPRESSION_TYPE_GZIP => CompressionType::Gzip,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum FileLoaderError {
    TooBig,
    EncodingAutoDetectionFailed,
    ConversionFallback,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for FileLoaderError {
    type GlibType = ffi::GtkSourceFileLoaderError;

    fn to_glib(&self) -> ffi::GtkSourceFileLoaderError {
        match *self {
            FileLoaderError::TooBig => ffi::GTK_SOURCE_FILE_LOADER_ERROR_TOO_BIG,
            FileLoaderError::EncodingAutoDetectionFailed => ffi::GTK_SOURCE_FILE_LOADER_ERROR_ENCODING_AUTO_DETECTION_FAILED,
            FileLoaderError::ConversionFallback => ffi::GTK_SOURCE_FILE_LOADER_ERROR_CONVERSION_FALLBACK,
            FileLoaderError::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceFileLoaderError> for FileLoaderError {
    fn from_glib(value: ffi::GtkSourceFileLoaderError) -> Self {
        match value {
            ffi::GTK_SOURCE_FILE_LOADER_ERROR_TOO_BIG => FileLoaderError::TooBig,
            ffi::GTK_SOURCE_FILE_LOADER_ERROR_ENCODING_AUTO_DETECTION_FAILED => FileLoaderError::EncodingAutoDetectionFailed,
            ffi::GTK_SOURCE_FILE_LOADER_ERROR_CONVERSION_FALLBACK => FileLoaderError::ConversionFallback,
        }
    }
}

impl ErrorDomain for FileLoaderError {
    fn domain() -> glib_ffi::GQuark {
        unsafe { ffi::gtk_source_file_loader_error_quark() }
    }

    fn code(self) -> i32 {
        self.to_glib() as i32
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            x if x == ffi::GTK_SOURCE_FILE_LOADER_ERROR_TOO_BIG as i32 => Some(FileLoaderError::TooBig),
            x if x == ffi::GTK_SOURCE_FILE_LOADER_ERROR_ENCODING_AUTO_DETECTION_FAILED as i32 => Some(FileLoaderError::EncodingAutoDetectionFailed),
            x if x == ffi::GTK_SOURCE_FILE_LOADER_ERROR_CONVERSION_FALLBACK as i32 => Some(FileLoaderError::ConversionFallback),
            _ => Some(FileLoaderError::__Nonexhaustive(())),
        }
    }
}

#[cfg(feature = "v3_14")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum FileSaverError {
    InvalidChars,
    ExternallyModified,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[cfg(feature = "v3_14")]
#[doc(hidden)]
impl ToGlib for FileSaverError {
    type GlibType = ffi::GtkSourceFileSaverError;

    fn to_glib(&self) -> ffi::GtkSourceFileSaverError {
        match *self {
            FileSaverError::InvalidChars => ffi::GTK_SOURCE_FILE_SAVER_ERROR_INVALID_CHARS,
            FileSaverError::ExternallyModified => ffi::GTK_SOURCE_FILE_SAVER_ERROR_EXTERNALLY_MODIFIED,
            FileSaverError::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[cfg(feature = "v3_14")]
#[doc(hidden)]
impl FromGlib<ffi::GtkSourceFileSaverError> for FileSaverError {
    fn from_glib(value: ffi::GtkSourceFileSaverError) -> Self {
        match value {
            ffi::GTK_SOURCE_FILE_SAVER_ERROR_INVALID_CHARS => FileSaverError::InvalidChars,
            ffi::GTK_SOURCE_FILE_SAVER_ERROR_EXTERNALLY_MODIFIED => FileSaverError::ExternallyModified,
        }
    }
}

#[cfg(feature = "v3_14")]
impl ErrorDomain for FileSaverError {
    fn domain() -> glib_ffi::GQuark {
        unsafe { ffi::gtk_source_file_saver_error_quark() }
    }

    fn code(self) -> i32 {
        self.to_glib() as i32
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            x if x == ffi::GTK_SOURCE_FILE_SAVER_ERROR_INVALID_CHARS as i32 => Some(FileSaverError::InvalidChars),
            x if x == ffi::GTK_SOURCE_FILE_SAVER_ERROR_EXTERNALLY_MODIFIED as i32 => Some(FileSaverError::ExternallyModified),
            _ => Some(FileSaverError::__Nonexhaustive(())),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum GutterRendererAlignmentMode {
    Cell,
    First,
    Last,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for GutterRendererAlignmentMode {
    type GlibType = ffi::GtkSourceGutterRendererAlignmentMode;

    fn to_glib(&self) -> ffi::GtkSourceGutterRendererAlignmentMode {
        match *self {
            GutterRendererAlignmentMode::Cell => ffi::GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_CELL,
            GutterRendererAlignmentMode::First => ffi::GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_FIRST,
            GutterRendererAlignmentMode::Last => ffi::GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_LAST,
            GutterRendererAlignmentMode::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceGutterRendererAlignmentMode> for GutterRendererAlignmentMode {
    fn from_glib(value: ffi::GtkSourceGutterRendererAlignmentMode) -> Self {
        match value {
            ffi::GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_CELL => GutterRendererAlignmentMode::Cell,
            ffi::GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_FIRST => GutterRendererAlignmentMode::First,
            ffi::GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_LAST => GutterRendererAlignmentMode::Last,
        }
    }
}

#[cfg(feature = "v3_14")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum NewlineType {
    Lf,
    Cr,
    CrLf,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[cfg(feature = "v3_14")]
#[doc(hidden)]
impl ToGlib for NewlineType {
    type GlibType = ffi::GtkSourceNewlineType;

    fn to_glib(&self) -> ffi::GtkSourceNewlineType {
        match *self {
            NewlineType::Lf => ffi::GTK_SOURCE_NEWLINE_TYPE_LF,
            NewlineType::Cr => ffi::GTK_SOURCE_NEWLINE_TYPE_CR,
            NewlineType::CrLf => ffi::GTK_SOURCE_NEWLINE_TYPE_CR_LF,
            NewlineType::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[cfg(feature = "v3_14")]
#[doc(hidden)]
impl FromGlib<ffi::GtkSourceNewlineType> for NewlineType {
    fn from_glib(value: ffi::GtkSourceNewlineType) -> Self {
        match value {
            ffi::GTK_SOURCE_NEWLINE_TYPE_LF => NewlineType::Lf,
            ffi::GTK_SOURCE_NEWLINE_TYPE_CR => NewlineType::Cr,
            ffi::GTK_SOURCE_NEWLINE_TYPE_CR_LF => NewlineType::CrLf,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SmartHomeEndType {
    Disabled,
    Before,
    After,
    Always,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for SmartHomeEndType {
    type GlibType = ffi::GtkSourceSmartHomeEndType;

    fn to_glib(&self) -> ffi::GtkSourceSmartHomeEndType {
        match *self {
            SmartHomeEndType::Disabled => ffi::GTK_SOURCE_SMART_HOME_END_DISABLED,
            SmartHomeEndType::Before => ffi::GTK_SOURCE_SMART_HOME_END_BEFORE,
            SmartHomeEndType::After => ffi::GTK_SOURCE_SMART_HOME_END_AFTER,
            SmartHomeEndType::Always => ffi::GTK_SOURCE_SMART_HOME_END_ALWAYS,
            SmartHomeEndType::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceSmartHomeEndType> for SmartHomeEndType {
    fn from_glib(value: ffi::GtkSourceSmartHomeEndType) -> Self {
        match value {
            ffi::GTK_SOURCE_SMART_HOME_END_DISABLED => SmartHomeEndType::Disabled,
            ffi::GTK_SOURCE_SMART_HOME_END_BEFORE => SmartHomeEndType::Before,
            ffi::GTK_SOURCE_SMART_HOME_END_AFTER => SmartHomeEndType::After,
            ffi::GTK_SOURCE_SMART_HOME_END_ALWAYS => SmartHomeEndType::Always,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ViewGutterPosition {
    Lines,
    Marks,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for ViewGutterPosition {
    type GlibType = ffi::GtkSourceViewGutterPosition;

    fn to_glib(&self) -> ffi::GtkSourceViewGutterPosition {
        match *self {
            ViewGutterPosition::Lines => ffi::GTK_SOURCE_VIEW_GUTTER_POSITION_LINES,
            ViewGutterPosition::Marks => ffi::GTK_SOURCE_VIEW_GUTTER_POSITION_MARKS,
            ViewGutterPosition::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceViewGutterPosition> for ViewGutterPosition {
    fn from_glib(value: ffi::GtkSourceViewGutterPosition) -> Self {
        match value {
            ffi::GTK_SOURCE_VIEW_GUTTER_POSITION_LINES => ViewGutterPosition::Lines,
            ffi::GTK_SOURCE_VIEW_GUTTER_POSITION_MARKS => ViewGutterPosition::Marks,
        }
    }
}

