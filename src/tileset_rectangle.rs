use bevy::math::Vec2;
use bevy::reflect::Reflect;

use crate::ldtk;

#[derive(Debug, Reflect)]
pub struct TilesetRectangle {
    pub corner: Vec2,
    pub size: Vec2,
    pub tileset_uid: i64,
}

impl TilesetRectangle {
    pub(crate) fn new(value: &ldtk::TilesetRectangle) -> Self {
        let corner = (value.x as f32, value.y as f32).into();
        let size = (value.w as f32, value.h as f32).into();
        let tileset_uid = value.tileset_uid;

        Self {
            corner,
            size,
            tileset_uid,
        }
    }
}
