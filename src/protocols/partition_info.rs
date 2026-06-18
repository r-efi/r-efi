//! Partition Info Protocol
//!
//! The Partition Info protocol provides access to partition information.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x8cf2f62c,
    0xbc9b,
    0x4821,
    0x80,
    0x8d,
    &[0xec, 0x9e, 0xc4, 0x21, 0xa1, 0xa0],
);

pub const REVISION: u32 = 0x00001000;

pub const TYPE_OTHER: u32 = 0x00000000;
pub const TYPE_MBR: u32 = 0x00000001;
pub const TYPE_GPT: u32 = 0x00000002;

#[derive(Clone, Copy)]
#[repr(C)]
pub union ProtocolInfo {
    pub gpt: crate::gpt::PartitionEntry,
}

#[derive(Clone, Copy)]
#[repr(C, packed(1))]
pub struct Protocol {
    pub revision: u32,
    pub r#type: u32,
    pub system: u8,
    pub reserved: [u8; 7],
    pub info: ProtocolInfo,
}

#[cfg(test)]
mod test {
    use core::mem;
    use super::*;

    // Since the spec uses `#pragma pack(1)` on some structures, verify that
    // the layouts are correctly translated into Rust.
    #[test]
    fn layout() {
        assert_eq!(mem::align_of::<Protocol>(), 1);
        assert_eq!(mem::size_of::<Protocol>(), 4 + 4 + 1 + 7 + 128);
    }
}
