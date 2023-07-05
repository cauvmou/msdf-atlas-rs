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
    use crate::sys::{msdf_atlas_FontGeometry_FontGeometry, msdf_atlas_FontGeometry_loadCharset, msdf_atlas_Charset_ASCII, msdf_atlas_Charset, msdf_atlas_FontGeometry_getGlyphs, msdf_atlas_TightAtlasPacker_TightAtlasPacker, msdf_atlas_TightAtlasPacker_DimensionsConstraint, msdf_atlas_TightAtlasPacker_setDimensionsConstraint, msdf_atlas_TightAtlasPacker_DimensionsConstraint_POWER_OF_TWO_SQUARE, msdf_atlas_TightAtlasPacker_setMinimumScale, msdfgen_loadFontData, msdfgen_initializeFreetype, msdf_atlas_FontGeometry_GlyphRange, msdf_atlas_GlyphGeometry, msdfgen_edgeColoringInkTrap, msdf_atlas_TightAtlasPacker_setPixelRange, msdf_atlas_TightAtlasPacker_setMiterLimit, msdf_atlas_TightAtlasPacker_pack, msdf_atlas_FontGeometry, msdf_atlas_TightAtlasPacker};

    const FONT_DATA: &[u8] = include_bytes!("../test_assets/Roboto-Regular.ttf");

    #[test]
    fn export_roboto() {
        unsafe {
            let type_handle = msdfgen_initializeFreetype();
            let font = msdfgen_loadFontData(type_handle, FONT_DATA.as_ptr(), FONT_DATA.len() as i32);

            let mut font_geometry = msdf_atlas_FontGeometry::new();
            
            let charset: *const msdf_atlas_Charset = &msdf_atlas_Charset_ASCII;
            font_geometry.loadCharset(font, 1.0, charset, true, true);

            let mut glyphs = font_geometry.getGlyphs();
            for glyph in glyph_range_to_vec_ref(&mut glyphs) {
                glyph.edgeColoring(Some(msdfgen_edgeColoringInkTrap), 3.0, 0);
            }

            let mut packer = msdf_atlas_TightAtlasPacker::new();

            packer.setDimensionsConstraint(msdf_atlas_TightAtlasPacker_DimensionsConstraint_POWER_OF_TWO_SQUARE);
            packer.setMinimumScale(24.0);
            packer.setPixelRange(2.0);
            packer.setMiterLimit(1.0);
            packer.pack(glyphs.begin().cast_mut(), glyphs.size() as i32);

            let (mut width, mut height) = (0, 0);
            packer.getDimensions(&mut width, &mut height);
        }
    }

    unsafe fn glyph_range_to_vec_ref<'a>(range: &'a mut msdf_atlas_FontGeometry_GlyphRange) -> Vec<&'a mut msdf_atlas_GlyphGeometry> {
        let start = range.begin();
        let end = range.end();
        let len = end.offset_from(start);
        
        let mut out = Vec::with_capacity(len as usize);
        for i in 0..len {
            let glyph = start.offset(i).cast_mut();
            if let Some(r) = glyph.as_mut() {
                out.push(r);
            }
        }

        out
    }
}
