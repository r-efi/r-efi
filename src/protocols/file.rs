//! File Protocol
//!
//! Provides an interface to interact with both files and directories. This protocol is typically
//! obtained via an EFI_SIMPLE_FILE_SYSTEM protocol or via another EFI_FILE_PROTOCOL.

use crate::signatures;

pub const REVISION: u64 = 0x0000_0000_0001_0000u64;
pub const REVISION2: u64 = 0x0000_0000_0002_0000u64;
pub const LATEST_REVISION: u64 = REVISION2;

pub const MODE_READ: u64 = 0x0000000000000001u64;
pub const MODE_WRITE: u64 = 0x0000000000000002u64;
pub const MODE_CREATE: u64 = 0x0000000000000000u64;

pub const READ_ONLY: u64 = 0x0000000000000001u64;
pub const HIDDEN: u64 = 0x0000000000000002u64;
pub const SYSTEM: u64 = 0x0000000000000004u64;
pub const RESERVED: u64 = 0x0000000000000008u64;
pub const DIRECTORY: u64 = 0x0000000000000010u64;
pub const ARCHIVE: u64 = 0x0000000000000020u64;
pub const VALID_ATTR: u64 = 0x0000000000000037u64;

pub const INFO_ID: crate::base::Guid = crate::base::Guid::from_fields(
    0x09576e92,
    0x6d3f,
    0x11d2,
    0x8e,
    0x39,
    &[0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);
pub const SYSTEM_INFO_ID: crate::base::Guid = crate::base::Guid::from_fields(
    0x09576e93,
    0x6d3f,
    0x11d2,
    0x8e,
    0x39,
    &[0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);
pub const SYSTEM_VOLUME_LABEL_ID: crate::base::Guid = crate::base::Guid::from_fields(
    0xdb47d7d3,
    0xfe81,
    0x11d3,
    0x9a,
    0x35,
    &[0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IoToken {
    pub event: crate::base::Event,
    pub status: crate::base::Status,
    pub buffer_size: usize,
    pub buffer: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Info {
    pub size: u64,
    pub file_size: u64,
    pub physical_size: u64,
    pub create_time: crate::system::Time,
    pub last_access_time: crate::system::Time,
    pub modification_time: crate::system::Time,
    pub attribute: u64,
    pub file_name: [crate::base::Char16; 0],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SystemInfo {
    pub size: u64,
    pub read_only: crate::base::Boolean,
    pub volume_size: u64,
    pub free_space: u64,
    pub block_size: u32,
    pub volume_label: [crate::base::Char16; 0],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SystemVolumeLabel {
    pub volume_label: [crate::base::Char16; 0],
}

#[repr(C)]
pub struct Protocol {
    pub revision: u64,
    pub open: signatures::protocols::file::OpenSignature,
    pub close: signatures::protocols::file::CloseSignature,
    pub delete: signatures::protocols::file::DeleteSignature,
    pub read: signatures::protocols::file::ReadSignature,
    pub write: signatures::protocols::file::WriteSignature,
    pub get_position: signatures::protocols::file::GetPositionSignature,
    pub set_position: signatures::protocols::file::SetPositionSignature,
    pub get_info: signatures::protocols::file::GetInfoSignature,
    pub set_info: signatures::protocols::file::SetInfoSignature,
    pub flush: signatures::protocols::file::FlushSignature,
    pub open_ex: signatures::protocols::file::OpenExSignature,
    pub read_ex: signatures::protocols::file::ReadExSignature,
    pub write_ex: signatures::protocols::file::WriteExSignature,
    pub flush_ex: signatures::protocols::file::FlushExSignature,
}
