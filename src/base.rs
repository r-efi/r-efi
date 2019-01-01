//! UEFI Base Environment
//!
//! This module defines the base environment for UEFI development. It provides types and macros as
//! declared in the UEFI specification, as well as de-facto standard additions provided by the
//! reference implementation by Intel.
//!
//! # Target Configuration
//!
//! Wherever possible, native rust types are used to represent their UEFI counter-parts. However,
//! this means the ABI depends on the implementation of said rust types. Hence, native rust types
//! are only used where rust supports a stable ABI of said types, and their ABI matches the ABI
//! defined by the UEFI specification.
//!
//! Nevertheless, even if the ABI of a specific type is marked stable, this does not imply that it
//! is the same across architectures. For instance, rust's `u64` type has the same binary
//! representation as the `UINT64` type in UEFI. But this does not imply that it has the same
//! binary representation on `x86_64` and on `ppc64be`. As a result of this, the compilation of
//! this module is tied to the target-configuration you passed to the rust compiler. Wherever
//! possible and reasonable, any architecture differences are abstracted, though. This means that
//! in most cases you can use this module even though your target-configuration might not match
//! the native UEFI target-configuration.
//!
//! The recommend way to compile your code, is to use the native target-configuration for UEFI.
//! These configurations are not necessarily included in the upstream rust compiler. Hence, you
//! might have to craft one yourself. This project contains target-configurations for all targets
//! defined by the UEFI specification and supported by rust.
//!
//! However, there are situations where you want to access UEFI data from a non-native host. For
//! instance, a UEFI boot loader might store data in boot variables, formatted according to types
//! declared in the UEFI specification. An OS booted thereafter might want to access these
//! variables, but it might be compiled with a different target-configuration than the UEFI
//! environment that it was booted from. A similar situation occurs when you call UEFI runtime
//! functions from your OS. In all those cases, you should very likely be able to use this module
//! to interact with UEFI as well. This is, because most bits of the target-configuration of UEFI
//! and your OS very likely match. In fact, to figure out whether this is safe, you need to make
//! sure that the rust ABI would match in both target-configurations. If it is, all other details
//! are handled within this module just fine.
//!
//! In case of doubt, contact us!
//!
//! # Core Primitives
//!
//! Several of the UEFI primitives are represented by native Rust. These have no type aliases or
//! other definitions here, but you are recommended to use native rust directly. These include:
//!
//!  * `NULL`, `void *`: Void pointers have a native rust implementation in
//!                      [`c_void`](core::ffi::c_void). `NULL` is represented through
//!                      [`null`](core::ptr::null) and [`is_null()`](core::ptr) for
//!                      all pointer types.
//!  * `uint8_t`..`uint64_t`,
//!    `int8_t`..`int64_t`: Fixed-size integers are represented by their native rust equivalents
//!                         (`u8`..`u64`, `i8`..`i64`).
//!
//!  * `UINTN`, `INTN`: Native-sized (or instruction-width sized) integers are represented by
//!                     their native rust equivalents (`usize`, `isize`).
//!
//! # UEFI Details
//!
//! The UEFI Specification describes its target environments in detail. Each supported
//! architecture has a separate section with details on calling conventions, CPU setup, and more.
//! You are highly recommended to conduct the UEFI Specification for details on the programming
//! environment. Following a summary of key parts relevant to rust developers:
//!
//!  * Similar to rust, integers are either fixed-size, or native size. This maps nicely to the
//!    native rust types. The common `long`, `int`, `short` types known from ISO-C are not used.
//!    Whenever you refer to memory (either pointing to it, or remember the size of a memory
//!    block), the native size integers should be your tool of choice.
//!
//!  * Even though the CPU might run in any endianness, all stored data is little-endian. That
//!    means, if you encounter integers split into byte-arrays (e.g.,
//!    `CEfiDevicePathProtocol.length`), you must assume it is little-endian encoded. But if you
//!    encounter native integers, you must assume they are encoded in native endianness.
//!    For now the UEFI specification only defines little-endian architectures, hence this did not
//!    pop up as actual issue. Future extensions might change this, though.
//!
//!  * The Microsoft calling-convention is used. That is, all external calls to UEFI functions
//!    follow a calling convention that is very similar to that used on Microsoft Windows. All
//!    such ABI functions must be marked with the right calling-convention. The UEFI Specification
//!    defines some additional common rules for all its APIs, though. You will most likely not see
//!    any of these mentioned in the individual API documentions. So here is a short reminder:
//!
//!     * Pointers must reference physical-memory locations (no I/O mappings, no
//!       virtual addresses, etc.). Once ExitBootServices() was called, and the
//!       virtual address mapping was set, you must provide virtual-memory
//!       locations instead.
//!     * Pointers must be correctly aligned.
//!     * NULL is disallowed, unless explicitly mentioned otherwise.
//!     * Data referenced by pointers is undefined on error-return from a
//!       function.
//!     * You must not pass data larger than native-size (sizeof(CEfiUSize)) on
//!       the stack. You must pass them by reference.
//!
//!  * Stack size is at least 128KiB and 16-byte aligned. All stack space might be marked
//!    non-executable! Once ExitBootServices() was called, you must guarantee at least 4KiB of
//!    stack space, 16-byte aligned for all runtime services you call.
//!    Details might differ depending on architectures. But the numbers here should serve as
//!    ball-park figures.

