//! Human Interface Infrastructure (HII) Package List Protocol
//!
//! Installed onto an image handle during load if the image contains a custom PE/COFF
//! resource with type 'HII'. The protocol's interface pointer points to the HII package
//! list which is contained in the resource's data.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::new(
    0x6a1ee763_u32.to_ne_bytes(),
    0xd47a_u16.to_ne_bytes(),
    0x43b4_u16.to_ne_bytes(),
    0xaa,
    0xbe,
    [0xef, 0x1d, 0xe2, 0xab, 0x56, 0xfc],
);
