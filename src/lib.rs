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
// work on them. This separation is private to this implementation. We export all symbols in a
// global namespace below. The reason for this is that we want this module to be a 1-to-1 mapping
// to the UEFI specification. We should not apply any policies on top, unless they are hidden in
// our implementation.
//
// Thus, all base types and all symbols from the system definition are exported directly in the
// crate root. This allows mirroring the entire UEFI namespace as given in the specification.
//
// Note that the individual protocols are put into submodules. The specification does this in most
// parts as well (by prefixing all symbols). This is not true in all cases, as the specification
// suffers from lack of namespaces in the reference C language. However, we decided to namespace
// the remaining bits as well, for better consistency throughout the API.
#[macro_use]
mod base;
#[macro_use]
mod system;

//
// Re-export base
//

pub use self::base::Boolean;
pub use self::base::Char8;
pub use self::base::Char16;
pub use self::base::Guid;
pub use self::base::Status;
pub use self::base::Handle;
pub use self::base::Event;
pub use self::base::Lba;
pub use self::base::Tpl;
pub use self::base::PhysicalAddress;
pub use self::base::VirtualAddress;
pub use self::base::ImageEntryPoint;

//
// Re-export system
//

pub use self::system::TIME_ADJUST_DAYLIGHT;
pub use self::system::TIME_IN_DAYLIGHT;
pub use self::system::UNSPECIFIED_TIMEZONE;
pub use self::system::Time;
pub use self::system::TimeCapabilities;

pub use self::system::VARIABLE_NON_VOLATILE;
pub use self::system::VARIABLE_BOOTSERVICE_ACCESS;
pub use self::system::VARIABLE_RUNTIME_ACCESS;
pub use self::system::VARIABLE_HARDWARE_ERROR_RECORD;
pub use self::system::VARIABLE_AUTHENTICATED_WRITE_ACCESS;
pub use self::system::VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS;
pub use self::system::VARIABLE_APPEND_WRITE;
pub use self::system::VARIABLE_ENHANCED_AUTHENTICATED_ACCESS;
pub use self::system::VARIABLE_AUTHENTICATION_3_CERT_ID_SHA256;
pub use self::system::VariableAuthentication3CertId;
pub use self::system::VariableAuthentication;
pub use self::system::VariableAuthentication2;
pub use self::system::VARIABLE_AUTHENTICATION_3_TIMESTAMP_TYPE;
pub use self::system::VARIABLE_AUTHENTICATION_3_NONCE_TYPE;
pub use self::system::VariableAuthentication3;
pub use self::system::VariableAuthentication3Nonce;
pub use self::system::HARDWARE_ERROR_VARIABLE_GUID;

pub use self::system::OPTIONAL_POINTER;

pub use self::system::ResetType;

pub use self::system::CapsuleBlockDescriptorUnion;
pub use self::system::CapsuleBlockDescriptor;
pub use self::system::CAPSULE_FLAGS_PERSIST_ACROSS_RESET;
pub use self::system::CAPSULE_FLAGS_POPULATE_SYSTEM_TABLE;
pub use self::system::CAPSULE_FLAGS_INITIATE_RESET;
pub use self::system::CapsuleHeader;
pub use self::system::OS_INDICATIONS_BOOT_TO_FW_UI;
pub use self::system::OS_INDICATIONS_TIMESTAMP_REVOCATION;
pub use self::system::OS_INDICATIONS_FILE_CAPSULE_DELIVERY_SUPPORTED;
pub use self::system::OS_INDICATIONS_FMP_CAPSULE_SUPPORTED;
pub use self::system::OS_INDICATIONS_CAPSULE_RESULT_VAR_SUPPORTED;
pub use self::system::OS_INDICATIONS_START_OS_RECOVERY;
pub use self::system::OS_INDICATIONS_START_PLATFORM_RECOVERY;
pub use self::system::CAPSULE_REPORT_GUID;
pub use self::system::CapsuleResultVariableHeader;
pub use self::system::CapsuleResultVariableFMP;

