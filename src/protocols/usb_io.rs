//! USB I/O Protocol
//!
//! Provides services to manage and communicate with USB devices. This protocol is used by code,
//! typically drivers, running in the EFI boot services environment to access USB devices like
//! USB keyboards, mice, and mass storage devices.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x2b2f68d6,
    0x0cd2,
    0x44cf,
    0x8e,
    0x8b,
    &[0xbb, 0xa2, 0x0b, 0x1b, 0x5b, 0x75],
);

pub type DataDirection = u32;

pub const DATA_IN: DataDirection = 0x00000000;
pub const DATA_OUT: DataDirection = 0x00000001;
pub const NO_DATA: DataDirection = 0x00000002;

pub const NOERROR: u32 = 0x0000;
pub const ERR_NOTEXECUTE: u32 = 0x0001;
pub const ERR_STALL: u32 = 0x0002;
pub const ERR_BUFFER: u32 = 0x0004;
pub const ERR_BABBLE: u32 = 0x0008;
pub const ERR_NAK: u32 = 0x0010;
pub const ERR_CRC: u32 = 0x0020;
pub const ERR_TIMEOUT: u32 = 0x0040;
pub const ERR_BITSTUFF: u32 = 0x0080;
pub const ERR_SYSTEM: u32 = 0x0100;

pub type DeviceRequest = crate::industry::usb::DeviceRequest;
pub type DeviceDescriptor = crate::industry::usb::DeviceDescriptor;
pub type ConfigDescriptor = crate::industry::usb::ConfigDescriptor;
pub type InterfaceDescriptor = crate::industry::usb::InterfaceDescriptor;
pub type EndpointDescriptor = crate::industry::usb::EndpointDescriptor;

pub type AsyncUsbTransferCallback = unsafe extern "efiapi" fn(
    *mut core::ffi::c_void,
    usize,
    *mut core::ffi::c_void,
    u32,
) -> crate::base::Status;

pub type ProtocolControlTransfer = unsafe extern "efiapi" fn(
    *mut Protocol,
    *mut DeviceRequest,
    DataDirection,
    u32,
    *mut core::ffi::c_void,
    usize,
    *mut u32,
) -> crate::base::Status;

pub type ProtocolBulkTransfer = unsafe extern "efiapi" fn(
    *mut Protocol,
    u8,
    *mut core::ffi::c_void,
    *mut usize,
    usize,
    *mut u32,
) -> crate::base::Status;

pub type ProtocolAsyncInterruptTransfer = unsafe extern "efiapi" fn(
    *mut Protocol,
    u8,
    crate::base::Boolean,
    usize,
    usize,
    Option<AsyncUsbTransferCallback>,
    *mut core::ffi::c_void,
) -> crate::base::Status;

pub type ProtocolSyncInterruptTransfer = unsafe extern "efiapi" fn(
    *mut Protocol,
    u8,
    *mut core::ffi::c_void,
    *mut usize,
    usize,
    *mut u32,
) -> crate::base::Status;

pub type ProtocolIsochronousTransfer = unsafe extern "efiapi" fn(
    *mut Protocol,
    u8,
    *mut core::ffi::c_void,
    usize,
    *mut u32,
) -> crate::base::Status;

pub type ProtocolAsyncIsochronousTransfer = unsafe extern "efiapi" fn(
    *mut Protocol,
    u8,
    *mut core::ffi::c_void,
    usize,
    Option<AsyncUsbTransferCallback>,
    *mut core::ffi::c_void,
) -> crate::base::Status;

pub type ProtocolGetDeviceDescriptor = unsafe extern "efiapi" fn(
    *mut Protocol,
    *mut DeviceDescriptor,
) -> crate::base::Status;

pub type ProtocolGetConfigDescriptor = unsafe extern "efiapi" fn(
    *mut Protocol,
    *mut ConfigDescriptor,
) -> crate::base::Status;

pub type ProtocolGetInterfaceDescriptor = unsafe extern "efiapi" fn(
    *mut Protocol,
    *mut InterfaceDescriptor,
) -> crate::base::Status;

pub type ProtocolGetEndpointDescriptor = unsafe extern "efiapi" fn(
    *mut Protocol,
    u8,
    *mut EndpointDescriptor,
) -> crate::base::Status;

pub type ProtocolGetStringDescriptor = unsafe extern "efiapi" fn(
    *mut Protocol,
    u16,
    u8,
    *mut *mut crate::base::Char16,
) -> crate::base::Status;

pub type ProtocolGetSupportedLanguages = unsafe extern "efiapi" fn(
    *mut Protocol,
    *mut *mut u16,
    *mut u16,
) -> crate::base::Status;

pub type ProtocolPortReset = unsafe extern "efiapi" fn(
    *mut Protocol,
) -> crate::base::Status;

#[repr(C)]
pub struct Protocol {
    pub control_transfer: ProtocolControlTransfer,
    pub bulk_transfer: ProtocolBulkTransfer,
    pub async_interrupt_transfer: ProtocolAsyncInterruptTransfer,
    pub sync_interrupt_transfer: ProtocolSyncInterruptTransfer,
    pub isochronous_transfer: ProtocolIsochronousTransfer,
    pub async_isochronous_transfer: ProtocolAsyncIsochronousTransfer,
    pub get_device_descriptor: ProtocolGetDeviceDescriptor,
    pub get_config_descriptor: ProtocolGetConfigDescriptor,
    pub get_interface_descriptor: ProtocolGetInterfaceDescriptor,
    pub get_endpoint_descriptor: ProtocolGetEndpointDescriptor,
    pub get_string_descriptor: ProtocolGetStringDescriptor,
    pub get_supported_languages: ProtocolGetSupportedLanguages,
    pub port_reset: ProtocolPortReset,
}
