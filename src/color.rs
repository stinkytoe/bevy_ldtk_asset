use bevy_color::Color;

use crate::Result;
use crate::ldtk_import_error;

// Format should be: Hex color "#rrggbb"
// from: https://ldtk.io/json-2/#ldtk-ProjectJson;bgColor
pub(crate) fn bevy_color_from_ldtk_string(color: &str) -> crate::Result<Color> {
    (color.len() == 7)
        .then(|| {
            let hashmark = &color[0..1];
            let r = &color[1..3];
            let g = &color[3..5];
            let b = &color[5..7];

            (hashmark, r, g, b)
        })
        .filter(|(hashmark, _, _, _)| *hashmark == "#")
        .map(|(_, r, g, b)| -> Result<Color> {
            Ok(Color::srgb_u8(
                u8::from_str_radix(r, 16)?,
                u8::from_str_radix(g, 16)?,
                u8::from_str_radix(b, 16)?,
            ))
        })
        .transpose()?
        .ok_or(ldtk_import_error!(
            "Could not produce Bevy color from Ldtk input string! given: {color}"
        ))
}

// Raw color stored in lower 24 bits of value.
// Only used in EnumValueDefinition (i think?)
pub(crate) fn bevy_color_from_ldtk_int(color: i64) -> Color {
    let r = ((color & 0xFF0000) >> 16) as u8;
    let g = ((color & 0xFF00) >> 8) as u8;
    let b = (color & 0xFF) as u8;

    Color::srgb_u8(r, g, b)
}
