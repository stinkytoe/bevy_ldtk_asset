use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};

use crate::{
    assets::world::WorldAsset,
    prelude::{LdtkEntity, LevelAsset},
    structs::{Layer, SpawnEntities},
};

/// A bundle for spawning Worlds. Use the Bevy asset label syntax:
/// "project.ldtk#World" to specify a given world.
///
/// load_parameters will determine whether bevy_ldtk_asset should
/// spawn entities for the user, or simply load the data.
#[derive(Bundle, Default)]
pub struct WorldBundle {
    #[allow(missing_docs)]
    pub world: Handle<WorldAsset>,
    #[allow(missing_docs)]
    pub spawn_entities: SpawnEntities,
    #[allow(missing_docs)]
    pub spatial_bundle: SpatialBundle,
}

/// A bundle for spawning Levels. Use the Bevy asset label syntax:
/// "project.ldtk#World/Level" to specify a given world.
///
/// load_parameters will determine whether bevy_ldtk_asset should
/// spawn entities for the user, or simply load the data.
#[derive(Bundle, Default)]
pub struct LevelBundle {
    #[allow(missing_docs)]
    pub level: Handle<LevelAsset>,
    #[allow(missing_docs)]
    pub spawn_entities: SpawnEntities,
    #[allow(missing_docs)]
    pub spatial_bundle: SpatialBundle,
}

#[derive(Bundle, Default)]
pub(crate) struct LayerVisibleBundle<M: Material2d> {
    pub(crate) name: Name,
    pub(crate) layer: Layer,
    pub(crate) mesh: MaterialMesh2dBundle<M>,
}

/// A bundle for spawning LDtk entities
#[derive(Bundle, Default)]
pub(crate) struct LdtkEntityLayerBundle {
    pub(crate) name: Name,
    pub(crate) layer: Layer,
    pub(crate) spatial_bundle: SpatialBundle,
}

/// A bundle for spawning LDtk entities
#[derive(Bundle, Default)]
pub struct LdtkEntityBundle {
    pub name: Name,
    pub entity: LdtkEntity,
    pub spatial_bundle: SpatialBundle,
}
