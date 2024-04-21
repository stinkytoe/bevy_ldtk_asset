use bevy::prelude::*;

#[derive(Asset, Debug)]
#[cfg_attr(not(feature = "enable_reflect"), derive(TypePath))]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct LevelAsset {
    pub(crate) _project_handle: Handle<crate::project::ProjectAsset>,
    pub(crate) _iid: String,
}
