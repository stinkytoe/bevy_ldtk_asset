use bevy::math::I64Vec2;
use bevy::prelude::*;

use crate::ldtk;

#[derive(Debug)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct Tile {
    alpha: f32,
    flip_h: bool,
    flip_v: bool,
    location: I64Vec2,
    source: UVec2,
    tileset_id: i64,
}

impl Tile {
    pub fn alpha(&self) -> f32 {
        self.alpha
    }

    pub fn flip_h(&self) -> bool {
        self.flip_h
    }

    pub fn flip_v(&self) -> bool {
        self.flip_v
    }

    pub fn location(&self) -> I64Vec2 {
        self.location
    }

    pub fn source(&self) -> UVec2 {
        self.source
    }

    pub fn tileset_id(&self) -> i64 {
        self.tileset_id
    }
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

#[derive(Component, Debug)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct Tiles {
    pub tiles: Vec<Tile>,
}
