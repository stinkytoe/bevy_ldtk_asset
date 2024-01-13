use crate::{ldtk::level_asset::LevelAsset, prelude::ProjectAsset};
use bevy::{prelude::*, utils::HashSet};

#[derive(Debug, Default, Resource)]
pub(crate) struct ProjectEntities {
    pub(crate) to_load: HashSet<(Entity, Handle<ProjectAsset>)>,
    pub(crate) loaded: HashSet<(Entity, Handle<ProjectAsset>)>,
    // pub(crate) to_unload: HashSet<Entity>,
}

#[derive(Debug, Default, Resource)]
pub(crate) struct LevelEntities {
    pub(crate) to_load: HashSet<(Entity, Handle<LevelAsset>)>,
    pub(crate) loaded: HashSet<(Entity, Handle<LevelAsset>)>,
    // pub(crate) to_unload: HashSet<Entity>,
}
