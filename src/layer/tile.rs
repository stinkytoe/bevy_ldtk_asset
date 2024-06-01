use bevy::{math::I64Vec2, prelude::*};

use crate::ldtk;

#[derive(Debug, Reflect)]
pub struct Tile {
    pub alpha: f32,
    pub flip_h: bool,
    pub flip_v: bool,
    pub location: I64Vec2,
    pub source: UVec2,
    pub tileset_id: i64,
}

impl From<&ldtk::TileInstance> for Tile {
    fn from(value: &ldtk::TileInstance) -> Self {
        Self {
            alpha: value.a as f32,
            flip_h: value.f & 1 == 1,
            flip_v: value.f & 2 == 2,
            location: (value.px[0], value.px[1]).into(),
            source: (value.src[0] as u32, value.src[1] as u32).into(),
            tileset_id: value.t,
        }
    }
}

#[derive(Component, Debug, Reflect)]
pub struct Tiles {
    pub tiles: Vec<Tile>,
}
