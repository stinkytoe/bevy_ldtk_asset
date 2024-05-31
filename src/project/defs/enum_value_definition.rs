use bevy::prelude::*;

use crate::ldtk;
use crate::project::defs::TilesetRectangle;

#[derive(Debug, Default, Reflect)]
pub struct EnumValueDefinition {
    // ?? No idea what this is...
    pub color: i64,
    pub id: String,
    pub tile_rect: Option<TilesetRectangle>,
}

impl From<&ldtk::EnumValueDefinition> for EnumValueDefinition {
    fn from(value: &ldtk::EnumValueDefinition) -> Self {
        EnumValueDefinition {
            color: value.color,
            id: value.id.clone(),
            tile_rect: value.tile_rect.as_ref().map(TilesetRectangle::from),
        }
    }
}
