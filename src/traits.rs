use bevy::prelude::*;

use crate::prelude::{LevelAsset, ProjectAsset, WorldAsset};

pub trait HasIdentifier {
    fn identifier(&self) -> &String;
}

pub trait SpawnsEntities {
    fn spawn_entities(
        &self,
        commands: &mut Commands,
        entity: Entity,
        projects: &Assets<ProjectAsset>,
        worlds: &Assets<WorldAsset>,
        levels: &Assets<LevelAsset>,
    );
}
