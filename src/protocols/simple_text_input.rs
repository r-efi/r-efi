//! Simple Text Input Protocol
//!
//! XXX

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_spec(
    0x387477c1, 0x69c7, 0x11d2, 0x8e, 0x39, &[0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]
);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InputKey {
    pub scan_code: u16,
    pub unicode_char: crate::base::Char16,
}

#[repr(C)]
pub struct Protocol {
    pub reset: eficall!{fn(
        *mut Protocol,
        crate::base::Boolean,
    ) -> crate::base::Status},
    pub read_key_stroke: eficall!{fn(
        *mut Protocol,
        *mut InputKey,
    ) -> crate::base::Status},
    pub wait_for_key: crate::base::Event,
}
