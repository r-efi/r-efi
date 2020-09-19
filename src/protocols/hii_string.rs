//! HII (Human Interface Infrastructure) String Protocol from Section 34.3
//!
//! Interfaces which manipulate string data.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xfd96974,
    0x23aa,
    0x4cdc,
    0xb9,
    0xcb,
    &[0x98, 0xd1, 0x77, 0x50, 0x32, 0x2a],
);

#[repr(C)]
pub struct Protocol {
    pub new_string: eficall! {fn(
        *const Protocol,
        crate::hii::Handle,
        *mut crate::hii::StringId,
        *const crate::base::Char8,
        *const crate::base::Char16,
        crate::protocols::hii_font::String,
        *const crate::protocols::hii_font::Info,
    ) -> crate::base::Status},
    pub get_string: eficall! {fn(
        *const Protocol,
        *const crate::base::Char8,
        crate::hii::Handle,
        crate::hii::StringId,
        crate::protocols::hii_font::String,
        *mut usize,
        *mut *mut crate::protocols::hii_font::Info,
    ) -> crate::base::Status},
    pub set_string: eficall! {fn(
        *const Protocol,
        crate::hii::Handle,
        crate::hii::StringId,
        *const crate::base::Char8,
        crate::protocols::hii_font::String,
        *const crate::protocols::hii_font::Info,
    ) -> crate::base::Status},
    pub get_languages: eficall! {fn(
        *const Protocol,
        crate::hii::Handle,
        *mut crate::base::Char8,
        *mut usize,
    ) -> crate::base::Status},
    pub get_secondary_languages: eficall! {fn(
        *const Protocol,
        crate::hii::Handle,
        *const crate::base::Char8,
        *mut crate::base::Char8,
        *mut usize,
    ) -> crate::base::Status},
}
