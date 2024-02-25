use bevy::prelude::*;

/// A component included in WorldBundle, which we will use to determine if a given
/// asset should spawn its associated entities, or simply be loaded as data
#[derive(Component, Default)]
pub enum LoadParameters {
    #[default]
    /// Load nothing
    Nothing,
    /// Load all entities
    Everything,
}
