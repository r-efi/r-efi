//! HII String Protocol

use crate::signatures;

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
    pub new_string: signatures::protocols::hii_string::NewStringSignature,
    pub get_string: signatures::protocols::hii_string::GetStringSignature,
    pub set_string: signatures::protocols::hii_string::SetStringSignature,
    pub get_languages: signatures::protocols::hii_string::GetLanguagesSignature,
    pub get_secondary_languages: signatures::protocols::hii_string::GetSecondaryLanguagesSignature,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Info {
    pub font_style: crate::hii::FontStyle,
    pub font_size: u16,
    pub font_name: [crate::base::Char16; 0],
}
