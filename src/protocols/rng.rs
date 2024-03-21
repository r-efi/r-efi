//! Random Number Generator Protocol
//!
//! This protocol is used to provide random numbers for use in applications, or
//! entropy for seeding other random number generators.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::new(
    0x3152bca5_u32.to_ne_bytes(),
    0xeade_u16.to_ne_bytes(),
    0x433d_u16.to_ne_bytes(),
    0x86,
    0x2e,
    [0xc0, 0x1c, 0xdc, 0x29, 0x1f, 0x44],
);

pub type Algorithm = crate::base::Guid;

pub const ALGORITHM_SP800_90_HASH_256_GUID: Algorithm = crate::base::Guid::new(
    0xa7af67cb_u32.to_ne_bytes(),
    0x603b_u16.to_ne_bytes(),
    0x4d42_u16.to_ne_bytes(),
    0xba,
    0x21,
    [0x70, 0xbf, 0xb6, 0x29, 0x3f, 0x96],
);
pub const ALGORITHM_SP800_90_HMAC_256_GUID: Algorithm = crate::base::Guid::new(
    0xc5149b43_u32.to_ne_bytes(),
    0xae85_u16.to_ne_bytes(),
    0x4f53_u16.to_ne_bytes(),
    0x99,
    0x82,
    [0xb9, 0x43, 0x35, 0xd3, 0xa9, 0xe7],
);
pub const ALGORITHM_SP800_90_CTR_256_GUID: Algorithm = crate::base::Guid::new(
    0x44f0de6e_u32.to_ne_bytes(),
    0x4d8c_u16.to_ne_bytes(),
    0x4045_u16.to_ne_bytes(),
    0xa8,
    0xc7,
    [0x4d, 0xd1, 0x68, 0x85, 0x6b, 0x9e],
);
pub const ALGORITHM_X9_31_3DES_GUID: Algorithm = crate::base::Guid::new(
    0x63c4785a_u32.to_ne_bytes(),
    0xca34_u16.to_ne_bytes(),
    0x4012_u16.to_ne_bytes(),
    0xa3,
    0xc8,
    [0x0b, 0x6a, 0x32, 0x4f, 0x55, 0x46],
);
pub const ALGORITHM_X9_31_AES_GUID: Algorithm = crate::base::Guid::new(
    0xacd03321_u32.to_ne_bytes(),
    0x777e_u16.to_ne_bytes(),
    0x4d3d_u16.to_ne_bytes(),
    0xb1,
    0xc8,
    [0x20, 0xcf, 0xd8, 0x88, 0x20, 0xc9],
);
pub const ALGORITHM_RAW: Algorithm = crate::base::Guid::new(
    0xe43176d7_u32.to_ne_bytes(),
    0xb6e8_u16.to_ne_bytes(),
    0x4827_u16.to_ne_bytes(),
    0xb7,
    0x84,
    [0x7f, 0xfd, 0xc4, 0xb6, 0x85, 0x61],
);

pub type ProtocolGetInfo = eficall! {fn(
    *mut Protocol,
    *mut usize,
    *mut Algorithm,
) -> crate::base::Status};

pub type ProtocolGetRng = eficall! {fn(
    *mut Protocol,
    *mut Algorithm,
    usize,
    *mut u8,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub get_info: ProtocolGetInfo,
    pub get_rng: ProtocolGetRng,
}
