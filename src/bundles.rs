use bevy::prelude::*;

use crate::{assets::world::WorldAsset, structs::LoadParameters};

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
    pub load_parameters: LoadParameters,
    #[allow(missing_docs)]
    pub _spatial_bundle: SpatialBundle,
}
