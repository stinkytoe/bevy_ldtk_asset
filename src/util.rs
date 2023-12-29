use bevy::prelude::*;
use bevy::utils::thiserror;
use hex::FromHex;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ColorParseError {
    #[error("Provided color string not seven characters! {0}")]
    _BadStringLength(String),
    #[error("Unable to parse given color string! expect hex color in format: #rrggbb, got: {0}")]
    _UnableToParse(String),
}

// Format should be: Hex color "#rrggbb"
// from: https://ldtk.io/json-2/#ldtk-ProjectJson;bgColor
pub(crate) fn get_bevy_color_from_ldtk(color: &str) -> Result<Color, ColorParseError> {
    if color.len() != 7 {
        return Err(ColorParseError::_BadStringLength(color.to_owned()));
    }

    if color.get(0..1) != Some("#") {
        return Err(ColorParseError::_UnableToParse(color.to_owned()));
    };

    let (Some(red_hex), Some(green_hex), Some(blue_hex)) =
        (color.get(1..3), color.get(3..5), color.get(5..7))
    else {
        return Err(ColorParseError::_UnableToParse(color.to_owned()));
    };

    let hex_to_float = |hex: &str| -> Result<f32, ColorParseError> {
        let Ok(byte) = <[u8; 1]>::from_hex(hex) else {
            return Err(ColorParseError::_UnableToParse(color.to_string()));
        };

        Ok(byte[0] as f32 / 255.0)
    };

    Ok(Color::rgb(
        hex_to_float(red_hex)?,
        hex_to_float(green_hex)?,
        hex_to_float(blue_hex)?,
    ))
}
