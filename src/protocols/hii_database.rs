//! Human Interface Infrastructure (HII) Protocol
//!
//! Database manager for HII-related data structures.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xef9fc172,
    0xa1b2,
    0x4693,
    0xb3,
    0x27,
    &[0x6d, 0x32, 0xfc, 0x41, 0x60, 0x42],
);

pub const SET_KEYBOARD_LAYOUT_EVENT_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x14982a4f,
    0xb0ed,
    0x45b8,
    0xa8,
    0x11,
    &[0x5a, 0x7a, 0x9b, 0xc2, 0x32, 0xdf],
);

#[repr(C)]
pub struct Protocol {
    pub new_package_list: eficall! {fn(
        *const Protocol,
        *const crate::hii::PackageListHeader,
        crate::base::Handle,
        *mut crate::hii::Handle,
    ) -> crate::base::Status},
    pub remove_package_list: eficall! {fn(
        *const Protocol,
        crate::hii::Handle,
    ) -> crate::base::Status},
    pub update_package_list: eficall! {fn(
        *const Protocol,
        crate::hii::Handle,
        *const crate::hii::PackageListHeader,
    ) -> crate::base::Status},
    pub list_package_lists: eficall! {fn(
        *const Protocol,
        u8,
        *const crate::base::Guid,
        *mut usize,
        *mut crate::hii::Handle,
    ) -> crate::base::Status},
    pub export_package_lists: eficall! {fn(
        *const Protocol,
        crate::hii::Handle,
        *mut usize,
        *mut crate::hii::PackageListHeader,
    ) -> crate::base::Status},
    pub register_package_notify: eficall! {fn(
        *const Protocol,
        u8,
        *const crate::base::Guid,
        Notify,
        NotifyType,
        *mut crate::base::Handle,
    ) -> crate::base::Status},
    pub unregister_package_notify: eficall! {fn(
        *const Protocol,
        crate::base::Handle,
    ) -> crate::base::Status},
    pub find_keyboard_layouts: eficall! {fn(
        *const Protocol,
        *mut u16,
        *mut crate::base::Guid,
    ) -> crate::base::Status},
    pub get_keyboard_layout: eficall! {fn(
        *const Protocol,
        *const crate::base::Guid,
        *mut u16,
        *mut KeyboardLayout,
    ) -> crate::base::Status},
    pub set_keyboard_layout: eficall! {fn(
        *const Protocol,
        *mut crate::base::Guid,
    ) -> crate::base::Status},
    pub get_package_list_handle: eficall! {fn(
        *const Protocol,
        crate::hii::Handle,
        *mut crate::base::Handle,
    ) -> crate::base::Status},
}

