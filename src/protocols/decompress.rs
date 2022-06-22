//! Decompress Protocol
//!
//! The decompress protocol provides a decompression service that allows a compressed source
//! buffer in memory to be decompressed into a destination buffer in memory.

use crate::signatures;

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xd8117cfe,
    0x94a6,
    0x11d4,
    0x9a,
    0x3a,
    &[0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);

#[repr(C)]
pub struct Protocol {
    pub get_info: signatures::protocols::decompress::GetInfoSignature,
    pub decompress: signatures::protocols::decompress::DecompressSignature,
}
