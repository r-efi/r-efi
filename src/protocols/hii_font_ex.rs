//! HII Font Ex Protocol

use crate::signatures;

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x849e6875,
    0xdb35,
    0x4df8,
    0xb4,
    0x1e,
    &[0xc8, 0xf3, 0x37, 0x18, 0x07, 0x3f],
);

#[repr(C)]
pub struct Protocol {
    pub string_to_image_ex: signatures::protocols::hii_font_ex::StringIdToImageExSignature,
    pub string_id_to_image_ex: signatures::protocols::hii_font_ex::StringIdToImageExSignature,
    pub get_glyph_ex: signatures::protocols::hii_font_ex::GetGlyphExSignature,
    pub get_font_info_ex: signatures::protocols::hii_font_ex::GetFontInfoExSignature,
    pub get_glyph_info: signatures::protocols::hii_font_ex::GetGlyphInfoSignature,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DisplayInfo {
    pub foreground_color: super::graphics_output::BltPixel,
    pub background_color: super::graphics_output::BltPixel,
    pub font_info_mask: InfoMask,
    pub font_info: super::hii_string::Info,
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
    pub bitmap: *mut super::graphics_output::BltPixel,
    pub screen: *mut super::graphics_output::Protocol,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ImageOutput {
    pub width: u16,
    pub height: u16,
    pub image: ImageOutputImage,
}
