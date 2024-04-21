use bevy::prelude::*;

use crate::project::ProjectAsset;

pub use crate::ldtk::WorldLayout;

#[derive(Component, Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct WorldBundleLoadSettings;

#[derive(Bundle, Debug, Default)]
pub struct WorldBundle {
    pub world: Handle<WorldAsset>,
    pub settings: WorldBundleLoadSettings,
}

#[derive(Asset, Debug)]
#[cfg_attr(not(feature = "enable_reflect"), derive(TypePath))]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct WorldAsset {
    pub(crate) _project_handle: Handle<ProjectAsset>,
    pub(crate) _iid: String,
}
