//! Simple Text Input Protocol
//!
//! The simple-text-input protocol defines how to read basic key-strokes. It is limited to
//! non-modifiers and lacks any detailed reporting. It is mostly useful for debugging and admin
//! interaction.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x387477c1,
    0x69c7,
    0x11d2,
    0x8e,
    0x39,
    &[0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

// EFI scan codes
pub const SCAN_NULL: u16 = 0x0000;
pub const SCAN_UP: u16 = 0x0001;
pub const SCAN_DOWN: u16 = 0x0002;
pub const SCAN_RIGHT: u16 = 0x0003;
pub const SCAN_LEFT: u16 = 0x0004;
pub const SCAN_HOME: u16 = 0x0005;
pub const SCAN_END: u16 = 0x0006;
pub const SCAN_INSERT: u16 = 0x0007;
pub const SCAN_DELETE: u16 = 0x0008;
pub const SCAN_PAGE_UP: u16 = 0x0009;
pub const SCAN_PAGE_DOWN: u16 = 0x000a;
pub const SCAN_F1: u16 = 0x000b;
pub const SCAN_F2: u16 = 0x000c;
pub const SCAN_F3: u16 = 0x000d;
pub const SCAN_F4: u16 = 0x000e;
pub const SCAN_F5: u16 = 0x000f;
pub const SCAN_F6: u16 = 0x0010;
pub const SCAN_F7: u16 = 0x0011;
pub const SCAN_F8: u16 = 0x0012;
pub const SCAN_F9: u16 = 0x0013;
pub const SCAN_F10: u16 = 0x0014;
pub const SCAN_ESC: u16 = 0x0017;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct InputKey {
    pub scan_code: u16,
    pub unicode_char: crate::base::Char16,
}

pub type ProtocolReset = unsafe extern "efiapi" fn(
    *mut Protocol,
    crate::base::Boolean,
) -> crate::base::Status;

pub type ProtocolReadKeyStroke = unsafe extern "efiapi" fn(
    *mut Protocol,
    *mut InputKey,
) -> crate::base::Status;

#[repr(C)]
pub struct Protocol {
    pub reset: ProtocolReset,
    pub read_key_stroke: ProtocolReadKeyStroke,
    pub wait_for_key: crate::base::Event,
}
