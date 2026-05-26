//! Partition Info Protocol
//!
//! The Partition Info protocol provides access to partition information.

use crate::efi::Guid;

pub const PROTOCOL_GUID: Guid = Guid::from_fields(
    0x8cf2f62c,
    0xbc9b,
    0x4821,
    0x80,
    0x8d,
    &[0xec, 0x9e, 0xc4, 0x21, 0xa1, 0xa0],
);

pub const REVISION: u32 = 0x00010000;

pub const TYPE_OTHER: u32 = 0x00;
pub const TYPE_MBR: u32 = 0x01;
pub const TYPE_GPT: u32 = 0x02;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Protocol {
    pub revision: u32,
    pub partition_type: u32,
    pub system: u8,
    pub reserved: [u8; 7],
    pub info: [u8; 128],
}
