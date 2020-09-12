//! Block I/O Protocol
//!
//! Used to abstract mass storage devices to allow code running in the EFI boot services environment
//! to access the storage devices without specific knowledge of the type of device or controller that
//! manages the device.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x964e5b21,
    0x6459,
    0x11d2,
    0x8e,
    0x39,
    &[0x0, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

pub const REVISION: u64 = 0x0000000000010000u64;
pub const REVISION2: u64 = 0x0000000000020001u64;
pub const REVISION3: u64 = 0x000000000002001fu64;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BlockIoMedia {
    pub media_id: u32,
    pub removable_media: bool,
    pub media_present: bool,
    pub logical_partition: bool,
    pub read_only: bool,
    pub write_caching: bool,
    pub block_size: u32,
    pub io_align: u32,
    pub last_block: u64,
    pub lowest_aligned_lba: u64,
    pub logical_blocks_per_physical_block: u64,
    pub optimal_transfer_length_granularity: u32,
}

#[repr(C)]
pub struct Protocol {
    pub revision: u64,

    pub media: *const BlockIoMedia,

    pub reset: eficall! {fn(
        *mut Protocol,
        crate::base::Boolean,
    ) -> crate::base::Status},

    pub read_blocks: eficall! {fn(
        *mut Protocol,
        u32,
        crate::base::Lba,
        usize,
        *mut core::ffi::c_void,
    ) -> crate::base::Status},

    pub write_blocks: eficall! {fn(
        *mut Protocol,
        u32,
        crate::base::Lba,
        usize,
        *mut core::ffi::c_void,
    ) -> crate::base::Status},

    pub flush_blocks: eficall! {fn(
        *mut Protocol,
    ) -> crate::base::Status},
}
