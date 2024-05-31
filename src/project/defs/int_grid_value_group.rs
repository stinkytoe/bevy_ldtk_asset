use bevy::prelude::*;
use thiserror::Error;

use crate::{
    ldtk,
    util::{bevy_color_from_ldtk, ColorParseError},
};

#[derive(Debug, Default, Reflect)]
pub struct IntGridValueGroup {
    pub color: Option<Color>,
    pub identifier: Option<String>,
    pub uid: i64,
}

#[derive(Debug, Error)]
pub enum IntGridValueGroupFromError {
    #[error(transparent)]
    ColorParseError(#[from] ColorParseError),
}

impl TryFrom<&ldtk::IntGridValueGroupDefinition> for IntGridValueGroup {
    type Error = IntGridValueGroupFromError;

    fn try_from(value: &ldtk::IntGridValueGroupDefinition) -> Result<Self, Self::Error> {
        Ok(Self {
            color: match value.color.as_ref() {
                Some(color) => Some(bevy_color_from_ldtk(color)?),
                None => None,
            },
            identifier: value.identifier.clone(),
            uid: value.uid,
        })
    }
}
