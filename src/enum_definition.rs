use bevy_color::Color;
use bevy_reflect::Reflect;

use crate::color::bevy_color_from_ldtk_int;
use crate::ldtk;
use crate::tileset_rectangle::TilesetRectangle;

#[derive(Debug, Reflect)]
pub struct EnumValueDefinition {
    pub color: Color,
    pub id: String,
    pub tile: Option<TilesetRectangle>,
}

impl EnumValueDefinition {
    pub fn new(value: &ldtk::EnumValueDefinition) -> Self {
        let color = bevy_color_from_ldtk_int(value.color);
        let id = value.id.clone();
        let tile = value.tile_rect.as_ref().map(TilesetRectangle::new);

        Self { color, id, tile }
    }
}

#[derive(Debug, Reflect)]
pub struct EnumDefinition {
    pub external_rel_path: Option<String>,
    pub icon_tileset_uid: Option<i64>,
    pub identifier: String,
    pub tags: Vec<String>,
    pub values: Vec<EnumValueDefinition>,
}

impl From<ldtk::EnumDefinition> for (i64, EnumDefinition) {
    fn from(value: ldtk::EnumDefinition) -> Self {
        let uid = value.uid;

        let external_rel_path = value.external_rel_path.clone();
        let icon_tileset_uid = value.icon_tileset_uid;
        let identifier = value.identifier.clone();
        let tags = value.tags.clone();
        let values = value.values.iter().map(EnumValueDefinition::new).collect();

        (
            uid,
            EnumDefinition {
                external_rel_path,
                icon_tileset_uid,
                identifier,
                tags,
                values,
            },
        )
    }
}
