use bevy::{prelude::*, utils::HashSet};

use crate::ldtk::level_asset::LevelAsset;

#[derive(Debug, Default, Resource)]
pub(crate) struct LdtkLevels {
    pub(crate) to_load: HashSet<(Entity, Handle<LevelAsset>)>,
    pub(crate) loaded: HashSet<(Entity, Handle<LevelAsset>)>,
    // pub(crate) to_unload: HashSet<Entity>,
}
