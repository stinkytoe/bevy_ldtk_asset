use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

use crate::project::defs::EntityDefinition;
use crate::project::defs::EnumDefinition;
use crate::project::defs::LayerDefinition;
use crate::project::defs::TilesetDefinition;
use crate::project::WorldsToLoad;
use crate::traits::AssetProvidesProjectHandle;
use crate::traits::DependencyLoader;
use crate::traits::DependencyLoaderError;
use crate::world::LevelsToLoad;
use crate::world::WorldAsset;
use crate::world::WorldBundle;

#[derive(Asset, Debug, Reflect)]
pub struct ProjectAsset {
    pub bg_color: Color,
    pub external_levels: bool,
    pub iid: String,
    pub json_version: String,

    pub(crate) world_assets_by_identifier: HashMap<String, Handle<WorldAsset>>,
    pub(crate) world_assets_by_iid: HashMap<String, Handle<WorldAsset>>,

    // indexed by LDtk provided path
    pub(crate) tileset_assets: HashMap<String, Handle<Image>>,
    pub(crate) background_assets: HashMap<String, Handle<Image>>,

    pub(crate) layer_defs: HashMap<i64, LayerDefinition>,
    pub(crate) entity_defs: HashMap<i64, EntityDefinition>,
    pub(crate) tileset_defs: HashMap<i64, TilesetDefinition>,
    pub(crate) enum_defs: HashMap<i64, EnumDefinition>,

    pub(crate) settings: ProjectSettings,
    #[reflect(ignore)]
    pub(crate) self_handle: Handle<ProjectAsset>,
}

impl AssetProvidesProjectHandle for ProjectAsset {
    fn project_handle(&self) -> &Handle<ProjectAsset> {
        &self.self_handle
    }
}

impl DependencyLoader for ProjectAsset {
    type Child = WorldAsset;
    type ChildrenToLoad = WorldsToLoad;
    type GrandchildrenToLoad = LevelsToLoad;

    fn next_tier(
        &self,
        project_asset: &ProjectAsset,
        to_load: &WorldsToLoad,
    ) -> Result<HashMap<Handle<WorldAsset>, LevelsToLoad>, DependencyLoaderError> {
        match to_load {
            WorldsToLoad::None => Self::merge_empty(),
            WorldsToLoad::ByIdentifiers(ids) => {
                Self::merge_filtered(ids, &project_asset.world_assets_by_identifier)
            }
            WorldsToLoad::ByIids(ids) => {
                Self::merge_filtered(ids, &project_asset.world_assets_by_iid)
            }
            WorldsToLoad::All(levels_to_load) => {
                Self::merge_all(levels_to_load, &project_asset.world_assets_by_iid)
            }
        }
    }

    fn spawn_child(
        child_builder: &mut ChildBuilder,
        world: Handle<Self::Child>,
        levels_to_load: Self::GrandchildrenToLoad,
    ) {
        child_builder.spawn(WorldBundle {
            world,
            levels_to_load,
            spatial: SpatialBundle::default(),
        });
    }
}

#[derive(Component, Clone, Debug, Reflect, Serialize, Deserialize)]
pub struct ProjectSettings {
    level_separation: f32,
    layer_separation: f32,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            level_separation: 10.0,
            layer_separation: 0.1,
        }
    }
}
