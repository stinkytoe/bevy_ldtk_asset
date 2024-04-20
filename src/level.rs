use bevy::prelude::*;
use bevy::utils::thiserror;
use core::f32;
use std::path::PathBuf;
use thiserror::Error;

use crate::ldtk;
use crate::{
    field_instance::{FieldInstance, FieldInstanceValueError},
    util::{bevy_color_from_ldtk, ColorParseError},
};

#[derive(Debug, Default)]
#[cfg_attr(feature = "enable_typepath", derive(TypePath))]
pub struct LevelBackgroundPosition {
    crop_location: Vec2,
    crop_size: Vec2,
    scale: Vec2,
    top_left: Vec2,
}

impl LevelBackgroundPosition {
    pub fn crop_location(&self) -> Vec2 {
        self.crop_location
    }

    pub fn crop_size(&self) -> Vec2 {
        self.crop_size
    }

    pub fn scale(&self) -> Vec2 {
        self.scale
    }

    pub fn top_left(&self) -> Vec2 {
        self.top_left
    }
}

impl From<&ldtk::LevelBackgroundPosition> for LevelBackgroundPosition {
    fn from(value: &ldtk::LevelBackgroundPosition) -> Self {
        Self {
            crop_location: (value.crop_rect[0] as f32, -value.crop_rect[1] as f32).into(),
            crop_size: (value.crop_rect[2] as f32, value.crop_rect[3] as f32).into(),
            scale: (value.scale[0] as f32, value.scale[1] as f32).into(),
            top_left: (value.top_left_px[0] as f32, -value.top_left_px[1] as f32).into(),
        }
    }
}

#[derive(Debug, Error)]
pub enum NeighbourError {
    #[error("Given unknown neighbour string from LDtk project! {0}")]
    BadString(String),
}

#[derive(Debug)]
#[cfg_attr(feature = "enable_typepath", derive(TypePath))]
pub enum NeighbourDir {
    North,
    South,
    West,
    East,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
    Lower,
    Greater,
    Overlap,
}

#[derive(Debug)]
#[cfg_attr(feature = "enable_typepath", derive(TypePath))]
pub struct Neighbour {
    level_iid: String,
    dir: NeighbourDir,
}

impl Neighbour {
    pub fn iid(&self) -> &str {
        self.level_iid.as_ref()
    }

    pub fn dir(&self) -> &NeighbourDir {
        &self.dir
    }
}

impl TryFrom<&ldtk::NeighbourLevel> for Neighbour {
    type Error = NeighbourError;

    fn try_from(value: &ldtk::NeighbourLevel) -> Result<Self, Self::Error> {
        Ok(Self {
            level_iid: value.level_iid.clone(),
            dir: match value.dir.as_str() {
                "n" => NeighbourDir::North,
                "s" => NeighbourDir::South,
                "w" => NeighbourDir::West,
                "e" => NeighbourDir::East,
                "<" => NeighbourDir::Lower,
                ">" => NeighbourDir::Greater,
                "o" => NeighbourDir::Overlap,
                "nw" => NeighbourDir::NorthWest,
                "ne" => NeighbourDir::NorthEast,
                "sw" => NeighbourDir::SouthWest,
                "se" => NeighbourDir::SouthEast,
                _ => return Err(NeighbourError::BadString(value.dir.clone())),
            },
        })
    }
}

#[derive(Debug, Error)]
pub enum LevelComponentError {
    #[error("NeighbourError {0}")]
    NeighbourError(#[from] NeighbourError),
    #[error("ColorParseError {0}")]
    ColorParseError(#[from] ColorParseError),
    #[error("FieldInstanceError {0}")]
    FieldInstanceValueError(#[from] FieldInstanceValueError),
}

#[derive(Component, Debug, Default)]
#[cfg_attr(feature = "enable_typepath", derive(TypePath))]
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

// This component is immediately removed,
// and the appropriate assets/components/child entities/etc are added
#[derive(Component, Debug)]
#[cfg_attr(feature = "enable_typepath", derive(TypePath))]
pub struct LevelBundleLoadSettings {
    bg_color: bool,
    bg_image: bool,
    layers: bool,
    entities: bool,
}

impl LevelBundleLoadSettings {
    pub fn bg_color(&self) -> bool {
        self.bg_color
    }

    pub fn bg_image(&self) -> bool {
        self.bg_image
    }

    pub fn layers(&self) -> bool {
        self.layers
    }

    pub fn entities(&self) -> bool {
        self.entities
    }
}

impl Default for LevelBundleLoadSettings {
    fn default() -> Self {
        Self {
            bg_color: true,
            bg_image: true,
            layers: true,
            entities: true,
        }
    }
}

#[derive(Bundle, Debug, Default)]
pub struct LevelBundle {
    pub level: Handle<LevelAsset>,
    pub load_settings: LevelBundleLoadSettings,
}

#[derive(Asset, Debug, TypePath)]
pub struct LevelAsset {
    pub(crate) project_handle: Handle<crate::project::ProjectAsset>,
    pub(crate) iid: String,
}
