use bevy::prelude::*;

use crate::{
    ldtk,
    traits::{HasIdentifier, SpawnsEntities},
};

/// An asset representing an LDTK level
#[derive(Asset, Clone, Debug, TypePath)]
pub struct LevelAsset {
    identifier: String,
    _background_color: Color,
    _background: Option<(String, ldtk::BgPos)>,
}

impl LevelAsset {
    pub(crate) fn new(level: &ldtk::Level) -> Self {
        Self {
            identifier: level.identifier.clone(),
            _background_color: Color::WHITE,
            _background: None,
        }
    }
}

impl HasIdentifier for LevelAsset {
    fn identifier(&self) -> &String {
        &self.identifier
    }
}

impl SpawnsEntities for LevelAsset {
    fn spawn_entities(&self, commands: &mut Commands, entity: Entity) {
        commands
            .entity(entity)
            .insert((Name::from(self.identifier().as_str()),));
    }
}
