//! Transmission Control Protocol version 6
//!
//! It provides services to send and receive data stream.

use crate::efi::{Boolean, Ipv6Address};
use crate::protocols::{ip6, managed_network, simple_network};

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x46e44855,
    0xbd60,
    0x4ab7,
    0xab,
    0x0d,
    &[0xa6, 0x79, 0xb9, 0x44, 0x7d, 0x77],
);

pub const SERVICE_BINDING_PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xec20eb79,
    0x6c1a,
    0x4664,
    0x9a,
    0x0d,
    &[0xd2, 0xe4, 0xcc, 0x16, 0xd6, 0x64],
);

#[repr(C)]
pub struct AccessPoint {
    pub station_address: Ipv6Address,
    pub station_port: u16,
    pub remote_address: Ipv6Address,
    pub remote_port: u16,
    pub active_flag: Boolean,
}

#[repr(C)]
pub struct Tcp6Option {
    pub receive_buffer_size: u32,
    pub send_buffer_size: u32,
    pub max_syn_back_log: u32,
    pub connection_timeout: u32,
    pub data_retries: u32,
    pub fin_timeout: u32,
    pub time_wait_timeout: u32,
    pub keep_alive_probes: u32,
    pub keep_alive_time: u32,
    pub keep_alive_interval: u32,
    pub enable_nagle: Boolean,
    pub enable_time_stamp: Boolean,
    pub enable_window_scaling: Boolean,
    pub enable_selective_ack: Boolean,
    pub enable_path_mtu_discovery: Boolean,
}

#[repr(C)]
pub struct ConfigData {
    pub traffic_class: u8,
    pub hop_limit: u8,
    pub access_point: AccessPoint,
    pub control_option: *mut Tcp6Option,
}

pub type ConnectionState = u32;

pub const CONNECTION_CLOSED: ConnectionState = 0;
pub const STATE_LISTEN: ConnectionState = 1;
pub const SYN_SENT: ConnectionState = 2;
pub const SYN_RECEIVED: ConnectionState = 3;
pub const ESTABLISHED: ConnectionState = 4;
pub const FIN_WAIT1: ConnectionState = 5;
pub const FIN_WAIT2: ConnectionState = 6;
pub const CLOSING: ConnectionState = 7;
pub const TIME_WAIT: ConnectionState = 8;
pub const CLOSE_WAIT: ConnectionState = 9;
pub const LAST_ACK: ConnectionState = 10;

#[repr(C)]
pub struct CompletionToken {
    pub event: crate::base::Event,
    pub status: crate::base::Status,
}

#[repr(C)]
pub struct ConnectionToken {
    pub completion_token: CompletionToken,
}

#[repr(C)]
pub struct ListenToken {
    pub completion_token: CompletionToken,
    pub new_child_handle: crate::base::Handle,
}

#[repr(C)]
pub struct FragmentData {
    pub fragment_length: u32,
    pub fragment_buffer: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct ReceiveData {
    pub urgent_flag: Boolean,
    pub data_length: u32,
    pub fragment_count: u32,
    pub fragment_table: *mut FragmentData,
}

#[repr(C)]
pub struct TransmitData {
    pub push: Boolean,
    pub urgent: Boolean,
    pub data_length: u32,
    pub fragment_count: u32,
    pub fragment_table: *mut FragmentData,
}

#[repr(C)]
pub struct IoToken {
    pub rx_data: *mut ReceiveData,
    pub tx_data: *mut TransmitData,
}

#[repr(C)]
pub struct CloseToken {
    pub completion_token: CompletionToken,
    pub abort_on_close: Boolean,
}

pub type GetModeData = eficall! {fn(
    *mut Protocol,
    *mut ConnectionState,
    *mut ConfigData,
    *mut ip6::ModeData,
    *mut managed_network::ConfigData,
    *mut simple_network::Mode,
) -> crate::base::Status};

pub type Configure = eficall! {fn(
    *mut Protocol,
    *mut ConfigData
) -> crate::base::Status};

pub type Connect = eficall! {fn(
    *mut Protocol,
    *mut ConnectionToken
) -> crate::base::Status};

pub type Accept = eficall! {fn(
    *mut Protocol,
    *mut ListenToken
) -> crate::base::Status};

pub type Transmit = eficall! {fn(
    *mut Protocol,
    *mut IoToken
) -> crate::base::Status};

pub type Receive = eficall! {fn(
    *mut Protocol,
    *mut IoToken
) -> crate::base::Status};

pub type Close = eficall! {fn(
    *mut Protocol,
    *mut CloseToken
) -> crate::base::Status};

pub type Cancel = eficall! {fn(
    *mut Protocol,
    *mut CompletionToken
) -> crate::base::Status};

pub type Poll = eficall! {fn(
    *mut Protocol,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub get_mode_data: GetModeData,
    pub configure: Configure,
    pub connect: Connect,
    pub accept: Accept,
    pub transmit: Transmit,
    pub receive: Receive,
    pub close: Close,
    pub cancel: Cancel,
    pub poll: Poll,
}
