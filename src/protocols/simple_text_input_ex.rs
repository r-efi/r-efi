//! Extended Simple Text Input Protocol
//!
//! The simple-text-input-ex protocol extends the simple-text-input protocol by allowing more
//! details reporting about modifiers, etc.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xdd9e7534, 0x7762, 0x4698, 0x8c, 0x14, &[0xf5, 0x85, 0x17, 0xa6, 0x25, 0xaa]
);

pub type KeyNotifyFunction = eficall!{fn(*mut InputKeyData) -> crate::base::Status};
pub type KeyToggleState = u8;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct InputKeyState {
    pub shift_state: u32,
    pub toggle_state: KeyToggleState,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct InputKeyData {
    pub key: crate::protocols::simple_text_input::InputKey,
    pub state: InputKeyState,
}

#[repr(C)]
pub struct Protocol {
    pub reset: eficall!{fn(
        *mut Protocol,
        crate::base::Boolean,
    ) -> crate::base::Status},
    pub read_key_stroke_ex: eficall!{fn(
        *mut Protocol,
        *mut InputKeyData,
    ) -> crate::base::Status},
    pub wait_for_key_ex: crate::base::Event,
    pub set_state: eficall!{fn(
        *mut Protocol,
        *mut KeyToggleState,
    ) -> crate::base::Status},
    pub register_key_notify: eficall!{fn(
        *mut Protocol,
        *mut InputKeyData,
        KeyNotifyFunction,
        *mut crate::base::Handle,
    ) -> crate::base::Status},
    pub unregister_key_notify: eficall!{fn(
        *mut Protocol,
        crate::base::Handle,
    ) -> crate::base::Status},
}