// Target Architecture
//
// The UEFI Specification explicitly lists all supported target architectures. While external
// implementors are free to port UEFI to other targets, we need information on the target
// architecture to successfully compile for it. This includes calling-conventions, register
// layouts, endianness, and more. Most of these details are hidden in the rust-target-declaration.
// However, some details are still left to the actual rust code.
//
// This initial check just makes sure the compilation is halted with a suitable error message if
// the target architecture is not supported.
//
// We try to minimize conditional compilations as much as possible. A simple search for
// `target_arch` should reveal all uses throughout the code-base. If you add your target to this
// error-check, you must adjust all other uses as well.
//
// Similarly, UEFI only defines configurations for little-endian architectures so far. Several
// bits of the specification are thus unclear how they would be applied on big-endian systems. We
// therefore mark it as unsupported. If you override this, you are on your own.
#[cfg(not(any(target_arch = "arm",
              target_arch = "aarch64",
              target_arch = "x86",
              target_arch = "x86_64")))]
compile_error!("The target architecture is not supported.");
#[cfg(not(any(target_endian = "little")))]
compile_error!("The target endianness is not supported.");

use core::convert::TryFrom;

// eficall_arch!()
//
// This macro is the architecture-dependent implementation of eficall!(). See the documentation of
// the eficall!() macro for a description. We need to split the exported wrapper from the internal
// backend to make rustdoc attach to the right symbol.

#[cfg(target_arch = "arm")]
macro_rules! eficall_arch {
    (fn $in:tt $(-> $out:ty)?) => { extern "aapcs" fn $in $( -> $out )? };
}

// XXX: Rust does not define aapcs64, yet. Once it does, we should switch to it, rather than
//      referring to the system default.
#[cfg(target_arch = "aarch64")]
macro_rules! eficall_arch {
    (fn $in:tt $(-> $out:ty)?) => { extern "C" fn $in $( -> $out )? };
}

#[cfg(target_arch = "x86")]
macro_rules! eficall_arch {
    (fn $in:tt $(-> $out:ty)?) => { extern "cdecl" fn $in $( -> $out )? };
}

#[cfg(target_arch = "x86_64")]
macro_rules! eficall_arch {
    (fn $in:tt $(-> $out:ty)?) => { extern "win64" fn $in $( -> $out )? };
}

#[cfg(not(any(target_arch = "arm",
              target_arch = "aarch64",
              target_arch = "x86",
              target_arch = "x86_64")))]
macro_rules! eficall_arch {
    (fn $in:tt $(-> $out:ty)?) => { extern "C" fn $in $( -> $out )? };
}

