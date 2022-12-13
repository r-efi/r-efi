//! Driver Diagnostics Protocol
//!
//! Defined in UEFI Specification, Section 11.4
//! Used to perform diagnostics on a controller that a UEFI driver is managing.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x4d330321,
    0x025f,
    0x4aac,
    0x90,
    0xd8,
    &[0x5e, 0xd9, 0x00, 0x17, 0x3b, 0x63],
);

pub type DiagnosticType = u32;

pub const DRIVER_DIAGNOSTIC_TYPE_STANDARD: DiagnosticType = 0;
pub const DRIVER_DIAGNOSTIC_TYPE_EXTENDED: DiagnosticType = 1;
pub const DRIVER_DIAGNOSTIC_TYPE_MANUFACTURING: DiagnosticType = 2;
pub const DRIVER_DIAGNOSTIC_TYPE_CANCEL: DiagnosticType = 3;
pub const DRIVER_DIAGNOSTIC_TYPE_MAXIMUM: DiagnosticType = 4;

pub type RunDiagnostics = eficall! {fn(
    *mut Protocol,
    crate::base::Handle,
    crate::base::Handle,
    DiagnosticType,
    *mut crate::base::Char8,
    *mut *mut crate::base::Guid,
    *mut usize,
    *mut *mut crate::base::Char16,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub run_diagnostics: RunDiagnostics,
    pub supported_languages: *mut crate::base::Char8,
}
