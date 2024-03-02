use bevy::prelude::*;

use crate::{
    ldtk,
    traits::{HasIdentifier, SpawnsEntities},
};

use super::{project::ProjectAsset, world::WorldAsset};

/// An asset representing an LDTK level
#[derive(Asset, Clone, Debug, TypePath)]
pub struct LevelAsset {
    project_handle: Handle<ProjectAsset>,
    identifier: String,
    background_color: Color,
    background: Option<(String, ldtk::BgPos)>,
}

impl LevelAsset {
    pub(crate) fn new(level: &ldtk::Level, project_handle: Handle<ProjectAsset>) -> Self {
        Self {
            project_handle,
            identifier: level.identifier.clone(),
            background_color: Color::WHITE,
            background: None,
        }
    }

    /// Returns a handle to the project which defines this level
    pub fn project_handle(&self) -> &Handle<ProjectAsset> {
        &self.project_handle
    }

    /// The background color of a level
    /// If a pixel is transparent in all layers and the background
    /// image, then this is the color which will show
    pub fn background_color(&self) -> Color {
        self.background_color
    }

    /// An optional background image to show, behind all layers
    pub fn background(&self) -> Option<&(String, ldtk::BgPos)> {
        self.background.as_ref()
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
