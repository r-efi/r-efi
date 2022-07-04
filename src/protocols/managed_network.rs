//! Managed Network Protocol
//!
//! It provides raw (unformatted) asynchronous network packet I/O services.
//! These services make it possible for multiple-event-driven drivers and
//! applications to access and use the system network interfaces at the same time.

use crate::efi::Boolean;

#[repr(C)]
pub struct ConfigData {
    pub received_queue_timeout_value: u32,
    pub transmit_queue_timeout_value: u32,
    pub protocol_type_filter: u16,
    pub enable_unicast_receive: Boolean,
    pub enable_multicast_receive: Boolean,
    pub enable_broadcast_receive: Boolean,
    pub enable_promiscuous_receive: Boolean,
    pub flush_queues_on_reset: Boolean,
    pub enable_receive_timestamps: Boolean,
    pub disable_background_polling: Boolean,
}
