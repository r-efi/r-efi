//! SMBIOS Protocol
//!
//! This protocol provides the interface for managing SMBIOS records.

/// The SMBIOS protocol GUID.
pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x03583ff6,
    0xcb36,
    0x4940,
    0x94,
    0x7e,
    &[0xb9, 0xb3, 0x9f, 0x4a, 0xfa, 0xf7],
);

/// The reserved SMBIOS handle used for automatic handle assignment.
pub const HANDLE_PI_RESERVED: u16 = 0xfffe;
