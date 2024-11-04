use bevy_asset::Handle;
use bevy_math::Vec2;
use bevy_reflect::Reflect;

use crate::ldtk;
use crate::tileset_definition::TilesetDefinition;
use crate::uid::UidMap;
use crate::{Error, Result};

#[derive(Clone, Debug, Reflect)]
pub struct TilesetRectangle {
    pub corner: Vec2,
    pub size: Vec2,
    pub tileset_definition: Handle<TilesetDefinition>,
}

impl TilesetRectangle {
    pub(crate) fn new(
        value: &ldtk::TilesetRectangle,
        tileset_definitions: &UidMap<Handle<TilesetDefinition>>,
    ) -> Result<Self> {
        let corner = (value.x as f32, value.y as f32).into();
        let size = (value.w as f32, value.h as f32).into();
        let tileset_definition = tileset_definitions
            .get(&value.tileset_uid)
            .ok_or(Error::LdtkImportError(format!(
                "Bad tileset definition uid! given: {}",
                value.tileset_uid
            )))?
            .clone();

        Ok(Self {
            corner,
            size,
            tileset_definition,
        })
    }
}
