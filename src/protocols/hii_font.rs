//! HII Font Protocol

use crate::signatures;

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xe9ca4775,
    0x8657,
    0x47fc,
    0x97,
    0xe7,
    &[0x7e, 0xd6, 0x5a, 0x08, 0x43, 0x24],
);

#[repr(C)]
pub struct Protocol {
    pub string_to_image: signatures::protocols::hii_font::StringToImageSignature,
    pub string_id_to_image: signatures::protocols::hii_font::StringIdToImageSignature,
    pub get_glyph: signatures::protocols::hii_font::GetGlyphSignature,
    pub get_font_info: signatures::protocols::hii_font::GetFontInfoSignature,
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
#[derive(Clone, Copy, Debug)]
pub struct RowInfo {
    pub start_index: usize,
    pub end_index: usize,
    pub line_height: usize,
    pub line_width: usize,
    pub baseline_offset: usize,
}

pub type Handle = *mut core::ffi::c_void;
