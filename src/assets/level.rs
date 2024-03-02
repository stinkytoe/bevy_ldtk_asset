use bevy::prelude::*;

use crate::{
    ldtk,
    traits::{HasIdentifier, SpawnsEntities},
};

use super::{project::ProjectAsset, world::WorldAsset};

/// An asset representing an LDTK level
#[derive(Asset, Clone, Debug, TypePath)]
pub struct LevelAsset {
    identifier: String,
    project_handle: Handle<ProjectAsset>,
    _background_color: Color,
    _background: Option<(String, ldtk::BgPos)>,
}

impl LevelAsset {
    pub(crate) fn new(level: &ldtk::Level, project_handle: Handle<ProjectAsset>) -> Self {
        Self {
            identifier: level.identifier.clone(),
            project_handle,
            _background_color: Color::WHITE,
            _background: None,
        }
    }

    /// Returns a handle to the project which defines this level
    pub fn project_handle(&self) -> &Handle<ProjectAsset> {
        &self.project_handle
    }
}

impl HasIdentifier for LevelAsset {
    fn identifier(&self) -> &String {
        &self.identifier
    }
}

impl SpawnsEntities for LevelAsset {
    fn spawn_entities(
        &self,
        commands: &mut Commands,
        entity: Entity,
        _projects: &Assets<ProjectAsset>,
        _worlds: &Assets<WorldAsset>,
        _levels: &Assets<LevelAsset>,
    ) {
        commands
            .entity(entity)
            .insert((Name::from(self.identifier().as_str()),));
    }
}
