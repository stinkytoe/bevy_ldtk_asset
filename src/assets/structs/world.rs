use std::collections::HashMap;

use crate::util::ColorParseError;
use crate::{assets, ldtk_json};
use bevy::prelude::*;
use bevy::utils::thiserror;
use thiserror::Error;

#[derive(Clone, Debug)]
pub(crate) struct World {
    pub(crate) _identifier: String,
    pub(crate) iid: String,
    pub(crate) levels: HashMap<String, assets::structs::level::Level>,
    pub(crate) _world_grid_height: i64,
    pub(crate) _world_grid_width: i64,
    pub(crate) _world_layout: ldtk_json::WorldLayout,
}

#[derive(Debug, Error)]
pub(crate) enum WorldError {
    #[error("Bad Color Value: {0}")]
    BadColor(#[from] ColorParseError),
    #[error("world_grid_(width|height) as None in GridVania layout")]
    BadGridVania,
    #[error("Possible attempt to parse root as world, in multiworld context")]
    ParseRootAsWorld,
    #[error("Parsing a world in a multiworld context failed")]
    ParseMultiWorld,
}

impl TryFrom<&ldtk_json::LdtkJson> for World {
    type Error = WorldError;

    fn try_from(value: &ldtk_json::LdtkJson) -> Result<Self, Self::Error> {
        let world_grid_helper = |world_grid_value: Option<i64>,
                                 which: &str|
         -> Result<i64, Self::Error> {
            if let Some(world_grid_value) = world_grid_value {
                Ok(world_grid_value)
            } else if let Some(world_layout) = value.world_layout.as_ref() {
                if *world_layout == ldtk_json::WorldLayout::GridVania {
                    error!("worldGrid{} was None while in GridVania layout!", which);
                    Err(WorldError::BadGridVania)
                } else {
                    Ok(0)
                }
            } else {
                error!("WorldGrid{which} and WorldLayout both None! Did we call this on a multiroot project?");
                Err(WorldError::ParseRootAsWorld)
            }
        };

        let world_layout_helper = |world_layout: Option<&ldtk_json::WorldLayout>| -> Result<ldtk_json::WorldLayout, Self::Error> {
            if let Some(world_layout) = world_layout {
                Ok(world_layout.clone())
            } else {
                error!("World Layout None! Did we call this on a multiroot project?");
                Err(WorldError::ParseRootAsWorld)
            }
        };

        Ok(Self {
            _identifier: "(root)".to_string(),
            iid: value.iid.clone(),
            levels: HashMap::default(),
            _world_grid_height: world_grid_helper(value.world_grid_height, "Height")?,
            _world_grid_width: world_grid_helper(value.world_grid_width, "Width")?,
            _world_layout: world_layout_helper(value.world_layout.as_ref())?,
        })
    }
}

impl TryFrom<&ldtk_json::World> for World {
    type Error = WorldError;

    fn try_from(value: &ldtk_json::World) -> Result<Self, Self::Error> {
        let world_layout_helper = |world_layout: Option<&ldtk_json::WorldLayout>| -> Result<ldtk_json::WorldLayout, Self::Error> {
            if let Some(world_layout) = world_layout {
                Ok(world_layout.clone())
            } else {
                error!("World Layout None! This shouldn't happen in a multiworld context!");
                Err(WorldError::ParseMultiWorld)
            }
        };

        Ok(Self {
            _identifier: value.identifier.clone(),
            iid: value.iid.clone(),
            levels: HashMap::default(),
            _world_grid_height: value.world_grid_height,
            _world_grid_width: value.world_grid_width,
            _world_layout: world_layout_helper(value.world_layout.as_ref())?,
        })
    }
}
