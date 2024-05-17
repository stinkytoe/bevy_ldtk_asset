use bevy::prelude::*;

use crate::entity::EntityAsset;

#[derive(Component, Debug, Default, Reflect)]
pub enum EntitiesToLoad {
    None,
    ByIdentifiers(Vec<String>),
    ByIids(Vec<String>),
    #[default]
    All,
}

#[derive(Bundle, Debug, Default)]
pub struct EntityBundle {
    pub(crate) world: Handle<EntityAsset>,
    pub(crate) spatial: SpatialBundle,
}
