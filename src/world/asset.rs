use bevy::prelude::*;
use thiserror::Error;

use crate::ldtk::{self, WorldLayout};
use crate::project::ProjectAsset;

#[derive(Debug, Error)]
pub(crate) enum NewWorldAssetError {
    #[error("Field is None when parsing a single world project? field: {0}")]
    NoneInSingleWorldProject(String),
    #[error("Missing worldLayout?")]
    MissingWorldLayout,
}

#[derive(Asset, Debug, Reflect)]
pub struct WorldAsset {
    pub identifier: String,
    pub iid: String,
    pub world_grid_size: Vec2,
    pub world_layout: WorldLayout,

    #[reflect(ignore)]
    pub project: Handle<ProjectAsset>,
}

impl WorldAsset {
    pub(crate) fn new_from_project(
        value: &ldtk::LdtkJson,
        project: Handle<ProjectAsset>,
    ) -> Result<Self, NewWorldAssetError> {
        let world_grid_size = Vec2::new(
            value
                .world_grid_width
                .ok_or(NewWorldAssetError::NoneInSingleWorldProject(
                    "world_grid_width".to_string(),
                ))? as f32,
            value
                .world_grid_height
                .ok_or(NewWorldAssetError::NoneInSingleWorldProject(
                    "world_grid_height".to_string(),
                ))? as f32,
        );

        Ok(Self {
            identifier: "World".to_string(),
            iid: value.iid.clone(),
            world_grid_size,
            world_layout: value
                .world_layout
                .clone()
                .ok_or(NewWorldAssetError::MissingWorldLayout)?,
            project,
        })
    }

    pub(crate) fn new_from_world(
        value: &ldtk::World,
        project: Handle<ProjectAsset>,
    ) -> Result<Self, NewWorldAssetError> {
        let world_grid_size = Vec2::new(
            value.world_grid_width as f32,
            value.world_grid_height as f32,
        );

        Ok(Self {
            identifier: value.identifier.clone(),
            iid: value.iid.clone(),
            world_grid_size,
            world_layout: value
                .world_layout
                .clone()
                .ok_or(NewWorldAssetError::MissingWorldLayout)?,
            project,
        })
    }
}
