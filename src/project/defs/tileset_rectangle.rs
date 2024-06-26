use bevy::prelude::*;

use crate::ldtk;

#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct TilesetRectangle {
    location: Vec2,
    size: Vec2,
    tileset_uid: i64,
}

impl TilesetRectangle {
    pub fn location(&self) -> Vec2 {
        self.location
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn tileset_uid(&self) -> i64 {
        self.tileset_uid
    }
}

impl From<&ldtk::TilesetRectangle> for TilesetRectangle {
    fn from(value: &ldtk::TilesetRectangle) -> Self {
        Self {
            location: (value.x as f32, value.y as f32).into(),
            size: (value.w as f32, value.h as f32).into(),
            tileset_uid: value.tileset_uid,
        }
    }
}

pub struct TilesetRectanglePlugin;

impl Plugin for TilesetRectanglePlugin {
    fn build(&self, app: &mut App) {
        app //
            .register_type::<TilesetRectangle>();
    }
}
