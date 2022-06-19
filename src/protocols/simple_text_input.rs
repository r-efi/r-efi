//! Simple Text Input Protocol
//!
//! The simple-text-input protocol defines how to read basic key-strokes. It is limited to
//! non-modifiers and lacks any detailed reporting. It is mostly useful for debugging and admin
//! interaction.

use crate::signatures;

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x387477c1,
    0x69c7,
    0x11d2,
    0x8e,
    0x39,
    &[0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct InputKey {
    pub scan_code: u16,
    pub unicode_char: crate::base::Char16,
}

#[repr(C)]
pub struct Protocol {
    pub reset: signatures::protocols::simple_text_input::ResetSignature,
    pub read_key_stroke: signatures::protocols::simple_text_input::ReadKeyStorkeSignature,
    pub wait_for_key: crate::base::Event,
}
