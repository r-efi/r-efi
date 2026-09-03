//! Component Name Protocol
//!
//! Allows a driver to provide a user readable name of a UEFI Driver, and a user readable name
//! for each of the controllers that the driver is managing. This protocol is used by platform
//! management utilities that wish to display names of components. These names may include the
//! names of expansion slots, external connectors, embedded devices, and add-in devices.
//!
//! Supported languages are specified in ISO 639-2 format.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x107a772c,
    0xd5e1,
    0x11d4,
    0x9a,
    0x46,
    &[0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);

pub type ProtocolGetDriverName = unsafe extern "efiapi" fn(
    *mut Protocol,
    *mut crate::base::Char8,
    *mut *mut crate::base::Char16,
) -> crate::base::Status;

pub type ProtocolGetControllerName = unsafe extern "efiapi" fn(
    *mut Protocol,
    crate::base::Handle,
    crate::base::Handle,
    *mut crate::base::Char8,
    *mut *mut crate::base::Char16,
) -> crate::base::Status;

#[repr(C)]
pub struct Protocol {
    pub get_driver_name: ProtocolGetDriverName,
    pub get_controller_name: ProtocolGetControllerName,
    pub supported_languages: *mut crate::base::Char8,
}
