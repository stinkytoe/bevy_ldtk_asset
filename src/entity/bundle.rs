use bevy::prelude::*;

use crate::entity::EntityAsset;

#[derive(Bundle, Debug, Default)]
pub struct EntityBundle {
    pub(crate) world: Handle<EntityAsset>,
    pub(crate) spatial: SpatialBundle,
}
