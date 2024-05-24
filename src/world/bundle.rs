use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::level::LevelsToLoad;
use crate::project::ProjectAsset;
use crate::traits::{ToLoad, ToLoadError};
use crate::world::WorldAsset;

#[derive(Component, Debug, Reflect)]
pub enum WorldsToLoad {
    None,
    ByIdentifiers(HashMap<String, LevelsToLoad>),
    ByIids(HashMap<String, LevelsToLoad>),
    All(LevelsToLoad),
}

impl Default for WorldsToLoad {
    fn default() -> Self {
        Self::All(LevelsToLoad::default())
    }
}

impl ToLoad<ProjectAsset, WorldAsset, LevelsToLoad> for WorldsToLoad {
    fn next_tier(
        &self,
        // assets_by_id: &HashMap<String, Handle<Child>>,
        parent_asset: &ProjectAsset,
    ) -> Result<HashMap<Handle<WorldAsset>, LevelsToLoad>, ToLoadError> {
        match self {
            WorldsToLoad::None => Self::merge_empty(),
            WorldsToLoad::ByIdentifiers(ids) => {
                Self::merge_filtered(ids, &parent_asset.world_assets_by_identifier)
            }
            WorldsToLoad::ByIids(ids) => {
                Self::merge_filtered(ids, &parent_asset.world_assets_by_iid)
            }
            WorldsToLoad::All(levels_to_load) => {
                Self::merge_all(levels_to_load, &parent_asset.world_assets_by_iid)
            }
        }
    }

    fn spawn_child(
        child_builder: &mut ChildBuilder,
        world: Handle<WorldAsset>,
        levels_to_load: LevelsToLoad,
    ) {
        child_builder.spawn(WorldBundle {
            world,
            levels_to_load,
            spatial: SpatialBundle::default(),
        });
    }
}

#[derive(Bundle, Debug, Default)]
pub struct WorldBundle {
    pub(crate) world: Handle<WorldAsset>,
    pub(crate) levels_to_load: LevelsToLoad,
    pub(crate) spatial: SpatialBundle,
}
