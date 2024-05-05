use bevy::prelude::*;
use std::path::PathBuf;
use thiserror::Error;

use crate::field_instance::FieldInstance;
use crate::field_instance::FieldInstanceValueParseError;
use crate::ldtk;
use crate::level::LevelBackgroundPosition;
use crate::level::Neighbour;
use crate::level::NeighbourError;
use crate::util::bevy_color_from_ldtk;
use crate::util::ColorParseError;

#[derive(Debug, Error)]
pub enum LevelComponentError {
    #[error("NeighbourError {0}")]
    NeighbourError(#[from] NeighbourError),
    #[error("ColorParseError {0}")]
    ColorParseError(#[from] ColorParseError),
    #[error("FieldInstanceError {0}")]
    FieldInstanceValueError(#[from] FieldInstanceValueParseError),
}

#[derive(Component, Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct LevelComponent {
    bg_color: Color,
    bg_pos: Option<LevelBackgroundPosition>,
    neighbours: Vec<Neighbour>,
    bg_rel_path: Option<PathBuf>,
    external_rel_path: Option<PathBuf>,
    field_instances: Vec<FieldInstance>,
    identifier: String,
    iid: String,
    // TODO: layer_instances
    size: Vec2,
    // (worldX, worldY, and worldDepth)
    // In Bevy coordinate system, not necessarily the same as Bevy transform!
    world_location: Vec3,
}

impl LevelComponent {
    pub fn bg_color(&self) -> Color {
        self.bg_color
    }

    pub fn bg_pos(&self) -> Option<&LevelBackgroundPosition> {
        self.bg_pos.as_ref()
    }

    pub fn neighbours(&self) -> &[Neighbour] {
        self.neighbours.as_ref()
    }

    pub fn bg_rel_path(&self) -> Option<&PathBuf> {
        self.bg_rel_path.as_ref()
    }

    pub fn external_rel_path(&self) -> Option<&PathBuf> {
        self.external_rel_path.as_ref()
    }

    pub fn field_instances(&self) -> &[FieldInstance] {
        self.field_instances.as_ref()
    }

    pub fn identifier(&self) -> &str {
        self.identifier.as_ref()
    }

    pub fn iid(&self) -> &str {
        self.iid.as_ref()
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn world_location(&self) -> Vec3 {
        self.world_location
    }
}

impl TryFrom<&ldtk::Level> for LevelComponent {
    type Error = LevelComponentError;

    fn try_from(value: &ldtk::Level) -> Result<Self, Self::Error> {
        Ok(Self {
            bg_color: bevy_color_from_ldtk(&value.bg_color)?,
            bg_pos: value.bg_pos.as_ref().map(LevelBackgroundPosition::from),
            neighbours: value
                .neighbours
                .iter()
                .map(|neighbour| neighbour.try_into())
                .collect::<Result<_, _>>()?,
            bg_rel_path: value.bg_rel_path.as_ref().map(PathBuf::from),
            external_rel_path: value.external_rel_path.as_ref().map(PathBuf::from),
            field_instances: value
                .field_instances
                .iter()
                .map(|field_instance| field_instance.try_into())
                .collect::<Result<_, _>>()?,
            identifier: value.identifier.clone(),
            iid: value.iid.clone(),
            size: (value.px_wid as f32, value.px_hei as f32).into(),
            world_location: (
                value.world_x as f32,
                -value.world_y as f32,
                value.world_depth as f32,
            )
                .into(),
        })
    }
}