/// Annotate function with UEFI calling convention
///
/// This macro takes a function-declaration as argument and produces the same function-declaration
/// but annotated with the correct calling convention. Since the default `extern "C"` annotation
/// depends on your compiler defaults, we cannot use it. Instead, this macro selects the default
/// for your target platform.
///
/// # Calling Conventions
///
/// The UEFI specification defines the calling convention for each platform individually. It
/// usually refers to other standards for details, but adds some restrictions on top. As of this
/// writing, it mentions:
///
///  * aarch32 / arm: The `aapcs` calling-convention is used. It is native to aarch32 and described
///                   in a document called
///                   "Procedure Call Standard for the ARM Architecture". It is openly distributed
///                   by ARM and widely known under the keyword `aapcs`.
///  * aarch64: The `aapcs64` calling-convention is used. It is native to aarch64 and described in
///             a document called
///             "Procedure Call Standard for the ARM 64-bit Architecture (AArch64)". It is openly
///             distributed by ARM and widely known under the keyword `aapcs64`.
///  * ia-64: The "P64 C Calling Convention" as described in the
///           "Itanium Software Conventions and Runtime Architecture Guide". It is also
///           standardized in the "Intel Itanium SAL Specification".
///  * RISC-V: The "Standard RISC-V C Calling Convention" is used. The UEFI specification
///            describes it in detail, but also refers to the official RISC-V resources for
///            detailed information.
///  * x86 / ia-32: The `cdecl` C calling convention is used. Originated in the C Language and
///                 originally tightly coupled to C specifics. Unclear whether a formal
///                 specification exists (does anyone know?). Most compilers support it under the
///                 `cdecl` keyword, and in nearly all situations it is the default on x86.
///  * x86_64 / amd64 / x64: The `win64` calling-convention is used. It is similar to the `sysv64`
///                          convention that is used on most non-windows x86_64 systems, but not
///                          exactly the same. Microsoft provides open documentation on it. See
///                          MSDN "x64 Software Conventions -> Calling Conventions".
///                          The UEFI Specification does not directly refer to `win64`, but
///                          contains a full specification of the calling convention itself.
///
/// Note that in most cases the UEFI Specification adds several more restrictions on top of the
/// common calling-conventions. These restrictions usually do not affect how the compiler will lay
/// out the function calls. Instead, it usually only restricts the set of APIs that are allowed in
/// UEFI. Therefore, most compilers already support the calling conventions used on UEFI.
///
/// # Variadics
///
/// For some reason, the rust compiler allows variadics only in combination with the `"C"` calling
/// convention, even if the selected calling-convention matches what `"C"` would select on the
/// target platform. Hence, we do not support variadics so far. Luckily, all of the UEFI functions
/// that use variadics are wrappers around more low-level accessors, so they are not necessarily
/// required.
#[macro_export]
macro_rules! eficall {
    ($($arg:tt)*) => { eficall_arch!($($arg)*) };
}

/// Boolean Type
///
/// This boolean type works very similar to the rust primitive type of [`bool`]. However, the rust
/// primitive type has no stable ABI, hence we provide this type to represent booleans on the FFI
/// interface.
///
/// UEFI defines booleans to be 1-byte integers, which can only have the values of `0` or `1`.
/// This enum provides the equivalent definitions as [`Boolean::False`] and [`Boolean::True`].
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Boolean {
    False = 0u8,
    True = 1u8,
}

/// Single-byte Character Type
///
/// The `Char8` type represents single-byte characters. UEFI defines them to be ASCII compatible,
/// using the ISO-Latin-1 character set.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Char8(u8);

/// Dual-byte Character Type
///
/// The `Char16` type represents dual-byte characters. UEFI defines them to be UCS-2 encoded.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Char16(u16);

pub enum TryFromCharError {
    Surrogate,
    Private,
    OutOfRange,
}

/// Status Codes
///
/// UEFI uses the `Status` type to represent all kinds of status codes. This includes return codes
/// from functions, but also complex state of different devices and drivers. It is a simple
/// `usize`, but wrapped in a rust-type to allow us to implement helpers on this type. Depending
/// on the context, different state is stored in it. Note that it is always binary compatible to a
/// usize!
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Status(usize);

/// Object Handles
///
/// Handles represent access to an opaque object. Handles are untyped by default, but get a
/// meaning when you combine them with an interface. Internally, they are simple void pointers. It
/// is the UEFI driver model that applies meaning to them.
pub type Handle = *mut core::ffi::c_void;

/// Event Objects
///
/// Event objects represent hooks into the main-loop of a UEFI environment. They allow to register
/// callbacks, to be invoked when a specific event happens. In most cases you use events to
/// register timer-based callbacks, as well as chaining events together. Internally, they are
/// simple void pointers. It is the UEFI task management that applies meaning to them.
pub type Event = *mut core::ffi::c_void;

