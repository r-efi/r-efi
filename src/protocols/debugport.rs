//! Debug Port Protocol
//!
//! It provides the communication link between the debug agent and the remote host.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::new(
    0xeba4e8d2_u32.to_ne_bytes(),
    0x3858_u16.to_ne_bytes(),
    0x41ec_u16.to_ne_bytes(),
    0xa2,
    0x81,
    [0x26, 0x47, 0xba, 0x96, 0x60, 0xd0],
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
