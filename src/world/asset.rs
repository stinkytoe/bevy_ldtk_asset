use bevy::prelude::*;
use bevy::utils::HashMap;
use thiserror::Error;

use crate::ldtk;
use crate::ldtk::WorldLayout;
use crate::level::LevelAsset;
use crate::project::ProjectAsset;

#[derive(Debug, Error)]
pub(crate) enum NewWorldAssetError {
    // #[error("Field is None when parsing a single world project? field: {0}")]
    // NoneInSingleWorldProject(String),
    #[error("Missing worldLayout?")]
    MissingWorldLayout,
}

#[derive(Asset, Clone, Debug, Reflect)]
pub struct WorldAsset {
    pub identifier: String,
    pub iid: String,
    pub world_grid_size: Vec2,
    pub world_layout: WorldLayout,
    #[reflect(ignore)]
    pub project: Handle<ProjectAsset>,
    pub level_assets_by_identifier: HashMap<String, Handle<LevelAsset>>,
    pub level_assets_by_iid: HashMap<String, Handle<LevelAsset>>,
}

impl WorldAsset {
    pub(crate) fn new(
        value: &ldtk::World,
        project: Handle<ProjectAsset>,
        level_assets_by_identifier: HashMap<String, Handle<LevelAsset>>,
        level_assets_by_iid: HashMap<String, Handle<LevelAsset>>,
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
            level_assets_by_identifier,
            level_assets_by_iid,
        })
    }
}
