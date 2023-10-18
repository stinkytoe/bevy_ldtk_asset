use anyhow::{bail, Result};
use bevy::prelude::*;
use hex::FromHex;

#[derive(Debug)]
pub(crate) struct ColorParseError {
    value: String,
}

impl std::fmt::Display for ColorParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unable to parse given ldtk color string into a bevy color! expected format: hex color #rrggbb, given: {}",
            self.value
        )
    }
}

// Format should be: Hex color "#rrggbb"
// from: https://ldtk.io/json-2/#ldtk-ProjectJson;bgColor
pub(crate) fn get_bevy_color_from_ldtk(color: &str) -> Result<Color> {
    if color.len() != 7 {
        bail!(ColorParseError {
            value: color.to_owned()
        })
    }

    if color.get(0..1) != Some("#") {
        bail!(ColorParseError {
            value: color.to_owned()
        })
    };

    let (Some(red_hex), Some(green_hex), Some(blue_hex)) =
        (color.get(1..3), color.get(3..5), color.get(5..7))
    else {
        bail!(ColorParseError {
            value: color.to_owned()
        })
    };

    let hex_to_float = |hex: &str| -> Result<f32> {
        let Ok(byte) = <[u8; 1]>::from_hex(hex) else {
            bail!(ColorParseError {
                value: color.to_owned()
            });
        };

        Ok(byte[0] as f32 / 255.0)
    };

    Ok(Color::rgb(
        hex_to_float(red_hex)?,
        hex_to_float(green_hex)?,
        hex_to_float(blue_hex)?,
    ))
}
