use bevy::prelude::*;

use crate::level::LevelBundleLoadSettings;
use crate::project::ProjectAsset;
use crate::world::LoadLevels;
use crate::world::WorldBundleLoadSettings;

#[derive(Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub enum LoadWorlds {
    None,
    ByIdentifiers(Vec<String>),
    ByIids(Vec<String>),
    #[default]
    All,
}

#[derive(Component, Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct ProjectBundleLoadSettings {
    pub load_worlds: LoadWorlds,
    pub load_levels: LoadLevels,
    pub world_bundle_load_settings: WorldBundleLoadSettings,
    pub level_bundle_load_settings: LevelBundleLoadSettings,
}

#[derive(Bundle, Debug, Default)]
pub struct ProjectBundle {
    pub project: Handle<ProjectAsset>,
    pub settings: ProjectBundleLoadSettings,
    pub spatial: SpatialBundle,
}
