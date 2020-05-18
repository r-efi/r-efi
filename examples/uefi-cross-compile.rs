// Bare UEFI Compilation
//
// This is a copy of a cross-compile test from the upstream rust repository. It
// is used to verify cross-compilation of UEFI targets works. It is included
// verbatim here to make sure we can run tests against it and get notifications
// when it breaks.

// -----------------------------------------------------------------------------
// COPY FROM `rust-lang/rust/src/test/codegen/uefi-cross-compile.rs`
// -----------------------------------------------------------------------------

// Checks whether UEFI targets cross-compile successfully.
//
// This test contains a simple UEFI program that prints "Hello World!\n" when
// run from the UEFI shell (or from any other UEFI environment). This program
// is not run as part of the test. The test merely verifies the
// cross-compilation does not fail and an entry-point is emitted.
//
// The imported definitions from the UEFI specification are intentionally left
// incomplete. Only the bits that are actually used by this test are defined.

// min-llvm-version 9.0

// compile-flags: --target x86_64-unknown-uefi

#![feature(abi_efiapi, lang_items, no_core)]
#![no_core]
#![no_main]

#[lang = "sized"]
pub trait Sized {}
#[lang = "freeze"]
pub trait Freeze {}
#[lang = "copy"]
pub trait Copy {}

impl<T: ?Sized> Copy for *mut T {}

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    pub reset: fn(),
    pub output_string: extern "efiapi" fn(*mut SimpleTextOutputProtocol, *mut usize) -> usize,
    pub test_string: fn(),
    pub query_mode: fn(),
    pub set_mode: fn(),
    pub set_attribute: fn(),
    pub clear_screen: fn(),
    pub set_cursor_position: fn(),
    pub enable_cursor: fn(),
    pub mode: *mut usize,
}

#[repr(C)]
pub struct SystemTable {
    pub hdr: [u8; 24],
    pub firmware_vendor: *mut usize,
    pub firmware_revision: u32,

    pub console_in_handle: *mut usize,
    pub con_in: *mut usize,
    pub console_out_handle: *mut usize,
    pub con_out: *mut SimpleTextOutputProtocol,
    pub standard_error_handle: *mut usize,
    pub std_err: *mut usize,

    pub runtime_services: *mut usize,
    pub boot_services: *mut usize,

    pub number_of_table_entries: usize,
    pub configuration_table: *mut usize,
}

// CHECK: define win64cc i64 @efi_main{{.*}}
#[export_name = "efi_main"]
pub extern "efiapi" fn main(_h: *mut usize, st: *mut SystemTable) -> usize {
    let s = [
        0x0048u16, 0x0065u16, 0x006cu16, 0x006cu16, 0x006fu16, // "Hello"
        0x0020u16, //                                             " "
        0x0057u16, 0x006fu16, 0x0072u16, 0x006cu16, 0x0064u16, // "World"
        0x0021u16, //                                             "!"
        0x000au16, //                                             "\n"
        0x0000u16, //                                             NUL
    ];
    let p = &s as *const [u16; 14] as *mut usize;

    let r = unsafe { ((*(*st).con_out).output_string)((*st).con_out, p) };

    // This immediately returns after printing to standard output. This works
    // fine in EfiShell, but might cause immediate screen-redraws when run by
    // the BootManager. In that case, for debugging purposes, simply add an
    // infinite loop here:
    //
    // loop {}

    return r;
}
