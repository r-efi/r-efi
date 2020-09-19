//! HII (Human Interface Infrastructure) Font Protocol and Font Protocol Ex from
//! Sections 34.1 and 34.2
//!
//! Interfaces which retrieve font information.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xe9ca4775,
    0x8657,
    0x47fc,
    0x97,
    0xe7,
    &[0x7e, 0xd6, 0x5a, 0x8, 0x43, 0x24],
);

pub const EX_PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x849e6875,
    0xdb35,
    0x4df8,
    0xb4,
    0x1e,
    &[0xc8, 0xf3, 0x37, 0x18, 0x7, 0x3f],
);

#[repr(C)]
pub struct Protocol {
    pub string_to_image: eficall! {fn(
        *const Protocol,
        OutFlags,
        String,
        *const DisplayInfo,
        *mut *mut ImageOutput,
        usize,
        usize,
        *mut *mut RowInfo,
        *mut usize,
        *mut usize,
    ) -> crate::base::Status},
    pub string_id_to_image: eficall! {fn(
        *const Protocol,
        OutFlags,
        crate::hii::Handle,
        crate::hii::StringId,
        *const crate::base::Char8,
        *const DisplayInfo,
        *mut *mut ImageOutput,
        usize,
        usize,
        *mut *mut RowInfo,
        *mut usize,
        *mut usize,
    ) -> crate::base::Status},
    pub get_glyph: eficall! {fn(
        *const Protocol,
        crate::base::Char16,
        *const DisplayInfo,
        *mut *mut ImageOutput,
        *mut usize,
    ) -> crate::base::Status},
    pub get_font_info: eficall! {fn(
        *const Protocol,
        *mut Handle,
        *const DisplayInfo,
        *mut *mut DisplayInfo,
        String,
    ) -> crate::base::Status},
}

#[repr(C)]
pub struct ExProtocol {
    pub string_to_image: eficall! {fn(
        *const Protocol,
        OutFlags,
        String,
        *const DisplayInfo,
        *mut *mut ImageOutput,
        usize,
        usize,
        *mut *mut RowInfo,
        *mut usize,
        *mut usize,
    ) -> crate::base::Status},
    pub string_id_to_image: eficall! {fn(
        *const Protocol,
        OutFlags,
        crate::hii::Handle,
        crate::hii::StringId,
        *const crate::base::Char8,
        *const DisplayInfo,
        *mut *mut ImageOutput,
        usize,
        usize,
        *mut *mut RowInfo,
        *mut usize,
        *mut usize,
    ) -> crate::base::Status},
    pub get_glyph: eficall! {fn(
        *const Protocol,
        crate::base::Char16,
        *const DisplayInfo,
        *mut *mut ImageOutput,
        usize,
    ) -> crate::base::Status},
    pub get_font_info: eficall! {fn(
        *const Protocol,
        *mut Handle,
        *const DisplayInfo,
        *mut *mut DisplayInfo,
        String,
    ) -> crate::base::Status},
    pub get_glyph_info: eficall! {fn(
        *const Protocol,
        crate::base::Char16,
        *const DisplayInfo,
        *mut crate::hii::GlyphInfo,
    ) -> crate::base::Status},
}

pub type OutFlags = u32;
pub const OUT_FLAG_CLIP: OutFlags = 0x00000001;
pub const OUT_FLAG_WRAP: OutFlags = 0x00000002;
pub const OUT_FLAG_CLIP_CLEAN_Y: OutFlags = 0x00000004;
pub const OUT_FLAG_CLIP_CLEAN_X: OutFlags = 0x00000008;
pub const OUT_FLAG_TRANSPARENT: OutFlags = 0x00000010;
pub const IGNORE_IF_NO_GLYPH: OutFlags = 0x00000020;
pub const IGNORE_LINE_BREAK: OutFlags = 0x00000040;
pub const DIRECT_TO_SCREEN: OutFlags = 0x00000080;

pub type String = *mut crate::base::Char16;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RowInfo {
    pub start_index: usize,
    pub end_index: usize,
    pub line_height: usize,
    pub line_width: usize,
    pub baseline_offset: usize,
}

pub type Handle = *mut core::ffi::c_void;

#[repr(C)]
#[derive(Debug)]
pub struct DisplayInfo {
    pub foreground_color: crate::protocols::graphics_output::BltPixel,
    pub background_color: crate::protocols::graphics_output::BltPixel,
    pub font_info_mask: InfoMask,
    pub font_info: Info,
}

pub type InfoMask = u32;
pub const INFO_SYS_FONT: InfoMask = 0x00000001;
pub const INFO_SYS_SIZE: InfoMask = 0x00000002;
pub const INFO_SYS_STYLE: InfoMask = 0x00000004;
pub const INFO_SYS_FORE_COLOR: InfoMask = 0x00000010;
pub const INFO_SYS_BACK_COLOR: InfoMask = 0x00000020;
pub const INFO_RESIZE: InfoMask = 0x00001000;
pub const INFO_RESTYLE: InfoMask = 0x00002000;
pub const INFO_ANY_FONT: InfoMask = 0x00010000;
pub const INFO_ANY_SIZE: InfoMask = 0x00020000;
pub const INFO_ANY_STYLE: InfoMask = 0x00040000;

#[repr(C)]
#[derive(Clone, Copy)]
pub union ImageOutputImage {
    pub bitmap: *mut crate::protocols::graphics_output::BltPixel,
    pub screen: *mut crate::protocols::graphics_output::Protocol,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ImageOutput {
    pub width: u16,
    pub height: u16,
    pub image: ImageOutputImage,
}

//
// NOTE: Technically this is defined in Section 34.3 (String Protocol),
// but it introduces a cross-dependency and seems more at home here.
//
#[repr(C)]
#[derive(Debug)]
pub struct Info {
    pub font_style: crate::hii::FontStyle,
    pub font_size: u16,
    pub font_name: [crate::base::Char16],
}
