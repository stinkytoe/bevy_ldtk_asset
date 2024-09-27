use bevy::color::Color;

use crate::error::Error;

// Format should be: Hex color "#rrggbb"
// from: https://ldtk.io/json-2/#ldtk-ProjectJson;bgColor
pub(crate) fn bevy_color_from_ldtk(color: &str) -> Result<Color, Error> {
    if color.len() != 7 {
        return Err(Error::LdtkImportError(format!(
            "LDtk color field not seven characters! given: {color}"
        )));
    }

    let hashmark = &color[0..1];
    let r = &color[1..3];
    let g = &color[3..5];
    let b = &color[5..7];

    if hashmark != "#" {
        return Err(Error::LdtkImportError(format!(
            "LDtk color field did not start with hash! given: {color}",
        )));
    };

    Ok(Color::srgb_u8(
        u8::from_str_radix(r, 16)?,
        u8::from_str_radix(g, 16)?,
        u8::from_str_radix(b, 16)?,
    ))
}
