use bevy::math::I64Vec2;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

use crate::entity::EntityAsset;
use crate::layer::LayerAsset;
use crate::level::LevelAsset;
use crate::world::WorldAsset;

#[derive(Asset, Clone, Debug, Reflect)]
pub struct ProjectAsset {
    pub bg_color: Color,
    pub external_levels: bool,
    pub iid: String,
    pub json_version: String,

    // Indexed by iid
    pub(crate) world_assets: HashMap<String, Handle<WorldAsset>>,
    pub(crate) level_assets: HashMap<String, Handle<LevelAsset>>,
    pub(crate) layer_assets: HashMap<String, Handle<LayerAsset>>,
    pub(crate) entity_assets: HashMap<String, Handle<EntityAsset>>,

    // indexed by LDtk provided path
    pub(crate) tileset_assets: HashMap<String, Handle<Image>>,
    pub(crate) background_assets: HashMap<String, Handle<Image>>,

    //
    pub(crate) settings: ProjectSettings,
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