/// Logical Block Addresses
///
/// The LBA type is used to denote logical block addresses of block devices. It is a simple 64-bit
/// integer, that is used to denote addresses when working with block devices.
pub type Lba = u64;

/// Thread Priority Levels
///
/// The process model of UEFI systems is highly simplified. Priority levels are used to order
/// execution of pending tasks. The TPL type denotes a priority level of a specific task. The
/// higher the number, the higher the priority. It is a simple integer type, but its range is
/// usually highly restricted. The UEFI task management provides constants and accessors for TPLs.
pub type Tpl = usize;

/// Physical Memory Address
///
/// A simple 64bit integer containing a physical memory address.
pub type PhysicalAddress = u64;

/// Virtual Memory Address
///
/// A simple 64bit integer containing a virtual memory address.
pub type VirtualAddress = u64;

/// Application Entry Point
///
/// This type defines the entry-point of UEFI applications. It is ABI and cannot be changed.
/// Whenever you load UEFI images, the entry-point is called with this signature.
///
/// In most cases the UEFI image (or application) is unloaded when control returns from the entry
/// point. In case of UEFI drivers, they can request to stay loaded until an explicit unload.
///
/// The system table is provided as mutable pointer. This is, because there is no guarantee that
/// timer interrupts do not modify the table. Furthermore, exiting boot services causes several
/// modifications on that table. And lastly, the system table lives longer than the function
/// invocation, if invoked as an UEFI driver.
/// In most cases it is perfectly fine to cast the pointer to a real rust reference. However, this
/// should be an explicit decision by the caller.
pub type ImageEntryPoint = fn(Handle, *mut crate::system::SystemTable) -> Status;

/// Globally Unique Identifiers
///
/// The `Guid` type represents globally unique identifiers as defined by RFC-4122 (i.e., only the
/// `10x` variant is used). The type must be 64-bit aligned.
///
/// Note that only the binary representation of Guids is stable. You are highly recommended to
/// interpret Guids as 128bit integers.
///
/// UEFI uses the Microsoft-style Guid format. Hence, a lot of documentation and code refers to
/// these Guids. If you thusly cannot treat Guids as 128-bit integers, this Guid type allows you
/// to access the individual fields of the Microsoft-style Guid. A reminder of the Guid encoding:
///
/// ```text
///    0                   1                   2                   3
///    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///   |                          time_low                             |
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///   |       time_mid                |         time_hi_and_version   |
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///   |clk_seq_hi_res |  clk_seq_low  |         node (0-1)            |
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///   |                         node (2-5)                            |
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
///
/// The individual fields are encoded as big-endian (network-byte-order). The Guid structure
/// allows you direct access to these fields. Make sure to convert endianness when accessing the
/// data. Data stored in Guid objects must be considered big-endian.
#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct Guid {
    pub time_low: u32,
    pub time_mid: u16,
    pub time_hi_and_version: u16,
    pub clk_seq_hi_res: u8,
    pub clk_seq_low: u8,
    pub node: [u8; 6],
}

impl PartialEq<bool> for Boolean {
    fn eq(&self, other: &bool) -> bool {
        *other == (*self).into()
    }
}

impl From<bool> for Boolean {
    fn from(b: bool) -> Self {
        match b {
            false => Boolean::False,
            true => Boolean::True,
        }
    }
}

impl From<Boolean> for bool {
    fn from(b: Boolean) -> Self {
        match b {
            Boolean::False  => false,
            Boolean::True   => true,
        }
    }
}

impl From<Char8> for char {
    fn from(c: Char8) -> Self {
        // A Char8 is always a valid Unicode codepoint.
        char::from(c.0)
    }
}

impl From<Char16> for char {
    fn from(c: Char16) -> Self {
        // A Char16 is always a valid Unicode codepoint.
        char::try_from(c.0 as u32).unwrap()
    }
}

impl TryFrom<char> for Char8 {
    type Error = TryFromCharError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        // A Char8 is any 8-bit Unicode codepoint, corresponding to
        // the ISO-Latin-1 encoding.
        match c as u32 {
            0x00 ... 0xff => Ok(Char8(c as u8)),
            _ => Err(TryFromCharError::OutOfRange),
        }
    }
}

