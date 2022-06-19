//! Simple Text Output Protocol
//!
//! The simple-text-output protocol provides a simple way to print text on screen. It is modeled
//! around the old VGA-consoles, but does not carry all the old cruft. It expects a rectangular
//! text array and allows you to move the cursor around to write Unicode symbols to screen.

use crate::signatures;

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x387477c2,
    0x69c7,
    0x11d2,
    0x8e,
    0x39,
    &[0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Mode {
    pub max_mode: i32,
    pub mode: i32,
    pub attribute: i32,
    pub cursor_column: i32,
    pub cursor_row: i32,
    pub cursor_visible: crate::base::Boolean,
}

#[repr(C)]
pub struct Protocol {
    pub reset: signatures::protocols::simple_text_output::ResetSignature,
    pub output_string: signatures::protocols::simple_text_output::OutputStringSignature,
    pub test_string: signatures::protocols::simple_text_output::TestStringSignature,
    pub query_mode: signatures::protocols::simple_text_output::QueryModeSignature,
    pub set_mode: signatures::protocols::simple_text_output::SetModeSignature,
    pub set_attribute: signatures::protocols::simple_text_output::SetAttributeSignature,
    pub clear_screen: signatures::protocols::simple_text_output::ClearScreenSignature,
    pub set_cursor_position: signatures::protocols::simple_text_output::SetCursorPositionSignature,
    pub enable_cursor: signatures::protocols::simple_text_output::EnableCursorSignature,
    pub mode: *mut Mode,
}
