use bevy_asset::Handle;
use bevy_math::{Rect, Vec2};
use bevy_reflect::Reflect;

use crate::ldtk;
use crate::tileset_definition::TilesetDefinition;
use crate::uid::UidMap;
use crate::{ldtk_import_error, Result};

#[derive(Clone, Debug, Reflect)]
pub struct TilesetRectangle {
    pub size: Vec2,
    pub region: Rect,
    pub tileset_definition: Handle<TilesetDefinition>,
}

impl TilesetRectangle {
    pub(crate) fn new(
        value: &ldtk::TilesetRectangle,
        tileset_definitions: &UidMap<Handle<TilesetDefinition>>,
    ) -> Result<Self> {
        let corner = Vec2::new(value.x as f32, value.y as f32);
        let size = Vec2::new(value.w as f32, value.h as f32);
        let region = Rect::from_corners(corner, corner + size);
        let tileset_definition = tileset_definitions
            .get(&value.tileset_uid)
            .ok_or(ldtk_import_error!(
                "Bad tileset definition uid! given: {}",
                value.tileset_uid
            ))?
            .clone();

        Ok(Self {
            size,
            region,
            tileset_definition,
        })
    }
}
