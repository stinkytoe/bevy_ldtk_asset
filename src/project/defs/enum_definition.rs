use bevy::prelude::*;
use std::path::PathBuf;

use crate::ldtk;
use crate::project::defs::EnumValueDefinition;

#[derive(Debug, Default, Reflect)]
pub struct EnumDefinition {
    pub external_rel_path: Option<PathBuf>,
    pub icon_tileset_uid: Option<i64>,
    pub identifier: String,
    pub tags: Vec<String>,
    pub uid: i64,
    pub values: Vec<EnumValueDefinition>,
}

impl From<&ldtk::EnumDefinition> for EnumDefinition {
    fn from(value: &ldtk::EnumDefinition) -> Self {
        EnumDefinition {
            external_rel_path: value.external_rel_path.as_ref().map(PathBuf::from),
            icon_tileset_uid: value.icon_tileset_uid,
            identifier: value.identifier.clone(),
            tags: value.tags.clone(),
            uid: value.uid,
            values: value.values.iter().map(EnumValueDefinition::from).collect(),
        }
    }
}
