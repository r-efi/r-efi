//! Extended Simple Text Input Protocol
//!
//! The simple-text-input-ex protocol extends the simple-text-input protocol by allowing more
//! details reporting about modifiers, etc.
//!
//! The scan codes from [`crate::protocols::simple_text_input`] also apply to this protocol.
//! Scan codes in the range 0x8000..=0xffff are reserved for OEM use.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xdd9e7534,
    0x7762,
    0x4698,
    0x8c,
    0x14,
    &[0xf5, 0x85, 0x17, 0xa6, 0x25, 0xaa],
);

// Additional EFI scan codes
pub const SCAN_F11: u16 = 0x0015;
pub const SCAN_F12: u16 = 0x0016;
pub const SCAN_PAUSE: u16 = 0x0048;
pub const SCAN_F13: u16 = 0x0068;
pub const SCAN_F14: u16 = 0x0069;
pub const SCAN_F15: u16 = 0x006a;
pub const SCAN_F16: u16 = 0x006b;
pub const SCAN_F17: u16 = 0x006c;
pub const SCAN_F18: u16 = 0x006d;
pub const SCAN_F19: u16 = 0x006e;
pub const SCAN_F20: u16 = 0x006f;
pub const SCAN_F21: u16 = 0x0070;
pub const SCAN_F22: u16 = 0x0071;
pub const SCAN_F23: u16 = 0x0072;
pub const SCAN_F24: u16 = 0x0073;
pub const SCAN_MUTE: u16 = 0x007f;
pub const SCAN_VOLUME_UP: u16 = 0x0080;
pub const SCAN_VOLUME_DOWN: u16 = 0x0081;
pub const SCAN_BRIGHTNESS_UP: u16 = 0x0100;
pub const SCAN_BRIGHTNESS_DOWN: u16 = 0x0101;
pub const SCAN_SUSPEND: u16 = 0x0102;
pub const SCAN_HIBERNATE: u16 = 0x0103;
pub const SCAN_TOGGLE_DISPLAY: u16 = 0x0104;
pub const SCAN_RECOVERY: u16 = 0x0105;
pub const SCAN_EJECT: u16 = 0x0106;

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
pub type KeyNotifyFunction = unsafe extern "efiapi" fn(*mut KeyData) -> crate::base::Status;

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

pub type ProtocolReset = unsafe extern "efiapi" fn(
    *mut Protocol,
    crate::base::Boolean,
) -> crate::base::Status;

pub type ProtocolReadKeyStrokeEx = unsafe extern "efiapi" fn(
    *mut Protocol,
    *mut KeyData,
) -> crate::base::Status;

pub type ProtocolSetState = unsafe extern "efiapi" fn(
    *mut Protocol,
    *mut KeyToggleState,
) -> crate::base::Status;

pub type ProtocolRegisterKeyNotify = unsafe extern "efiapi" fn(
    *mut Protocol,
    *mut KeyData,
    KeyNotifyFunction,
    *mut *mut core::ffi::c_void,
) -> crate::base::Status;

pub type ProtocolUnregisterKeyNotify = unsafe extern "efiapi" fn(
    *mut Protocol,
    *mut core::ffi::c_void,
) -> crate::base::Status;

#[repr(C)]
pub struct Protocol {
    pub reset: ProtocolReset,
    pub read_key_stroke_ex: ProtocolReadKeyStrokeEx,
    pub wait_for_key_ex: crate::base::Event,
    pub set_state: ProtocolSetState,
    pub register_key_notify: ProtocolRegisterKeyNotify,
    pub unregister_key_notify: ProtocolUnregisterKeyNotify,
}
