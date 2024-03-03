use bevy::prelude::*;

use crate::prelude::{LevelAsset, ProjectAsset, WorldAsset};

pub trait HasIdentifier {
    fn identifier(&self) -> &String;
}

#[allow(clippy::too_many_arguments)]
pub trait SpawnsEntities {
    fn spawn_entities(
        &self,
        commands: &mut Commands,
        entity: Entity,
        asset_server: &AssetServer,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        images: &mut Assets<Image>,
        projects: &Assets<ProjectAsset>,
        worlds: &Assets<WorldAsset>,
        levels: &Assets<LevelAsset>,
    );
}
