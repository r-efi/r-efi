//! Shell Parameters Protocol
//!
//! Defined in UEFI Shell Specification, Section 2.3

use super::shell;

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x752f3136,
    0x4e16,
    0x4fdc,
    0xa2,
    0x2a,
    &[0xe5, 0xf4, 0x68, 0x12, 0xf4, 0xca],
);

#[repr(C)]
pub struct Protocol {
    pub argv: *mut *mut crate::base::Char16,
    pub argc: usize,
    pub stdin: shell::FileHandle,
    pub stdout: shell::FileHandle,
    pub stderr: shell::FileHandle,
}
