use bevy::prelude::*;
use bevy::sprite::Anchor;
use hex::FromHex;
use path_clean::PathClean;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;

pub(crate) fn ldtk_path_to_asset_path(base_directory: &Path, ldtk_path: &Path) -> PathBuf {
    base_directory.join(ldtk_path).clean()
}

#[derive(Debug, Error)]
pub enum ColorParseError {
    #[error("Provided color string not seven characters! {0}")]
    BadStringLength(String),
    #[error("Unable to parse given color string! expect hex color in format: #rrggbb, got: {0}")]
    UnableToParse(String),
}

// Format should be: Hex color "#rrggbb"
// from: https://ldtk.io/json-2/#ldtk-ProjectJson;bgColor
pub(crate) fn bevy_color_from_ldtk(color: &str) -> Result<Color, ColorParseError> {
    if color.len() != 7 {
        return Err(ColorParseError::BadStringLength(color.to_owned()));
    }

    if color.get(0..1) != Some("#") {
        return Err(ColorParseError::UnableToParse(color.to_owned()));
    };

    let (Some(red_hex), Some(green_hex), Some(blue_hex)) =
        (color.get(1..3), color.get(3..5), color.get(5..7))
    else {
        return Err(ColorParseError::UnableToParse(color.to_owned()));
    };

    let hex_to_float = |hex: &str| -> Result<f32, ColorParseError> {
        let Ok(byte) = <[u8; 1]>::from_hex(hex) else {
            return Err(ColorParseError::UnableToParse(color.to_string()));
        };

        Ok(byte[0] as f32 / 255.0)
    };

    Ok(Color::rgb(
        hex_to_float(red_hex)?,
        hex_to_float(green_hex)?,
        hex_to_float(blue_hex)?,
    ))
}

#[derive(Debug, Error)]
pub enum AnchorIntoError {
    #[error("Provided array not four numbers!")]
    BadArrayLength,
}

pub(crate) fn bevy_anchor_from_ldtk(pivot: &[f64]) -> Result<Anchor, AnchorIntoError> {
    if pivot.len() != 2 {
        return Err(AnchorIntoError::BadArrayLength);
    }

    Ok(match (pivot[0] as f32, pivot[1] as f32) {
        (0.0, 0.0) => Anchor::TopLeft,
        (0.0, 0.5) => Anchor::CenterLeft,
        (0.0, 1.0) => Anchor::BottomLeft,
        (0.5, 0.0) => Anchor::TopCenter,
        (0.5, 0.5) => Anchor::Center,
        (0.5, 1.0) => Anchor::BottomCenter,
        (1.0, 0.0) => Anchor::TopRight,
        (1.0, 0.5) => Anchor::CenterRight,
        (1.0, 1.0) => Anchor::BottomRight,
        (x, y) => Anchor::Custom(Vec2::new(x - 0.5, 0.5 - y)),
    })
}
