//! SMBIOS Definitions
//!
//! This module contains a limited set of definitions from the SMBIOS
//! specification, which are relevant to UEFI.

/// `0xFFFE` is reserved by the SMBIOS specification for use by the UEFI PI
/// SMBIOS protocol. It never refers to a real SMBIOS entry and is purely used
/// for API purposes.
///
/// Reference SMBIOS 2.7, chapter 6.1.2.
pub const HANDLE_PI_RESERVED: u16 = 0xfffe;

/// Old SMBIOS editions had a hard limit on the length of a string. This symbol
/// defines the historical maximum length of _significant characters_ in an
/// SMBIOS text string. No such limitation exists in newer editions.
///
/// Reference SMBIOS 2.6, chapter 3.1.3. Removed in newer editions.
pub const STRING_MAX_LENGTH: usize = 64;
