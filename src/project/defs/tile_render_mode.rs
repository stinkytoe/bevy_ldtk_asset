use bevy::prelude::*;

use crate::ldtk;

#[derive(Debug, Reflect)]
pub enum TileRenderMode {
    Cover,
    FitInside,
    FullSizeCropped,
    FullSizeUncropped,
    NineSlice,
    Repeat,
    Stretch,
}

impl From<&ldtk::TileRenderMode> for TileRenderMode {
    fn from(value: &ldtk::TileRenderMode) -> Self {
        match value {
            ldtk::TileRenderMode::Cover => Self::Cover,
            ldtk::TileRenderMode::FitInside => Self::FitInside,
            ldtk::TileRenderMode::FullSizeCropped => Self::FullSizeCropped,
            ldtk::TileRenderMode::FullSizeUncropped => Self::FullSizeUncropped,
            ldtk::TileRenderMode::NineSlice => Self::NineSlice,
            ldtk::TileRenderMode::Repeat => Self::Repeat,
            ldtk::TileRenderMode::Stretch => Self::Stretch,
        }
    }
}
