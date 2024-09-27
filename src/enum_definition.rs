use bevy::reflect::Reflect;

use crate::error::Error;
use crate::ldtk;
use crate::tileset_rectangle::TilesetRectangle;

#[derive(Debug, Reflect)]
pub struct EnumDefinition {
    pub external_rel_path: Option<String>,
    pub icon_tileset_uid: Option<i64>,
    pub identifier: String,
    pub tags: Vec<String>,
    pub uid: i64,
    pub values: Vec<EnumValueDefinition>,
}

impl EnumDefinition {
    pub fn new(value: &ldtk::EnumDefinition) -> Result<Self, Error> {
        let external_rel_path = value.external_rel_path.clone();
        let icon_tileset_uid = value.icon_tileset_uid;
        let identifier = value.identifier.clone();
        let tags = value.tags.clone();
        let uid = value.uid;
        let values = value
            .values
            .iter()
            .map(EnumValueDefinition::new)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            external_rel_path,
            icon_tileset_uid,
            identifier,
            tags,
            uid,
            values,
        })
    }
}

#[derive(Debug, Reflect)]
pub struct EnumValueDefinition {
    pub color: i64,
    pub id: String,
    pub tile: Option<TilesetRectangle>,
}

impl EnumValueDefinition {
    pub fn new(value: &ldtk::EnumValueDefinition) -> Result<Self, Error> {
        let color = value.color;
        let id = value.id.clone();
        let tile = value.tile_rect.as_ref().map(TilesetRectangle::new);

        Ok(Self { color, id, tile })
    }
}
