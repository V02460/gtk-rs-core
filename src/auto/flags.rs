// This file was generated by gir (c6b70b0) from gir-files (469db10)
// DO NOT EDIT

use ffi;
use translate::*;

bitflags! {
    pub struct FileTest: u32 {
        const IS_REGULAR = 1;
        const IS_SYMLINK = 2;
        const IS_DIR = 4;
        const IS_EXECUTABLE = 8;
        const EXISTS = 16;
    }
}

#[doc(hidden)]
impl ToGlib for FileTest {
    type GlibType = ffi::GFileTest;

    fn to_glib(&self) -> ffi::GFileTest {
        ffi::GFileTest::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GFileTest> for FileTest {
    fn from_glib(value: ffi::GFileTest) -> FileTest {
        FileTest::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub struct FormatSizeFlags: u32 {
        const DEFAULT = 0;
        const LONG_FORMAT = 1;
        const IEC_UNITS = 2;
    }
}

#[doc(hidden)]
impl ToGlib for FormatSizeFlags {
    type GlibType = ffi::GFormatSizeFlags;

    fn to_glib(&self) -> ffi::GFormatSizeFlags {
        ffi::GFormatSizeFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GFormatSizeFlags> for FormatSizeFlags {
    fn from_glib(value: ffi::GFormatSizeFlags) -> FormatSizeFlags {
        FormatSizeFlags::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub struct KeyFileFlags: u32 {
        const NONE = 0;
        const KEEP_COMMENTS = 1;
        const KEEP_TRANSLATIONS = 2;
    }
}

#[doc(hidden)]
impl ToGlib for KeyFileFlags {
    type GlibType = ffi::GKeyFileFlags;

    fn to_glib(&self) -> ffi::GKeyFileFlags {
        ffi::GKeyFileFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GKeyFileFlags> for KeyFileFlags {
    fn from_glib(value: ffi::GKeyFileFlags) -> KeyFileFlags {
        KeyFileFlags::from_bits_truncate(value.bits())
    }
}

