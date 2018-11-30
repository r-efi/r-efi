//! UEFI Reference Specification Protocol Constants and Definitions
//!
//! XXX
//!
//! # Examples
//!
//! To write free-standing UEFI applications, you need to disable the entry-point provided by rust
//! and instead provide your own. Most target-configurations look for a function called `efi_main`
//! during linking and set it as entry point. If you use the target-configurations provided with
//! this module, they will pick the function called `efi_main` as entry-point.
//!
//! The following example shows a minimal UEFI application. Note that, depending on your rust
//! compiler, you might need further hooks to make this compile. In particular, you most likely
//! need a panic-handler and compiler-intrinsics as well.
//!
//! ```no_run
//! #![no_main]
//! #![no_std]
//!
//! extern crate r_efi;
//!
//! #![no_mangle]
//! pub extern fn efi_main(h: r_efi::Handle, st: *mut r_efi::SystemTable) -> r_efi::Status {
//!     0
//! }
//! ```

// We use globals a lot, since UEFI defines many Guid-globals and other constants. However, we
// want to be able to initialize these globals with proper constructor functions. We thus use the
// unstable `const fn` feature of rust. The `min_const_fn` is a subset of `const_fn`, only
// providing the most basic of its features, which appear to be sufficient for our use-cases.
//
// Additionally, the `const_int_ops` feature makes the standard library integer conversion helpers
// const-safe as well.
//
// The `macro_at_most_once_rep` feature allows us to use the `?` qualifier in macros, also known
// as Kleene Operator. It is similar to the existing `+` and `*` qualifiers, but limits the
// repetition to 0 or 1. We could come by without it, but it seems stable enough to rely on it.
#![feature(min_const_fn)]
#![feature(const_int_ops)]
#![feature(macro_at_most_once_rep)]

// We use several features of `core`. This includes things like `core::ffi::c_void`. See the
// `base` module for details on the required target-configuration and the implications thereof.
// Including `core` does not imply that the host must be a native UEFI target. The generated code
// can still be used cross-target. Again, the `base` module describes this in detail.
extern crate core;

// Import the different core modules. We separate them into different modules to make it easier to
// work on them and describe what each part implements. This is different to the reference
// implementation, which uses a flat namespace due to its origins in the C language. For
// compatibility, we provide this flat namespace as well. See the `UEFI` submodule.
#[macro_use]
pub mod base;
#[macro_use]
pub mod system;

/// Flat EFI Namespace
///
/// The EFI namespace re-exports all symbols in a single, flat namespace. This allows mirroring
/// the entire EFI namespace as given in the specification and makes it easier to refer to them
/// with the same names as the reference C implementation.
///
/// Note that the individual protocols are put into submodules. The specification does this in
/// most parts as well (by prefixing all symbols). This is not true in all cases, as the
/// specification suffers from lack of namespaces in the reference C language. However, we decided
/// to namespace the remaining bits as well, for better consistency throughout the API. This
/// should be self-explanatory in nearly all cases.
pub mod efi {
    //
    // Re-export base
    //

    pub use crate::base::Boolean;
    pub use crate::base::Char8;
    pub use crate::base::Char16;
    pub use crate::base::Guid;
    pub use crate::base::Status;
    pub use crate::base::Handle;
    pub use crate::base::Event;
    pub use crate::base::Lba;
    pub use crate::base::Tpl;
    pub use crate::base::PhysicalAddress;
    pub use crate::base::VirtualAddress;
    pub use crate::base::ImageEntryPoint;

    //
    // Re-export system
    //

    pub use crate::system::TIME_ADJUST_DAYLIGHT;
    pub use crate::system::TIME_IN_DAYLIGHT;
    pub use crate::system::UNSPECIFIED_TIMEZONE;
    pub use crate::system::Time;
    pub use crate::system::TimeCapabilities;

    pub use crate::system::VARIABLE_NON_VOLATILE;
    pub use crate::system::VARIABLE_BOOTSERVICE_ACCESS;
    pub use crate::system::VARIABLE_RUNTIME_ACCESS;
    pub use crate::system::VARIABLE_HARDWARE_ERROR_RECORD;
    pub use crate::system::VARIABLE_AUTHENTICATED_WRITE_ACCESS;
    pub use crate::system::VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS;
    pub use crate::system::VARIABLE_APPEND_WRITE;
    pub use crate::system::VARIABLE_ENHANCED_AUTHENTICATED_ACCESS;
    pub use crate::system::VARIABLE_AUTHENTICATION_3_CERT_ID_SHA256;
    pub use crate::system::VariableAuthentication3CertId;
    pub use crate::system::VariableAuthentication;
    pub use crate::system::VariableAuthentication2;
    pub use crate::system::VARIABLE_AUTHENTICATION_3_TIMESTAMP_TYPE;
    pub use crate::system::VARIABLE_AUTHENTICATION_3_NONCE_TYPE;
    pub use crate::system::VariableAuthentication3;
    pub use crate::system::VariableAuthentication3Nonce;
    pub use crate::system::HARDWARE_ERROR_VARIABLE_GUID;

    pub use crate::system::OPTIONAL_POINTER;

    pub use crate::system::ResetType;

