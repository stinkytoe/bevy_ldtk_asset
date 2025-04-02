#![allow(missing_docs)]

use bevy_math::I64Vec2;
use bevy_reflect::Reflect;

use crate::ldtk;
use crate::{Result, ldtk_import_error};

/// An individual tile in a [crate::layer::TilesLayer] instance.
///
/// This represents a square region within the tileset layer which is associated with the
/// containing layer instance. The size of the square, and the source image, are not defined here
/// but in the layer instance and are common to all tiles in that layer.
#[derive(Clone, Debug, Reflect)]
pub struct TileInstance {
    /// The overall opacity. This is applied even against pixels which already have opacity.
    pub opacity: f32,
    /// If true, flip this image horizontally before blotting onto layer visualization.
    pub flip_x: bool,
    /// If true, flip this image vertically before blotting onto layer visualization.
    pub flip_y: bool,
    /// Top left corner where this tile is to be blotted onto the layer visualization.
    pub offset: I64Vec2,
    /// The top left corner where we get the image from the associated tileset image.
    pub source: I64Vec2,
}

impl TileInstance {
    pub(crate) fn new(value: &ldtk::TileInstance) -> Result<Self> {
        let opacity = value.a as f32;
        let (flip_x, flip_y) = match value.f {
            0 => (false, false),
            1 => (true, false),
            2 => (false, true),
            3 => (true, true),
            _ => {
                return Err(ldtk_import_error!(
                    "Bad value for tile flip bits! given: {}",
                    value.f
                ));
            }
        };
        let offset = (value.px.len() == 2)
            .then(|| (value.px[0], value.px[1]).into())
            .ok_or(ldtk_import_error!(
                "Bad px vector in LDtk tile instance! given: {:?}",
                value.px
            ))?;
        let source = (value.src.len() == 2)
            .then(|| (value.src[0], value.src[1]).into())
            .ok_or(ldtk_import_error!(
                "Bad src vector in LDtk tile instance! given: {:?}",
                value.px
            ))?;

        Ok(Self {
            opacity,
            flip_x,
            flip_y,
            offset,
            source,
        })
    }
}
