use std::path::PathBuf;

use bevy::prelude::*;
use bevy::utils::thiserror;
use hex::FromHex;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ColorParseError {
    #[error("Provided color string not seven characters! {0}")]
    BadStringLength(String),
    #[error("Unable to parse given color string! {0}")]
    UnableToParse(String),
}

// Format should be: Hex color "#rrggbb"
// from: https://ldtk.io/json-2/#ldtk-ProjectJson;bgColor
pub(crate) fn get_bevy_color_from_ldtk(color: String) -> Result<Color, ColorParseError> {
    if color.len() != 7 {
        return Err(ColorParseError::BadStringLength(color));
    }

    if color.get(0..1) != Some("#") {
        return Err(ColorParseError::UnableToParse(color));
    };

    let (Some(red_hex), Some(green_hex), Some(blue_hex)) =
        (color.get(1..3), color.get(3..5), color.get(5..7))
    else {
        return Err(ColorParseError::UnableToParse(color));
    };

    let hex_to_float = |hex: &str| -> Result<f32, ColorParseError> {
        let Ok(byte) = <[u8; 1]>::from_hex(hex) else {
            return Err(ColorParseError::UnableToParse(color.clone()));
        };

        Ok(byte[0] as f32 / 255.0)
    };

    Ok(Color::rgb(
        hex_to_float(red_hex)?,
        hex_to_float(green_hex)?,
        hex_to_float(blue_hex)?,
    ))
}

// given the root directory of the project file, and a file path as given by LDtk,
// return the PathBuf representing the joined path
pub fn ldtk_project_path_join(ldtk_project_path: &str, ldtk_project_filename: &str) -> PathBuf {
    PathBuf::from(ldtk_project_path).join(ldtk_project_filename)
}
