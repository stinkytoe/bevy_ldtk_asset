use bevy::{color::Color, log::info};

use crate::error::Error;

// Format should be: Hex color "#rrggbb"
// from: https://ldtk.io/json-2/#ldtk-ProjectJson;bgColor
pub(crate) fn bevy_color_from_ldtk_string(color: &str) -> Result<Color, Error> {
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

pub(crate) fn bevy_color_from_ldtk_int(color: i64) -> Color {
    info!("Creating color from int: {color:016x}");

    let r = ((color & 0xFF0000) >> 16) as u8;
    let g = ((color & 0xFF00) >> 8) as u8;
    let b = (color & 0xFF) as u8;

    Color::srgb_u8(r, g, b)
}
