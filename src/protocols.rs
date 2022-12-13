//! UEFI Protocols
//!
//! The UEFI Specification splits most of its non-core parts into separate protocols. They can
//! refer to each other, but their documentation and implementation is split apart. We provide
//! each protocol as a separate module, so it is clearly defined where a symbol belongs to.

pub mod block_io;
pub mod debug_support;
pub mod debugport;
pub mod decompress;
pub mod device_path;
pub mod device_path_from_text;
pub mod device_path_to_text;
pub mod device_path_utilities;
pub mod disk_io;
pub mod disk_io2;
pub mod driver_binding;
pub mod file;
pub mod graphics_output;
pub mod hii_database;
pub mod hii_font;
pub mod hii_font_ex;
pub mod hii_string;
pub mod ip4;
pub mod ip6;
pub mod loaded_image;
pub mod loaded_image_device_path;
pub mod managed_network;
pub mod rng;
pub mod service_binding;
pub mod shell;
pub mod shell_parameters;
pub mod simple_file_system;
pub mod simple_network;
pub mod simple_text_input;
pub mod simple_text_input_ex;
pub mod simple_text_output;
pub mod tcp4;
pub mod tcp6;
pub mod timestamp;
pub mod udp4;
pub mod udp6;