impl TryFrom<char> for Char16 {
    type Error = TryFromCharError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        // A Char16 is any Unicode codepoint in the basic multilingual
        // plane, except any surrogate codepoint or a codepoint reserved
        // for private use.
        match c as u32 {
            0x0000 ... 0xd7ff => Ok(Char16(c as u16)),
            0xd800 ... 0xdfff => Err(TryFromCharError::Surrogate),
            0xe000 ... 0xf8ff => Err(TryFromCharError::Private),
            0xf900 ... 0xffff => Ok(Char16(c as u16)),
            _ => Err(TryFromCharError::OutOfRange),
        }
    }
}

impl Status {
    const WIDTH: usize = 8usize * core::mem::size_of::<Status>();
    const MASK: usize = 0xc0 << (Status::WIDTH - 8);
    const ERROR_MASK: usize = 0x80 << (Status::WIDTH - 8);
    const WARNING_MASK: usize = 0x00 << (Status::WIDTH - 8);

    /// Success Code
    ///
    /// This code represents a successfull function invocation. Its value is guaranteed to be 0.
    /// However, note that warnings are considered success as well, so this is not the only code
    /// that can be returned by UEFI functions on success. However, in nearly all situations
    /// warnings are not allowed, so the effective result will be SUCCESS.
    pub const SUCCESS: Status = Status::from_usize(0);

    // List of predefined error codes
    pub const LOAD_ERROR:                   Status = Status::from_usize( 1 | Status::ERROR_MASK);
    pub const INVALID_PARAMETER:            Status = Status::from_usize( 2 | Status::ERROR_MASK);
    pub const UNSUPPORTED:                  Status = Status::from_usize( 3 | Status::ERROR_MASK);
    pub const BAD_BUFFER_SIZE:              Status = Status::from_usize( 4 | Status::ERROR_MASK);
    pub const BUFFER_TOO_SMALL:             Status = Status::from_usize( 5 | Status::ERROR_MASK);
    pub const NOT_READY:                    Status = Status::from_usize( 6 | Status::ERROR_MASK);
    pub const DEVICE_ERROR:                 Status = Status::from_usize( 7 | Status::ERROR_MASK);
    pub const WRITE_PROTECTED:              Status = Status::from_usize( 8 | Status::ERROR_MASK);
    pub const OUT_OF_RESOURCES:             Status = Status::from_usize( 9 | Status::ERROR_MASK);
    pub const VOLUME_CORRUPTED:             Status = Status::from_usize(10 | Status::ERROR_MASK);
    pub const VOLUME_FULL:                  Status = Status::from_usize(11 | Status::ERROR_MASK);
    pub const NO_MEDIA:                     Status = Status::from_usize(12 | Status::ERROR_MASK);
    pub const MEDIA_CHANGED:                Status = Status::from_usize(13 | Status::ERROR_MASK);
    pub const NOT_FOUND:                    Status = Status::from_usize(14 | Status::ERROR_MASK);
    pub const ACCESS_DENIED:                Status = Status::from_usize(15 | Status::ERROR_MASK);
    pub const NO_RESPONSE:                  Status = Status::from_usize(16 | Status::ERROR_MASK);
    pub const NO_MAPPING:                   Status = Status::from_usize(17 | Status::ERROR_MASK);
    pub const TIMEOUT:                      Status = Status::from_usize(18 | Status::ERROR_MASK);
    pub const NOT_STARTED:                  Status = Status::from_usize(19 | Status::ERROR_MASK);
    pub const ALREADY_STARTED:              Status = Status::from_usize(20 | Status::ERROR_MASK);
    pub const ABORTED:                      Status = Status::from_usize(21 | Status::ERROR_MASK);
    pub const ICMP_ERROR:                   Status = Status::from_usize(22 | Status::ERROR_MASK);
    pub const TFTP_ERROR:                   Status = Status::from_usize(23 | Status::ERROR_MASK);
    pub const PROTOCOL_ERROR:               Status = Status::from_usize(24 | Status::ERROR_MASK);
    pub const INCOMPATIBLE_VERSION:         Status = Status::from_usize(25 | Status::ERROR_MASK);
    pub const SECURITY_VIOLATION:           Status = Status::from_usize(26 | Status::ERROR_MASK);
    pub const CRC_ERROR:                    Status = Status::from_usize(27 | Status::ERROR_MASK);
    pub const END_OF_MEDIA:                 Status = Status::from_usize(28 | Status::ERROR_MASK);
    pub const END_OF_FILE:                  Status = Status::from_usize(31 | Status::ERROR_MASK);
    pub const INVALID_LANGUAGE:             Status = Status::from_usize(32 | Status::ERROR_MASK);
    pub const COMPROMISED_DATA:             Status = Status::from_usize(33 | Status::ERROR_MASK);
    pub const IP_ADDRESS_CONFLICT:          Status = Status::from_usize(34 | Status::ERROR_MASK);
    pub const HTTP_ERROR:                   Status = Status::from_usize(35 | Status::ERROR_MASK);

