//! IPv6 Protocol
//!
//! It implements a simple packet-oriented interface that can be used by
//! drivers, daemons, and applications to transmit and receive network packets.

use crate::efi::{Boolean, Ipv6Address, MacAddress};

#[repr(C)]
pub struct ConfigData {
    default_protocol: u8,
    accept_any_protocol: Boolean,
    accept_icmp_errors: Boolean,
    accept_promiscuous: Boolean,
    destination_address: Ipv6Address,
    station_address: Ipv6Address,
    traffic_class: u8,
    hop_limit: u8,
    flow_lable: u32,
    receive_timeout: u32,
    transmit_timeout: u32,
}

#[repr(C)]
pub struct AddressInfo {
    address: Ipv6Address,
    prefix_length: u8,
}

#[repr(C)]
pub struct RouteTable {
    gateway: Ipv6Address,
    destination: Ipv6Address,
    prefix_length: u8,
}

pub type NeighboutState = u32;

pub const EFI_NEIGHBOUR_IN_COMPLETE: NeighboutState = 0;
pub const EFI_NEIGHBOUR_REACHABLE: NeighboutState = 1;
pub const EFI_NEIGHBOUR_STATE: NeighboutState = 2;
pub const EFI_NEIGHBOUR_DELAY: NeighboutState = 3;
pub const EFI_NEIGHBOUR_PROBE: NeighboutState = 4;

#[repr(C)]
pub struct NeighbourCache {
    neighbour: Ipv6Address,
    link_address: MacAddress,
    state: NeighboutState,
}

#[repr(C)]
pub struct IcmpType {
    r#type: u8,
    code: u8,
}

#[repr(C)]
pub struct ModeData {
    is_started: Boolean,
    max_packet_size: u32,
    config_data: ConfigData,
    is_configured: Boolean,
    address_count: u32,
    address_list: *mut AddressInfo,
    group_count: u32,
    group_table: *mut Ipv6Address,
    route_count: u32,
    route_table: *mut RouteTable,
    neighbour_count: u32,
    neighbour_cache: *mut NeighbourCache,
    prefix_count: u32,
    prefix_table: *mut AddressInfo,
    icmp_type_count: u32,
    icmp_type_list: *mut IcmpType,
}
