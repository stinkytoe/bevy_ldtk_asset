use bevy::{math::Vec2, reflect::Reflect};

use crate::ldtk;

#[derive(Debug, Reflect)]
pub struct TileInstance {
    pub opacity: f32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub location: Vec2,
    pub corner: Vec2,
}

impl TileInstance {
    pub(crate) fn new(value: &ldtk::TileInstance) -> crate::Result<Self> {
        let opacity = value.a as f32;
        let (flip_x, flip_y) = match value.f {
            0 => (false, false),
            1 => (true, false),
            2 => (false, true),
            3 => (true, true),
            _ => {
                return Err(crate::Error::LdtkImportError(format!(
                    "Bad value for tile flip bits! given: {}",
                    value.f
                )))
            }
        };
        let location = (value.px.len() == 2)
            .then(|| (value.px[0] as f32, -value.px[1] as f32).into())
            .ok_or(crate::Error::LdtkImportError(format!(
                "Bad px vector in LDtk tile instance! given: {:?}",
                value.px
            )))?;
        let corner = (value.src.len() == 2)
            .then(|| (value.src[0] as f32, -value.src[1] as f32).into())
            .ok_or(crate::Error::LdtkImportError(format!(
                "Bad src vector in LDtk tile instance! given: {:?}",
                value.px
            )))?;

        Ok(Self {
            opacity,
            flip_x,
            flip_y,
            location,
            corner,
        })
    }
}
