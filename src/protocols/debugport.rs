//! Debug Port Protocol
//!
//! It provides the communication link between the debug agent and the remote host.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xEBA4E8D2,
    0x3858,
    0x41EC,
    0xA2,
    0x81,
    &[0x26, 0x47, 0xBA, 0x96, 0x60, 0xD0],
);

pub type Reset = eficall! {fn(
    *mut Protocol,
) -> *mut crate::base::Status};

pub type Write = eficall! {fn(
    *mut Protocol,
    u32,
    *mut usize,
    *mut core::ffi::c_void
) -> *mut crate::base::Status};

pub type Read = eficall! {fn(
    *mut Protocol,
    u32,
    *mut usize,
    *mut core::ffi::c_void
) -> *mut crate::base::Status};

pub type Poll = eficall! {fn(
    *mut Protocol,
) -> *mut crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub reset: Reset,
    pub write: Write,
    pub read: Read,
    pub poll: Poll,
}
