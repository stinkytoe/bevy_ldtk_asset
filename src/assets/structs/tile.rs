use crate::ldtk_json;
use bevy::prelude::*;

#[derive(Clone, Debug)]
pub(crate) struct Tile {
    pub(crate) _alpha: f32,
    pub(crate) _flip_x: bool,
    pub(crate) _flip_y: bool,
    pub(crate) _location: Vec2,
    pub(crate) _tilemap_location: Vec2,
    pub(crate) _tile_id: i64,
}

impl From<&ldtk_json::TileInstance> for Tile {
    fn from(value: &ldtk_json::TileInstance) -> Self {
        Self {
            _alpha: value.a as f32,
            _flip_x: value.f & 0x01 == 0x01,
            _flip_y: value.f & 0x02 == 0x02,
            _location: Vec2::new(value.px[0] as f32, value.px[1] as f32),
            _tilemap_location: Vec2::new(value.src[0] as f32, value.src[1] as f32),
            _tile_id: value.t,
        }
    }
}
