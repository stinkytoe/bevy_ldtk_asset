use bevy::prelude::*;
use thiserror::Error;

use crate::ldtk;
use crate::ldtk::WorldLayout;
use crate::level::LayersToLoad;
use crate::level::LevelAsset;
use crate::level::LevelBundle;
use crate::project::ProjectAsset;
use crate::traits::AssetProvidesProjectHandle;
use crate::traits::DependencyLoader;

use super::component::LevelsToLoad;

#[derive(Debug, Error)]
pub(crate) enum NewWorldAssetError {
    #[error("Field is None when parsing a single world project? field: {0}")]
    NoneInSingleWorldProject(String),
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

        let world_layout = value
            .world_layout
            .clone()
            .ok_or(NewWorldAssetError::MissingWorldLayout)?;

        Ok(Self {
            identifier: "World".to_string(),
            iid: value.iid.clone(),
            world_grid_size,
            world_layout,
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

impl AssetProvidesProjectHandle for WorldAsset {
    fn project_handle(&self) -> &Handle<ProjectAsset> {
        &self.project
    }
}

impl DependencyLoader for WorldAsset {
    type Child = LevelAsset;
    type ChildrenToLoad = LevelsToLoad;
    type GrandchildrenToLoad = LayersToLoad;

    fn next_tier(
        &self,
        project_asset: &ProjectAsset,
        to_load: &Self::ChildrenToLoad,
    ) -> Result<
        bevy::utils::HashMap<Handle<Self::Child>, Self::GrandchildrenToLoad>,
        crate::traits::DependencyLoaderError,
    > {
        match to_load {
            LevelsToLoad::None => Self::merge_empty(),
            LevelsToLoad::ByIdentifiers(ids) => {
                Self::merge_filtered(ids, &project_asset.level_assets_by_identifier)
            }
            LevelsToLoad::ByIids(ids) => {
                Self::merge_filtered(ids, &project_asset.level_assets_by_iid)
            }
            LevelsToLoad::All(levels_to_load) => {
                Self::merge_all(levels_to_load, &project_asset.level_assets_by_iid)
            }
        }
    }

    fn spawn_child(
        child_builder: &mut ChildBuilder,
        level: Handle<Self::Child>,
        layers_to_load: Self::GrandchildrenToLoad,
    ) {
        child_builder.spawn(LevelBundle {
            level,
            layers_to_load,
            spatial: SpatialBundle::default(),
        });
    }
}
