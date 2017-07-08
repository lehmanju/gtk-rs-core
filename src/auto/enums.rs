// This file was generated by gir (89daf8f) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use glib_ffi;
use glib::error::ErrorDomain;
use glib::translate::*;
use std;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum FileType {
    Unknown,
    Regular,
    Directory,
    SymbolicLink,
    Special,
    Shortcut,
    Mountable,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for FileType {
    type GlibType = ffi::GFileType;

    fn to_glib(&self) -> ffi::GFileType {
        match *self {
            FileType::Unknown => ffi::G_FILE_TYPE_UNKNOWN,
            FileType::Regular => ffi::G_FILE_TYPE_REGULAR,
            FileType::Directory => ffi::G_FILE_TYPE_DIRECTORY,
            FileType::SymbolicLink => ffi::G_FILE_TYPE_SYMBOLIC_LINK,
            FileType::Special => ffi::G_FILE_TYPE_SPECIAL,
            FileType::Shortcut => ffi::G_FILE_TYPE_SHORTCUT,
            FileType::Mountable => ffi::G_FILE_TYPE_MOUNTABLE,
            FileType::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GFileType> for FileType {
    fn from_glib(value: ffi::GFileType) -> Self {
        match value as i32 {
            0 => FileType::Unknown,
            1 => FileType::Regular,
            2 => FileType::Directory,
            3 => FileType::SymbolicLink,
            4 => FileType::Special,
            5 => FileType::Shortcut,
            6 => FileType::Mountable,
            value => FileType::__Unknown(value),
        }
    }
}

#[cfg(feature = "v2_42")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum NotificationPriority {
    Normal,
    Low,
    High,
    Urgent,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(feature = "v2_42")]
#[doc(hidden)]
impl ToGlib for NotificationPriority {
    type GlibType = ffi::GNotificationPriority;

    fn to_glib(&self) -> ffi::GNotificationPriority {
        match *self {
            NotificationPriority::Normal => ffi::G_NOTIFICATION_PRIORITY_NORMAL,
            NotificationPriority::Low => ffi::G_NOTIFICATION_PRIORITY_LOW,
            NotificationPriority::High => ffi::G_NOTIFICATION_PRIORITY_HIGH,
            NotificationPriority::Urgent => ffi::G_NOTIFICATION_PRIORITY_URGENT,
            NotificationPriority::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[cfg(feature = "v2_42")]
#[doc(hidden)]
impl FromGlib<ffi::GNotificationPriority> for NotificationPriority {
    fn from_glib(value: ffi::GNotificationPriority) -> Self {
        match value as i32 {
            0 => NotificationPriority::Normal,
            1 => NotificationPriority::Low,
            2 => NotificationPriority::High,
            3 => NotificationPriority::Urgent,
            value => NotificationPriority::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ResourceError {
    NotFound,
    Internal,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for ResourceError {
    type GlibType = ffi::GResourceError;

    fn to_glib(&self) -> ffi::GResourceError {
        match *self {
            ResourceError::NotFound => ffi::G_RESOURCE_ERROR_NOT_FOUND,
            ResourceError::Internal => ffi::G_RESOURCE_ERROR_INTERNAL,
            ResourceError::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GResourceError> for ResourceError {
    fn from_glib(value: ffi::GResourceError) -> Self {
        match value as i32 {
            0 => ResourceError::NotFound,
            1 => ResourceError::Internal,
            value => ResourceError::__Unknown(value),
        }
    }
}

impl ErrorDomain for ResourceError {
    fn domain() -> glib_ffi::GQuark {
        unsafe { ffi::g_resource_error_quark() }
    }

    fn code(self) -> i32 {
        self.to_glib() as i32
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(ResourceError::NotFound),
            1 => Some(ResourceError::Internal),
            value => Some(ResourceError::__Unknown(value)),
        }
    }
}

