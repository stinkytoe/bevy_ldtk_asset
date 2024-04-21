use bevy::prelude::*;

use crate::project::ProjectAsset;

#[derive(Asset, Debug)]
#[cfg_attr(not(feature = "enable_reflect"), derive(TypePath))]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct WorldAsset {
    pub(crate) project_handle: Handle<ProjectAsset>,
    pub(crate) iid: String,
}
