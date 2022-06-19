//! Device Path Utilities Protocol
//!
//! The device-path utilities protocol provides common utilities for creating and manipulating
//! device paths and device nodes.

use crate::signatures;

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x379be4e,
    0xd706,
    0x437d,
    0xb0,
    0x37,
    &[0xed, 0xb8, 0x2f, 0xb7, 0x72, 0xa4],
);

#[repr(C)]
pub struct Protocol {
    pub get_device_path_size:
        signatures::protocols::device_path_utilities::GetDevicePathSizeSignature,
    pub duplicate_device_path:
        signatures::protocols::device_path_utilities::DuplicateDevicePathSignature,
    pub append_device_path: signatures::protocols::device_path_utilities::AppendDevicePathSignature,
    pub append_device_node: signatures::protocols::device_path_utilities::AppendDeviceNodeSignature,
    pub append_device_path_instance:
        signatures::protocols::device_path_utilities::AppendDevicePathInstanceSignature,
    pub get_next_device_path_instance:
        signatures::protocols::device_path_utilities::GetNextDevicePathInstanceSignature,
    pub is_device_path_multi_instance:
        signatures::protocols::device_path_utilities::IsDevicePathMultiInstanceSignature,
    pub create_device_node: signatures::protocols::device_path_utilities::CreateDeviceNodeSignature,
}