    // List of predefined warning codes
    pub const WARN_UNKNOWN_GLYPH:           Status = Status::from_usize( 1 | Status::WARNING_MASK);
    pub const WARN_DELETE_FAILURE:          Status = Status::from_usize( 2 | Status::WARNING_MASK);
    pub const WARN_WRITE_FAILURE:           Status = Status::from_usize( 3 | Status::WARNING_MASK);
    pub const WARN_BUFFER_TOO_SMALL:        Status = Status::from_usize( 4 | Status::WARNING_MASK);
    pub const WARN_STALE_DATA:              Status = Status::from_usize( 5 | Status::WARNING_MASK);
    pub const WARN_FILE_SYSTEM:             Status = Status::from_usize( 6 | Status::WARNING_MASK);
    pub const WARN_RESET_REQUIRED:          Status = Status::from_usize( 7 | Status::WARNING_MASK);

    /// Create Status Code from Integer
    ///
    /// This takes the literal value of a status code and turns it into a `Status` object. Note
    /// that we want it as `const fn` so we cannot use `core::convert::From`.
    pub const fn from_usize(v: usize) -> Status {
        Status(v)
    }

    fn value(&self) -> usize {
        self.0
    }

    fn mask(&self) -> usize {
        self.value() & Status::MASK
    }

    /// Check whether this is an error
    ///
    /// This returns true if the given status code is considered an error. Errors mean the
    /// operation did not success, nor produce any valuable output. Output parameters must be
    /// considered invalid if an error was returned. That is, its content is not well defined.
    pub fn is_error(&self) -> bool {
        self.mask() == Status::ERROR_MASK
    }

    /// Check whether this is a warning
    ///
    /// This returns true if the given status code is considered a warning. Warnings are to be
    /// treated as success, but might indicate data loss or other device errors. However, if an
    /// operation returns with a warning code, it must be considered successfull, and the output
    /// parameters are valid.
    pub fn is_warning(&self) -> bool {
        self.value() != 0 && self.mask() == Status::WARNING_MASK
    }
}

impl Guid {
    /// Initialize a Guid from its individual fields in native endianness
    ///
    /// This is the most basic initializer of a Guid object. It takes the individual fields in
    /// native endian and creates the big-endian Guid object with it.
    pub const fn from_native(
        time_low: u32,
        time_mid: u16,
        time_hi_and_version: u16,
        clk_seq_hi_res: u8,
        clk_seq_low: u8,
        node: &[u8; 6],
    ) -> Guid {
        Guid {
            time_low: time_low.to_be(),
            time_mid: time_mid.to_be(),
            time_hi_and_version: time_hi_and_version.to_be(),
            clk_seq_hi_res: clk_seq_hi_res.to_be(),
            clk_seq_low: clk_seq_low.to_be(),
            node: *node,
        }
    }

