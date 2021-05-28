//! Simple Network Protocol
//!
//! The simple network protcol provides services to initialize a network interface, transmit
//! packets, receive packets, and close a network interface.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xa19832b9,
    0xac25,
    0x11d3,
    0x9a,
    0x2d,
    &[0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);

pub const REVISION: u64 = 0x0000000000010000u64;

pub const MAX_MCAST_FILTER_CNT: usize = 16;

pub const RECEIVE_UNICAST: u32 = 0x00000001u32;
pub const RECEIVE_MULTICAST: u32 = 0x00000002u32;
pub const RECEIVE_BROADCAST: u32 = 0x00000004u32;
pub const RECEIVE_PROMISCUOUS: u32 = 0x00000008u32;
pub const RECEIVE_PROMISCUOUS_MULTICAST: u32 = 0x00000010u32;

pub const RECEIVE_INTERRUPT: u32 = 0x00000001u32;
pub const TRANSMIT_INTERRUPT: u32 = 0x00000002u32;
pub const COMMAND_INTERRUPT: u32 = 0x00000004u32;
pub const SOFTWARE_INTERRUPT: u32 = 0x000000008u32;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Mode {
    pub state: u32,
    pub hw_address_size: u32,
    pub media_header_size: u32,
    pub max_packet_size: u32,
    pub nvram_size: u32,
    pub nvram_access_size: u32,
    pub receive_filter_mask: u32,
    pub receive_filter_setting: u32,
    pub max_mcast_filter_count: u32,
    pub mcast_filter_count: u32,
    pub mcast_filter: [crate::base::MacAddress; MAX_MCAST_FILTER_CNT],
    pub current_address: crate::base::MacAddress,
    pub broadcast_address: crate::base::MacAddress,
    pub permanent_address: crate::base::MacAddress,
    pub if_type: u8,
    pub mac_address_changeable: crate::base::Boolean,
    pub multiple_tx_supported: crate::base::Boolean,
    pub media_present_supported: crate::base::Boolean,
    pub media_present: crate::base::Boolean,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum NetworkState {
    NetworkStopped,
    NetworkStarted,
    NetworkInitialized,
    NetworkMaxState,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Statistics {
    pub rx_total_frames: u64,
    pub rx_good_frames: u64,
    pub rx_undersize_frames: u64,
    pub rx_oversize_frames: u64,
    pub rx_dropped_frames: u64,
    pub rx_unicast_frames: u64,
    pub rx_broadcast_frames: u64,
    pub rx_multicast_frames: u64,
    pub rx_crc_error_frames: u64,
    pub rx_total_bytes: u64,
    pub tx_total_frames: u64,
    pub tx_good_frames: u64,
    pub tx_undersize_frames: u64,
    pub tx_oversize_frames: u64,
    pub tx_dropped_frames: u64,
    pub tx_unicast_frames: u64,
    pub tx_broadcast_frames: u64,
    pub tx_multicast_frames: u64,
    pub tx_crc_error_frames: u64,
    pub tx_total_bytes: u64,
    pub collisions: u64,
    pub unsupported_protocol: u64,
    pub rx_duplicated_frames: u64,
    pub rx_decrypt_error_frames: u64,
    pub tx_error_frames: u64,
    pub tx_retry_frames: u64,
}

#[repr(C)]
pub struct Protocol {
    pub revision: u64,
    pub start: eficall! {fn(
        *mut Protocol,
    ) -> crate::base::Status},
    pub stop: eficall! {fn(
        *mut Protocol,
    ) -> crate::base::Status},
    pub initialize: eficall! {fn(
        *mut Protocol,
        usize,
        usize,
    ) -> crate::base::Status},
    pub reset: eficall! {fn(
        *mut Protocol,
        crate::base::Boolean,
    ) -> crate::base::Status},
    pub shutdown: eficall! {fn(
        *mut Protocol,
    ) -> crate::base::Status},
    pub receive_filters: eficall! {fn(
        *mut Protocol,
        u32,
        u32,
        crate::base::Boolean,
        usize,
        *mut crate::base::MacAddress,
    ) -> crate::base::Status},
    pub station_address: eficall! {fn(
        *mut Protocol,
        crate::base::Boolean,
        *mut crate::base::MacAddress,
    ) -> crate::base::Status},
    pub statistics: eficall! {fn(
        *mut Protocol,
        crate::base::Boolean,
        *mut usize,
        *mut Statistics,
    ) -> crate::base::Status},
    pub mcast_ip_to_mac: eficall! {fn(
        *mut Protocol,
        crate::base::Boolean,
        *mut crate::base::IpAddress,
        *mut crate::base::MacAddress,
    ) -> crate::base::Status},
    pub nv_data: eficall! {fn(
        *mut Protocol,
        crate::base::Boolean,
        usize,
        usize,
        *mut core::ffi::c_void,
    ) -> crate::base::Status},
    pub get_status: eficall! {fn(
        *mut Protocol,
        *mut u32,
        *mut *mut core::ffi::c_void,
    ) -> crate::base::Status},
    pub transmit: eficall! {fn(
        *mut Protocol,
        usize,
        usize,
        *mut core::ffi::c_void,
        *mut crate::base::MacAddress,
        *mut crate::base::MacAddress,
        *mut u16,
    ) -> crate::base::Status},
    pub receive: eficall! {fn(
        *mut Protocol,
        *mut usize,
        *mut usize,
        *mut core::ffi::c_void,
        *mut crate::base::MacAddress,
        *mut crate::base::MacAddress,
        *mut u16,
    ) -> crate::base::Status},
    pub wait_for_packet: crate::base::Event,
    pub mode: *mut Mode,
}
