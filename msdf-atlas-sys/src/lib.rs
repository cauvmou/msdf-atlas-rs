#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(clippy::all)]
mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use sys::*;

#[cfg(test)]
mod test {
    use crate::sys::{msdf_atlas_FontGeometry_FontGeometry, msdf_atlas_FontGeometry_loadCharset, msdf_atlas_Charset_ASCII, msdf_atlas_Charset, msdf_atlas_FontGeometry_getGlyphs, msdf_atlas_TightAtlasPacker_TightAtlasPacker, msdf_atlas_TightAtlasPacker_DimensionsConstraint, msdf_atlas_TightAtlasPacker_setDimensionsConstraint, msdf_atlas_TightAtlasPacker_DimensionsConstraint_POWER_OF_TWO_SQUARE, msdf_atlas_TightAtlasPacker_setMinimumScale, msdfgen_loadFontData, msdfgen_initializeFreetype};

    const FONT_DATA: &[u8] = include_bytes!("../test_assets/Roboto-Regular.ttf");

    #[test]
    fn test() {
        unsafe {
            let type_handle = msdfgen_initializeFreetype();
            let font = msdfgen_loadFontData(type_handle, FONT_DATA.as_ptr(), FONT_DATA.len() as i32);
            let font_geometry = std::ptr::null_mut();
            msdf_atlas_FontGeometry_FontGeometry(font_geometry);
            let charset: *const msdf_atlas_Charset = &msdf_atlas_Charset_ASCII;
            msdf_atlas_FontGeometry_loadCharset(font_geometry, font, 1.0, charset, true, true);
            let glyphs = msdf_atlas_FontGeometry_getGlyphs(font_geometry);
            let glyphs = glyphs.glyphs;

            let packer = std::ptr::null_mut();
            msdf_atlas_TightAtlasPacker_TightAtlasPacker(packer);

            msdf_atlas_TightAtlasPacker_setDimensionsConstraint(packer, msdf_atlas_TightAtlasPacker_DimensionsConstraint_POWER_OF_TWO_SQUARE);
            msdf_atlas_TightAtlasPacker_setMinimumScale(packer, 24.0);


        }
    }
}