    /// Initialize a Guid from its individual fields as given in the specification
    ///
    /// This function initializes a Guid object given the individual fields as specified in the
    /// UEFI specification. That is, if you simply copy the literals from the specification into
    /// your code, this function will correctly initialize the Guid object.
    ///
    /// The UEFI specification provides definitions of Guids as a set of integer literals. They
    /// are meant to be directly assigned to the corresponding Guid fields. However, the
    /// specification assumes little-endian systems, therefore the literals are provided in
    /// big-endian format, so the conversion can be skipped. This will not work on big-endian
    /// systems, though. Hence, this function applies the required conversions.
    pub const fn from_spec(
        time_low: u32,
        time_mid: u16,
        time_hi_and_version: u16,
        clk_seq_hi_res: u8,
        clk_seq_low: u8,
        node: &[u8; 6],
    ) -> Guid {
        // The literals are given in inverted byte-order in the specification. Revert this to
        // native order and then simply use the basic constructor.
        Guid::from_native(
            time_low.swap_bytes(),
            time_mid.swap_bytes(),
            time_hi_and_version.swap_bytes(),
            clk_seq_hi_res.swap_bytes(),
            clk_seq_low.swap_bytes(),
            node,
        )
    }

    /// Access a Guid as raw byte array
    ///
    /// This provides access to a Guid through a byte array. It is a simple re-interpretation of
    /// the Guid value as a 128-bit byte array. No conversion is performed. This is a simple cast.
    pub fn as_bytes(&self) -> &[u8; 16] {
        unsafe {
            core::mem::transmute::<&Guid, &[u8; 16]>(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, size_of};

    #[test]
    fn types() {
        //
        // Booleans
        //

        assert_eq!(size_of::<Boolean>(), 1);
        assert_eq!(align_of::<Boolean>(), 1);
        assert_eq!(Boolean::False, false);
        assert_eq!(Boolean::True, true);
        assert_ne!(Boolean::False, Boolean::True);

        //
        // Char8 / Char16
        //

        assert_eq!(size_of::<Char8>(), 1);
        assert_eq!(align_of::<Char8>(), 1);
        assert_eq!(size_of::<Char16>(), 2);
        assert_eq!(align_of::<Char16>(), 2);

        assert_eq!(size_of::<Char8>(), size_of::<u8>());
        assert_eq!(align_of::<Char8>(), align_of::<u8>());
        assert_eq!(size_of::<Char16>(), size_of::<u16>());
        assert_eq!(align_of::<Char16>(), align_of::<u16>());

        //
        // Status
        //

        assert_eq!(size_of::<Status>(), size_of::<usize>());
        assert_eq!(align_of::<Status>(), align_of::<usize>());

        //
        // Handles / Events
        //

        assert_eq!(size_of::<Handle>(), size_of::<usize>());
        assert_eq!(align_of::<Handle>(), align_of::<usize>());
        assert_eq!(size_of::<Event>(), size_of::<usize>());
        assert_eq!(align_of::<Event>(), align_of::<usize>());

        assert_eq!(size_of::<Handle>(), size_of::<*mut ()>());
        assert_eq!(align_of::<Handle>(), align_of::<*mut ()>());
        assert_eq!(size_of::<Event>(), size_of::<*mut ()>());
        assert_eq!(align_of::<Event>(), align_of::<*mut ()>());

        //
        // Lba / Tpl
        //

        assert_eq!(size_of::<Lba>(), size_of::<u64>());
        assert_eq!(align_of::<Lba>(), align_of::<u64>());
        assert_eq!(size_of::<Tpl>(), size_of::<usize>());
        assert_eq!(align_of::<Tpl>(), align_of::<usize>());

        //
        // PhysicalAddress / VirtualAddress
        //

        assert_eq!(size_of::<PhysicalAddress>(), size_of::<u64>());
        assert_eq!(align_of::<PhysicalAddress>(), align_of::<u64>());
        assert_eq!(size_of::<VirtualAddress>(), size_of::<u64>());
        assert_eq!(align_of::<VirtualAddress>(), align_of::<u64>());

        //
        // ImageEntryPoint
        //

        assert_eq!(size_of::<ImageEntryPoint>(), size_of::<fn ()>());
        assert_eq!(align_of::<ImageEntryPoint>(), align_of::<fn ()>());

        //
        // Guid
        //

        assert_eq!(size_of::<Guid>(), 16);
        assert_eq!(align_of::<Guid>(), 8);
    }

    #[test]
    fn eficall() {
        //
        // Make sure the eficall!{} macro can deal with all kinds of function callbacks.
        //

        let _: eficall!{fn()};
        let _: eficall!{fn(i32)};
        let _: eficall!{fn(i32) -> i32};
        let _: eficall!{fn(i32, i32) -> (i32, i32)};
    }
}
