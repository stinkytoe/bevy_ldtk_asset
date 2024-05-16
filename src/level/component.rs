use bevy::prelude::*;
use thiserror::Error;

use crate::ldtk;

#[derive(Component, Debug, Default, Reflect)]
pub struct LevelComponent {}

#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct LevelBackgroundPosition {
    pub crop_top_left: Vec2,
    pub crop_bottom_right: Vec2,
    pub scale: Vec2,
    pub top_left: Vec2,
}

impl From<&ldtk::LevelBackgroundPosition> for LevelBackgroundPosition {
    fn from(value: &ldtk::LevelBackgroundPosition) -> Self {
        let crop_top_left = (value.crop_rect[0] as f32, value.crop_rect[1] as f32).into();
        let crop_bottom_right =
            crop_top_left + Vec2::new(value.crop_rect[2] as f32, value.crop_rect[3] as f32);
        let scale = (value.scale[0] as f32, value.scale[1] as f32).into();
        let top_left = (value.top_left_px[0] as f32, value.top_left_px[1] as f32).into();

        Self {
            crop_top_left,
            crop_bottom_right,
            scale,
            top_left,
        }
    }
}

#[derive(Debug, Error)]
pub enum NeighbourError {
    #[error("Given unknown neighbour string from LDtk project! {0}")]
    BadString(String),
}

#[derive(Clone, Debug, Reflect)]
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

#[derive(Clone, Debug, Reflect)]
pub struct Neighbour {
    pub level_iid: String,
    pub dir: NeighbourDir,
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

#[derive(Clone, Component, Debug, Reflect)]
pub struct Neighbours {
    pub neighbours: Vec<Neighbour>,
}