    pub use crate::system::CapsuleBlockDescriptorUnion;
    pub use crate::system::CapsuleBlockDescriptor;
    pub use crate::system::CAPSULE_FLAGS_PERSIST_ACROSS_RESET;
    pub use crate::system::CAPSULE_FLAGS_POPULATE_SYSTEM_TABLE;
    pub use crate::system::CAPSULE_FLAGS_INITIATE_RESET;
    pub use crate::system::CapsuleHeader;
    pub use crate::system::OS_INDICATIONS_BOOT_TO_FW_UI;
    pub use crate::system::OS_INDICATIONS_TIMESTAMP_REVOCATION;
    pub use crate::system::OS_INDICATIONS_FILE_CAPSULE_DELIVERY_SUPPORTED;
    pub use crate::system::OS_INDICATIONS_FMP_CAPSULE_SUPPORTED;
    pub use crate::system::OS_INDICATIONS_CAPSULE_RESULT_VAR_SUPPORTED;
    pub use crate::system::OS_INDICATIONS_START_OS_RECOVERY;
    pub use crate::system::OS_INDICATIONS_START_PLATFORM_RECOVERY;
    pub use crate::system::CAPSULE_REPORT_GUID;
    pub use crate::system::CapsuleResultVariableHeader;
    pub use crate::system::CapsuleResultVariableFMP;

    pub use crate::system::EVT_TIMER;
    pub use crate::system::EVT_RUNTIME;
    pub use crate::system::EVT_NOTIFY_WAIT;
    pub use crate::system::EVT_NOTIFY_SIGNAL;
    pub use crate::system::EVT_SIGNAL_EXIT_BOOT_SERVICES;
    pub use crate::system::EVT_SIGNAL_VIRTUAL_ADDRESS_CHANGE;
    pub use crate::system::EventNotify;
    pub use crate::system::EVENT_GROUP_EXIT_BOOT_SERVICES;
    pub use crate::system::EVENT_GROUP_VIRTUAL_ADDRESS_CHANGE;
    pub use crate::system::EVENT_GROUP_MEMORY_MAP_CHANGE;
    pub use crate::system::EVENT_GROUP_READY_TO_BOOT;
    pub use crate::system::EVENT_GROUP_RESET_SYSTEM;
    pub use crate::system::TimerDelay;
    pub use crate::system::TPL_APPLICATION;
    pub use crate::system::TPL_CALLBACK;
    pub use crate::system::TPL_NOTIFY;
    pub use crate::system::TPL_HIGH_LEVEL;

    pub use crate::system::AllocateType;
    pub use crate::system::MemoryType;
    pub use crate::system::MEMORY_UC;
    pub use crate::system::MEMORY_WC;
    pub use crate::system::MEMORY_WT;
    pub use crate::system::MEMORY_WB;
    pub use crate::system::MEMORY_UCE;
    pub use crate::system::MEMORY_WP;
    pub use crate::system::MEMORY_RP;
    pub use crate::system::MEMORY_XP;
    pub use crate::system::MEMORY_NV;
    pub use crate::system::MEMORY_MORE_RELIABLE;
    pub use crate::system::MEMORY_RO;
    pub use crate::system::MEMORY_RUNTIME;
    pub use crate::system::MEMORY_DESCRIPTOR_VERSION;
    pub use crate::system::MemoryDescriptor;

    pub use crate::system::InterfaceType;
    pub use crate::system::LocateSearchType;
    pub use crate::system::OPEN_PROTOCOL_BY_HANDLE_PROTOCOL;
    pub use crate::system::OPEN_PROTOCOL_GET_PROTOCOL;
    pub use crate::system::OPEN_PROTOCOL_TEST_PROTOCOL;
    pub use crate::system::OPEN_PROTOCOL_BY_CHILD_CONTROLLER;
    pub use crate::system::OPEN_PROTOCOL_BY_DRIVER;
    pub use crate::system::OPEN_PROTOCOL_EXCLUSIVE;
    pub use crate::system::OpenProtocolInformationEntry;

    pub use crate::system::ConfigurationTable;
    pub use crate::system::PROPERTIES_TABLE_GUID;
    pub use crate::system::PROPERTIES_TABLE_VERSION;
    pub use crate::system::PROPERTIES_RUNTIME_MEMORY_PROTECTION_NON_EXECUTABLE_PE_DATA;
    pub use crate::system::PropertiesTable;
    pub use crate::system::MEMORY_ATTRIBUTES_TABLE_GUID;
    pub use crate::system::MEMORY_ATTRIBUTES_TABLE_VERSION;
    pub use crate::system::MemoryAttributesTable;

    pub use crate::system::SPECIFICATION_REVISION;
    pub use crate::system::TableHeader;
    pub use crate::system::RUNTIME_SERVICES_SIGNATURE;
    pub use crate::system::RUNTIME_SERVICES_REVISION;
    pub use crate::system::RuntimeServices;
    pub use crate::system::BOOT_SERVICES_SIGNATURE;
    pub use crate::system::BOOT_SERVICES_REVISION;
    pub use crate::system::BootServices;
    pub use crate::system::SYSTEM_TABLE_SIGNATURE;
    pub use crate::system::SYSTEM_TABLE_REVISION_2_70;
    pub use crate::system::SYSTEM_TABLE_REVISION_2_60;
    pub use crate::system::SYSTEM_TABLE_REVISION_2_50;
    pub use crate::system::SYSTEM_TABLE_REVISION_2_40;
    pub use crate::system::SYSTEM_TABLE_REVISION_2_31;
    pub use crate::system::SYSTEM_TABLE_REVISION_2_30;
    pub use crate::system::SYSTEM_TABLE_REVISION_2_20;
    pub use crate::system::SYSTEM_TABLE_REVISION_2_10;
    pub use crate::system::SYSTEM_TABLE_REVISION_2_00;
    pub use crate::system::SYSTEM_TABLE_REVISION_1_10;
    pub use crate::system::SYSTEM_TABLE_REVISION_1_02;
    pub use crate::system::SystemTable;
}