pub use self::system::EVT_TIMER;
pub use self::system::EVT_RUNTIME;
pub use self::system::EVT_NOTIFY_WAIT;
pub use self::system::EVT_NOTIFY_SIGNAL;
pub use self::system::EVT_SIGNAL_EXIT_BOOT_SERVICES;
pub use self::system::EVT_SIGNAL_VIRTUAL_ADDRESS_CHANGE;
pub use self::system::EventNotify;
pub use self::system::EVENT_GROUP_EXIT_BOOT_SERVICES;
pub use self::system::EVENT_GROUP_VIRTUAL_ADDRESS_CHANGE;
pub use self::system::EVENT_GROUP_MEMORY_MAP_CHANGE;
pub use self::system::EVENT_GROUP_READY_TO_BOOT;
pub use self::system::EVENT_GROUP_RESET_SYSTEM;
pub use self::system::TimerDelay;
pub use self::system::TPL_APPLICATION;
pub use self::system::TPL_CALLBACK;
pub use self::system::TPL_NOTIFY;
pub use self::system::TPL_HIGH_LEVEL;

pub use self::system::AllocateType;
pub use self::system::MemoryType;
pub use self::system::MEMORY_UC;
pub use self::system::MEMORY_WC;
pub use self::system::MEMORY_WT;
pub use self::system::MEMORY_WB;
pub use self::system::MEMORY_UCE;
pub use self::system::MEMORY_WP;
pub use self::system::MEMORY_RP;
pub use self::system::MEMORY_XP;
pub use self::system::MEMORY_NV;
pub use self::system::MEMORY_MORE_RELIABLE;
pub use self::system::MEMORY_RO;
pub use self::system::MEMORY_RUNTIME;
pub use self::system::MEMORY_DESCRIPTOR_VERSION;
pub use self::system::MemoryDescriptor;

pub use self::system::InterfaceType;
pub use self::system::LocateSearchType;
pub use self::system::OPEN_PROTOCOL_BY_HANDLE_PROTOCOL;
pub use self::system::OPEN_PROTOCOL_GET_PROTOCOL;
pub use self::system::OPEN_PROTOCOL_TEST_PROTOCOL;
pub use self::system::OPEN_PROTOCOL_BY_CHILD_CONTROLLER;
pub use self::system::OPEN_PROTOCOL_BY_DRIVER;
pub use self::system::OPEN_PROTOCOL_EXCLUSIVE;
pub use self::system::OpenProtocolInformationEntry;

pub use self::system::ConfigurationTable;
pub use self::system::PROPERTIES_TABLE_GUID;
pub use self::system::PROPERTIES_TABLE_VERSION;
pub use self::system::PROPERTIES_RUNTIME_MEMORY_PROTECTION_NON_EXECUTABLE_PE_DATA;
pub use self::system::PropertiesTable;
pub use self::system::MEMORY_ATTRIBUTES_TABLE_GUID;
pub use self::system::MEMORY_ATTRIBUTES_TABLE_VERSION;
pub use self::system::MemoryAttributesTable;

pub use self::system::SPECIFICATION_REVISION;
pub use self::system::TableHeader;
pub use self::system::RUNTIME_SERVICES_SIGNATURE;
pub use self::system::RUNTIME_SERVICES_REVISION;
pub use self::system::RuntimeServices;
pub use self::system::BOOT_SERVICES_SIGNATURE;
pub use self::system::BOOT_SERVICES_REVISION;
pub use self::system::BootServices;
pub use self::system::SYSTEM_TABLE_SIGNATURE;
pub use self::system::SYSTEM_TABLE_REVISION_2_70;
pub use self::system::SYSTEM_TABLE_REVISION_2_60;
pub use self::system::SYSTEM_TABLE_REVISION_2_50;
pub use self::system::SYSTEM_TABLE_REVISION_2_40;
pub use self::system::SYSTEM_TABLE_REVISION_2_31;
pub use self::system::SYSTEM_TABLE_REVISION_2_30;
pub use self::system::SYSTEM_TABLE_REVISION_2_20;
pub use self::system::SYSTEM_TABLE_REVISION_2_10;
pub use self::system::SYSTEM_TABLE_REVISION_2_00;
pub use self::system::SYSTEM_TABLE_REVISION_1_10;
pub use self::system::SYSTEM_TABLE_REVISION_1_02;
pub use self::system::SystemTable;
