use bevy::prelude::*;
use serde::Deserialize;

use crate::ldtk;

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

    /// Returns the unique identifier for this level
    pub fn identifier(&self) -> &String {
        &self.identifier
    }
}
