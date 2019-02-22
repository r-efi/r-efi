//! Device Path Protocol
//!
//! The device path protocol defines how to obtain generic path/location information
//! concerning the phisycal or logical device.

pub const LOADED_IMAGE_DEVICE_PATH_ID: crate::base::Guid = crate::base::Guid::from_fields(
    0xbc62157e, 0x3e33, 0x4fec, 0x99, 0x20, &[0x2d, 0x3b, 0x36, 0xd7, 0x50, 0xdf]
);

pub const DEVICE_PATH_ID: crate::base::Guid = crate::base::Guid::from_fields(
    0x09576e91, 0x6d3f, 0x11d2, 0x8e, 0x39, &[0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]
);

pub const TYPE_HARDWARE_DEVICE_PATH:    u8 = 0x01;
pub const TYPE_ACPI_DEVICE_PATH:        u8 = 0x02;
pub const TYPE_MESSAGING_DEVICE_PATH:   u8 = 0x03;
pub const TYPE_MEDIA_DEVICE_PATH:       u8 = 0x04;
pub const TYPE_BIOS_DEVICE_PATH:        u8 = 0x05;
pub const TYPE_END:                     u8 = 0x7f;

pub mod hardware_path {
    pub const SUBTYPE_PCI:              u8 = 0x01;
    pub const SUBTYPE_PCCARD:           u8 = 0x02;
    pub const SUBTYPE_MMAP:             u8 = 0x03;
    pub const SUBTYPE_VENDOR:           u8 = 0x04;
    pub const SUBTYPE_CONTROLLER:       u8 = 0x05;
    pub const SUBTYPE_BMC:              u8 = 0x06;
}

pub mod acpi_path {
    pub const SUBTYPE_ACPI:             u8 = 0x01;
    pub const SUBTYPE_EXPANDED_ACPI:    u8 = 0x02;
    pub const SUBTYPE_ADR:              u8 = 0x03;
}

pub mod messaging_path {
    pub const SUBTYPE_ATAPI:            u8 = 0x01;
    pub const SUBTYPE_SCSI:             u8 = 0x02;
    pub const SUBTYPE_FIBRECHANNEL:     u8 = 0x03;
    pub const SUBTYPE_FIBRECHANNEL_EX:  u8 = 0x15;
    pub const SUBTYPE_1394:             u8 = 0x04;
    pub const SUBTYPE_USB:              u8 = 0x05;
    pub const SUBTYPE_SATA:             u8 = 0x12;
    pub const SUBTYPE_WWID:             u8 = 0x10;
    pub const SUBTYPE_LOGICAL_UNIT:     u8 = 0x11;
    pub const SUBTYPE_USB_CLASS:        u8 = 0x0f;
    pub const SUBTYPE_I2O:              u8 = 0x06;
    pub const SUBTYPE_MAC:              u8 = 0x0b;
    pub const SUBTYPE_IPV4:             u8 = 0x0c;
    pub const SUBTYPE_IPV6:             u8 = 0x0d;
    pub const SUBTYPE_VLAN:             u8 = 0x14;
    pub const SUBTYPE_INFINIBAND:       u8 = 0x09;
    pub const SUBTYPE_UART:             u8 = 0x0e;
    pub const SUBTYPE_VENDOR:           u8 = 0x0a;
    pub const SUBTYPE_SAS:              u8 = 0x16;
    pub const SUBTYPE_ISCSI:            u8 = 0x13;
    pub const SUBTYPE_NVM:              u8 = 0x17;
    pub const SUBTYPE_URI:              u8 = 0x18;
    pub const SUBTYPE_UFS:              u8 = 0x19;
    pub const SUBTYPE_SD:               u8 = 0x1a;
    pub const SUBTYPE_BLUETOOTH:        u8 = 0x1b;
    pub const SUBTYPE_WIRELESS:         u8 = 0x1c;
    pub const SUBTYPE_EMMC:             u8 = 0x1d;
    pub const SUBTYPE_BLUETOOTH_LE:     u8 = 0x1e;
    pub const SUBTYPE_DNS:              u8 = 0x1f;
}

pub mod media_path {
    pub const SUBTYPE_HDD:              u8 = 0x01;
    pub const SUBTYPE_CD:               u8 = 0x02;
    pub const SUBTYPE_VENDOR:           u8 = 0x03;
    pub const SUBTYPE_FILE:             u8 = 0x04;
    pub const SUBTYPE_MEDIA:            u8 = 0x05;
    pub const SUBTYPE_PIWG_FILE:        u8 = 0x06;
    pub const SUBTYPE_PIWG_VOLUME:      u8 = 0x07;
    pub const SUBTYPE_OFFSET:           u8 = 0x08;
    pub const SUBTYPE_RAMDISK:          u8 = 0x09;
}

pub mod bios_path {
    pub const SUBTYPE_BIOS:             u8 = 0x01;
}

pub mod end_path {
    pub const SUBTYPE_ENTIRE_PATH:      u8 = 0xff;
    pub const SUBTYPE_THIS_INSTANCE:    u8 = 0x01;
}

#[repr(C)]
pub struct Protocol {
    pub path_type: u8,
    pub path_subtype: u8,
    pub length: [u8; 2],
}
