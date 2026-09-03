//! Driver Supported EFI Version Protocol
//!
//! The Driver Supported EFI Version Protocol provides the version of the UEFI
//! Specification that a driver conforms to. It is optionally installed on a
//! driver's image handle to advertise the highest revision of the UEFI
//! Specification that the driver supports.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x5c198761,
    0x16a8,
    0x4e69,
    0x97,
    0x2c,
    &[0x89, 0xd6, 0x79, 0x54, 0xf8, 0x1d],
);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Protocol {
    pub length: u32,
    pub firmware_version: u32,
}
