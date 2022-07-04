//! TCPv6 Service Binding Protocol
//!
//! It is used to locate EFI TCPv6 Protocol drivers to create and destroy
//! protocol child instance of the driver to communicate with other host using
//! TCP protocol.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xec20eb79,
    0x6c1a,
    0x4664,
    0x9a,
    0x0d,
    &[0xd2, 0xe4, 0xcc, 0x16, 0xd6, 0x64],
);
