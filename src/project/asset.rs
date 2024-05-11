use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::ldtk;
use crate::level::LevelAsset;
use crate::world::WorldAsset;

pub(crate) trait ProjectResolver {
    fn value(&self) -> &ldtk::LdtkJson;
    fn single_world(&self) -> &Vec<ldtk::World>;
    fn levels(&self) -> &HashMap<String, Vec<ldtk::Level>>;

    fn get_worlds(&self) -> impl Iterator<Item = &ldtk::World> {
        if self.single_world().is_empty() {
            self.value().worlds.iter()
        } else {
            self.single_world().iter()
        }
    }

    fn get_world_by_iid(&self, iid: &str) -> Option<&ldtk::World> {
        self.get_worlds().find(|world| world.iid == iid)
    }

    fn get_levels_by_world_iid(&self, world_iid: &str) -> impl Iterator<Item = &ldtk::Level> {
        if self.value().external_levels {
            if let Some(levels) = self.levels().get(world_iid) {
                levels.iter()
            } else {
                [].iter()
            }
        } else if let Some(world) = self.get_world_by_iid(world_iid) {
            world.levels.iter()
        } else {
            [].iter()
        }
    }
}

#[derive(Debug)]
pub(crate) struct ProjectStub {
    pub(crate) value: ldtk::LdtkJson,
    pub(crate) single_world: Vec<ldtk::World>,
    pub(crate) external_levels: HashMap<String, Vec<ldtk::Level>>,
}

impl ProjectResolver for ProjectStub {
    fn value(&self) -> &ldtk::LdtkJson {
        &self.value
    }

    fn single_world(&self) -> &Vec<ldtk::World> {
        &self.single_world
    }

    fn levels(&self) -> &HashMap<String, Vec<ldtk::Level>> {
        &self.external_levels
    }
}

#[derive(Asset, Debug)]
#[cfg_attr(not(feature = "enable_reflect"), derive(TypePath))]
#[cfg_attr(
    feature = "enable_reflect",
    derive(Reflect),
    reflect(from_reflect = false)
)]
pub struct ProjectAsset {
    #[cfg_attr(feature = "enable_reflect", reflect(ignore))]
    pub(crate) value: ldtk::LdtkJson,
    // If it's NOT a multi world project, then explicitly create an ldtk::World
    // and store it here
    #[cfg_attr(feature = "enable_reflect", reflect(ignore))]
    pub(crate) single_world: Vec<ldtk::World>,
    // If this is an external levels project, store the ldtk::level objects here
    #[cfg_attr(feature = "enable_reflect", reflect(ignore))]
    pub(crate) external_levels: HashMap<String, Vec<ldtk::Level>>,
    pub(crate) world_handles: HashMap<String, Handle<WorldAsset>>,
    pub(crate) level_handles: HashMap<String, Handle<LevelAsset>>,
    pub(crate) tileset_handles: HashMap<String, Handle<Image>>,
    pub(crate) background_handles: HashMap<String, Handle<Image>>,
}

impl ProjectResolver for ProjectAsset {
    fn value(&self) -> &ldtk::LdtkJson {
        &self.value
    }

    fn single_world(&self) -> &Vec<ldtk::World> {
        &self.single_world
    }

    fn levels(&self) -> &HashMap<String, Vec<ldtk::Level>> {
        &self.external_levels
    }
}

impl ProjectAsset {
    pub(crate) fn get_level_by_iid(&self, iid: &str) -> Option<&ldtk::Level> {
        self.get_worlds()
            .flat_map(|world| self.get_levels_by_world_iid(&world.iid))
            .find(|level| level.iid == iid)
    }

    pub(crate) fn get_level_by_uid(&self, uid: i64) -> Option<&ldtk::Level> {
        self.get_worlds()
            .flat_map(|world| self.get_levels_by_world_iid(&world.iid))
            .find(|level| level.uid == uid)
    }

    pub(crate) fn get_layer_instance_by_level_layer_iid(
        &self,
        level_iid: &str,
        layer_iid: &str,
    ) -> Option<&ldtk::LayerInstance> {
        self.get_level_by_iid(level_iid)
            .and_then(|level| level.layer_instances.as_ref())
            .and_then(|layer_instances| {
                layer_instances
                    .iter()
                    .find(|layer_instance| layer_instance.iid == layer_iid)
            })
    }

    pub(crate) fn get_world_handle(&self, world_iid: &str) -> Option<&Handle<WorldAsset>> {
        self.world_handles.get(world_iid)
    }

    pub(crate) fn get_level_handle(&self, level_iid: &str) -> Option<&Handle<LevelAsset>> {
        self.level_handles.get(level_iid)
    }

    pub(crate) fn get_tileset_handle(&self, path: &str) -> Option<&Handle<Image>> {
        self.tileset_handles.get(path)
    }

    pub(crate) fn get_background_handle(&self, path: &str) -> Option<&Handle<Image>> {
        self.background_handles.get(path)
    }

    pub(crate) fn get_tileset_definition_by_uid(
        &self,
        uid: i64,
    ) -> Option<&ldtk::TilesetDefinition> {
        self.value
            .defs
            .tilesets
            .iter()
            .find(|tileset_def| tileset_def.uid == uid)
    }
}
