use bevy::prelude::*;
use bevy::utils::thiserror;
use thiserror::Error;

use crate::ldtk;

#[derive(Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
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
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
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
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
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