#[repr(C)]
#[derive(Debug)]
pub struct KeyboardLayout {
    pub layout_length: u16,
    pub guid: crate::base::Guid,
    pub layout_descriptor_string_offset: u32,
    pub descriptor_count: u8,
    pub descriptors: [KeyDescriptor],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KeyDescriptor {
    pub key: Key,
    pub unicode: crate::base::Char16,
    pub shifted_unicode: crate::base::Char16,
    pub alt_gr_unicode: crate::base::Char16,
    pub shifted_alt_gr_unicode: crate::base::Char16,
    pub modifier: u16,
    pub affected_attribute: u16,
}

pub const AFFECTED_BY_STANDARD_SHIFT: u16 = 0x0001;
pub const AFFECTED_BY_CAPS_LOCK: u16 = 0x0002;
pub const AFFECTED_BY_NUM_LOCK: u16 = 0x0004;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Key {
    EfiKeyLCtrl,
    EfiKeyA0,
    EfiKeyLAlt,
    EfiKeySpaceBar,
    EfiKeyA2,
    EfiKeyA3,
    EfiKeyA4,
    EfiKeyRCtrl,
    EfiKeyLeftArrow,
    EfiKeyDownArrow,
    EfiKeyRightArrow,
    EfiKeyZero,
    EfiKeyPeriod,
    EfiKeyEnter,
    EfiKeyLShift,
    EfiKeyB0,
    EfiKeyB1,
    EfiKeyB2,
    EfiKeyB3,
    EfiKeyB4,
    EfiKeyB5,
    EfiKeyB6,
    EfiKeyB7,
    EfiKeyB8,
    EfiKeyB9,
    EfiKeyB10,
    EfiKeyRShift,
    EfiKeyUpArrow,
    EfiKeyOne,
    EfiKeyTwo,
    EfiKeyThree,
    EfiKeyCapsLock,
    EfiKeyC1,
    EfiKeyC2,
    EfiKeyC3,
    EfiKeyC4,
    EfiKeyC5,
    EfiKeyC6,
    EfiKeyC7,
    EfiKeyC8,
    EfiKeyC9,
    EfiKeyC10,
    EfiKeyC11,
    EfiKeyC12,
    EfiKeyFour,
    EfiKeyFive,
    EfiKeySix,
    EfiKeyPlus,
    EfiKeyTab,
    EfiKeyD1,
    EfiKeyD2,
    EfiKeyD3,
    EfiKeyD4,
    EfiKeyD5,
    EfiKeyD6,
    EfiKeyD7,
    EfiKeyD8,
    EfiKeyD9,
    EfiKeyD10,
    EfiKeyD11,
    EfiKeyD12,
    EfiKeyD13,
    EfiKeyDel,
    EfiKeyEnd,
    EfiKeyPgDn,
    EfiKeySeven,
    EfiKeyEight,
    EfiKeyNine,
    EfiKeyE0,
    EfiKeyE1,
    EfiKeyE2,
    EfiKeyE3,
    EfiKeyE4,
    EfiKeyE5,
    EfiKeyE6,
    EfiKeyE7,
    EfiKeyE8,
    EfiKeyE9,
    EfiKeyE10,
    EfiKeyE11,
    EfiKeyE12,
    EfiKeyBackSpace,
    EfiKeyIns,
    EfiKeyHome,
    EfiKeyPgUp,
    EfiKeyNLck,
    EfiKeySlash,
    EfiKeyAsterisk,
    EfiKeyMinus,
    EfiKeyEsc,
    EfiKeyF1,
    EfiKeyF2,
    EfiKeyF3,
    EfiKeyF4,
    EfiKeyF5,
    EfiKeyF6,
    EfiKeyF7,
    EfiKeyF8,
    EfiKeyF9,
    EfiKeyF10,
    EfiKeyF11,
    EfiKeyF12,
    EfiKeyPrint,
    EfiKeySLck,
    EfiKeyPause,
}

pub const NULL_MODIFIER: u16 = 0x0000;
pub const LEFT_CONTROL_MODIFIER: u16 = 0x0001;
pub const RIGHT_CONTROL_MODIFIER: u16 = 0x0002;
pub const LEFT_ALT_MODIFIER: u16 = 0x0003;
pub const RIGHT_ALT_MODIFIER: u16 = 0x0004;
pub const ALT_GR_MODIFIER: u16 = 0x0005;
pub const INSERT_MODIFIER: u16 = 0x0006;
pub const DELETE_MODIFIER: u16 = 0x0007;
pub const PAGE_DOWN_MODIFIER: u16 = 0x0008;
pub const PAGE_UP_MODIFIER: u16 = 0x0009;
pub const HOME_MODIFIER: u16 = 0x000A;
pub const END_MODIFIER: u16 = 0x000B;
pub const LEFT_SHIFT_MODIFIER: u16 = 0x000C;
pub const RIGHT_SHIFT_MODIFIER: u16 = 0x000D;
pub const CAPS_LOCK_MODIFIER: u16 = 0x000E;
pub const NUM_LOCK_MODIFIER: u16 = 0x000F;
pub const LEFT_ARROW_MODIFIER: u16 = 0x0010;
pub const RIGHT_ARROW_MODIFIER: u16 = 0x0011;
pub const DOWN_ARROW_MODIFIER: u16 = 0x0012;
pub const UP_ARROW_MODIFIER: u16 = 0x0013;
pub const NS_KEY_MODIFIER: u16 = 0x0014;
pub const NS_KEY_DEPENDENCY_MODIFIER: u16 = 0x0015;
pub const FUNCTION_KEY_ONE_MODIFIER: u16 = 0x0016;
pub const FUNCTION_KEY_TWO_MODIFIER: u16 = 0x0017;
pub const FUNCTION_KEY_THREE_MODIFIER: u16 = 0x0018;
pub const FUNCTION_KEY_FOUR_MODIFIER: u16 = 0x0019;
pub const FUNCTION_KEY_FIVE_MODIFIER: u16 = 0x001A;
pub const FUNCTION_KEY_SIX_MODIFIER: u16 = 0x001B;
pub const FUNCTION_KEY_SEVEN_MODIFIER: u16 = 0x001C;
pub const FUNCTION_KEY_EIGHT_MODIFIER: u16 = 0x001D;
pub const FUNCTION_KEY_NINE_MODIFIER: u16 = 0x001E;
pub const FUNCTION_KEY_TEN_MODIFIER: u16 = 0x001F;
pub const FUNCTION_KEY_ELEVEN_MODIFIER: u16 = 0x0020;
pub const FUNCTION_KEY_TWELVE_MODIFIER: u16 = 0x0021;
pub const PRINT_MODIFIER: u16 = 0x0022;
pub const SYS_REQUEST_MODIFIER: u16 = 0x0023;
pub const SCROLL_LOCK_MODIFIER: u16 = 0x0024;
pub const PAUSE_MODIFIER: u16 = 0x0025;
pub const BREAK_MODIFIER: u16 = 0x0026;
pub const LEFT_LOGO_MODIFIER: u16 = 0x0027;
pub const RIGHT_LOGO_MODIFIER: u16 = 0x0028;
pub const MENU_MODIFIER: u16 = 0x0029;

pub type Notify = eficall! {fn(
    u8,
    *const crate::base::Guid,
    *const crate::hii::PackageHeader,
    crate::hii::Handle,
    NotifyType,
) -> crate::base::Status};

pub type NotifyType = usize;

pub const NOTIFY_NEW_PACK: NotifyType = 0x00000001;
pub const NOTIFY_REMOVE_PACK: NotifyType = 0x00000002;
pub const NOTIFY_EXPORT_PACK: NotifyType = 0x00000004;
pub const NOTIFY_ADD_PACK: NotifyType = 0x00000008;
