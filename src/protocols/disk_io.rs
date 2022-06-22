//! Disk I/O Protocol
//!
//! Abstracts block accesses of the Block I/O protocol to a more general offset-length protocol.
//! Firmware is responsible for adding this protocol to any Block I/O interface that appears
//! in the system that does not already have a Disk I/O protocol. File systems and other disk
//! access code utilize the Disk I/O protocol.

use crate::signatures;

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xce345171,
    0xba0b,
    0x11d2,
    0x8e,
    0x4f,
    &[0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

pub const REVISION: u64 = 0x0000000000010000u64;

#[repr(C)]
pub struct Protocol {
    pub revision: u64,
    pub read_disk: signatures::protocols::disk_io::ReadDiskSignature,
    pub write_disk: signatures::protocols::disk_io::WriteDiskSignature,
}
