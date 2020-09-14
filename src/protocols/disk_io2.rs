//! Disk I/O 2 Protocol
//!
//! Extends the Disk I/O protocol interface to enable non-blocking /
//! asynchronous byte-oriented disk operation.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x151c8eae,
    0x7f2c,
    0x472c,
    0x9e,
    0x54,
    &[0x98, 0x28, 0x19, 0x4f, 0x6a, 0x88],
);

pub const REVISION: u64 = 0x0000000000020000u64;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Token {
    event: crate::base::Event,
    transaction_status: crate::base::Status,
}

#[repr(C)]
pub struct Protocol {
    pub revision: u64,
    pub cancel: eficall! {fn(
        *mut Protocol,
    ) -> crate::base::Status},
    pub read_disk_ex: eficall! {fn(
        *mut Protocol,
        u32,
        u64,
        *mut Token,
        usize,
        *mut core::ffi::c_void,
    ) -> crate::base::Status},
    pub write_disk_ex: eficall! {fn(
        *mut Protocol,
        u32,
        u64,
        *mut Token,
        usize,
        *mut core::ffi::c_void,
    ) -> crate::base::Status},
    pub flush_disk_ex: eficall! {fn(
        *mut Protocol,
        *mut Token,
    ) -> crate::base::Status},
}
