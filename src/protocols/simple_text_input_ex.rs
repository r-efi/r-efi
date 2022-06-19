//! Extended Simple Text Input Protocol
//!
//! The simple-text-input-ex protocol extends the simple-text-input protocol by allowing more
//! details reporting about modifiers, etc.

use crate::signatures;

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xdd9e7534,
    0x7762,
    0x4698,
    0x8c,
    0x14,
    &[0xf5, 0x85, 0x17, 0xa6, 0x25, 0xaa],
);

pub const SHIFT_STATE_VALID: u32 = 0x80000000u32;
pub const RIGHT_SHIFT_PRESSED: u32 = 0x00000001u32;
pub const LEFT_SHIFT_PRESSED: u32 = 0x00000002u32;
pub const RIGHT_CONTROL_PRESSED: u32 = 0x00000004u32;
pub const LEFT_CONTROL_PRESSED: u32 = 0x00000008u32;
pub const RIGHT_ALT_PRESSED: u32 = 0x00000010u32;
pub const LEFT_ALT_PRESSED: u32 = 0x00000020u32;
pub const RIGHT_LOGO_PRESSED: u32 = 0x00000040u32;
pub const LEFT_LOGO_PRESSED: u32 = 0x00000080u32;
pub const MENU_KEY_PRESSED: u32 = 0x00000100u32;
pub const SYS_REQ_PRESSED: u32 = 0x00000200u32;

pub const TOGGLE_STATE_VALID: u8 = 0x80u8;
pub const KEY_STATE_EXPOSED: u8 = 0x40u8;
pub const SCROLL_LOCK_ACTIVE: u8 = 0x01u8;
pub const NUM_LOCK_ACTIVE: u8 = 0x02u8;
pub const CAPS_LOCK_ACTIVE: u8 = 0x04u8;

pub type KeyToggleState = u8;
pub type KeyNotifyFunction =
    signatures::protocols::simple_text_input_ex::KeyNotifyFunctionSignature;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct KeyState {
    pub key_shift_state: u32,
    pub key_toggle_state: KeyToggleState,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct KeyData {
    pub key: crate::protocols::simple_text_input::InputKey,
    pub key_state: KeyState,
}

#[repr(C)]
pub struct Protocol {
    pub reset: signatures::protocols::simple_text_input_ex::ResetSignature,
    pub read_key_stroke_ex: signatures::protocols::simple_text_input_ex::ReadKeyStrokeExSignature,
    pub wait_for_key_ex: crate::base::Event,
    pub register_key_notify:
        signatures::protocols::simple_text_input_ex::RegisterKeyNotifySignature,
    pub unregister_key_notify:
        signatures::protocols::simple_text_input_ex::UnregisterKeyNotifySignature,
}
