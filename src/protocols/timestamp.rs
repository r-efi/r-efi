//! EFI Timestamp Protocol
//!
//! The Timestamp protocol provides a platform independent interface for
//! retrieving a high resolution timestamp counter.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xafbfde41,
    0x2e6e,
    0x4262,
    0xba,
    0x65,
    &[0x62, 0xb9, 0x23, 0x6e, 0x54, 0x95],
);

#[repr(C)]
pub struct Properties {
    frequence: u64,
    end_value: u64,
}

pub type GetTimestamp = eficall! {fn() -> u64};
pub type GetProperties = eficall! {fn(
    *mut Properties
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    get_timestamp: GetTimestamp,
    get_properties: GetProperties,
}
