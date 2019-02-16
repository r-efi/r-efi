//! File Protocol
//!
//! Provides an interface to interact with both files and directories. This
//! protocol is typically obtained via an EFI_SIMPLE_FILE_SYSTEM protocol or
//! via another EFI_FILE_PROTOCOL.

pub const REVISION: u64 = 0x0001_0000;
pub const REVISION2: u64 = 0x0002_0000;

#[repr(C)]
#[derive(Debug)]
pub struct FileIoToken {
    pub event: crate::base::Event,
    pub status: crate::base::Status,
    pub buffer_size: u64,
    pub buffer: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct Protocol {
    pub revision: u64,
    pub open: eficall!{fn(
        *mut Protocol,
        *mut *mut Protocol,
        *mut u16,
        u64,
        u64,
    ) -> crate::base::Status},
    pub close: eficall!{fn(
        *mut Protocol,
    ) -> crate::base::Status},
    pub delete: eficall!{fn(
        *mut Protocol,
    ) -> crate::base::Status},
    pub read: eficall!{fn(
        *mut Protocol,
        *mut usize,
        *mut core::ffi::c_void,
    ) -> crate::base::Status},
    pub write: eficall!{fn(
        *mut Protocol,
        *mut usize,
        *mut core::ffi::c_void,
    ) -> crate::base::Status},
    pub get_position: eficall!{fn(
        *mut Protocol,
        *mut u64,
    ) -> crate::base::Status},
    pub set_position: eficall!{fn(
        *mut Protocol,
        u64,
    ) -> crate::base::Status},
    pub get_info: eficall!{fn(
        *mut Protocol,
        *mut crate::base::Guid,
        *mut usize,
        *mut core::ffi::c_void,
    ) -> crate::base::Status},
    pub set_info: eficall!{fn(
        *mut Protocol,
        *mut crate::base::Guid,
        usize,
        *mut core::ffi::c_void,
    ) -> crate::base::Status},
    pub flush: eficall!{fn(
        *mut Protocol,
    ) -> crate::base::Status},
    pub open_ex: eficall!{fn(
        *mut Protocol,
        *mut *mut Protocol,
        *mut u16,
        u64,
        u64,
        *mut FileIoToken,
    ) -> crate::base::Status},
    pub read_ex: eficall!{fn(
        *mut Protocol,
        *mut FileIoToken,
    ) -> crate::base::Status},
    pub write_ex: eficall!{fn(
        *mut Protocol,
        *mut FileIoToken,
    ) -> crate::base::Status},
    pub flush_ex: eficall!{fn(
        *mut Protocol,
        *mut FileIoToken,
    ) -> crate::base::Status},
}
