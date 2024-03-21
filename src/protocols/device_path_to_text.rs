//! Device Path to Text Protocol
//!
//! Convert device nodes and paths to text.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::new(
    0x8b843e20_u32.to_ne_bytes(),
    0x8132_u16.to_ne_bytes(),
    0x4852_u16.to_ne_bytes(),
    0x90,
    0xcc,
    [0x55, 0x1a, 0x4e, 0x4a, 0x7f, 0x1c],
);

pub type DevicePathToTextNode = eficall! {fn(
    *mut crate::protocols::device_path::Protocol,
    crate::base::Boolean,
    crate::base::Boolean,
) -> *mut crate::base::Char16};

pub type DevicePathToTextPath = eficall! {fn(
    *mut crate::protocols::device_path::Protocol,
    crate::base::Boolean,
    crate::base::Boolean,
) -> *mut crate::base::Char16};

#[repr(C)]
pub struct Protocol {
    pub convert_device_node_to_text: DevicePathToTextNode,
    pub convert_device_path_to_text: DevicePathToTextPath,
}
