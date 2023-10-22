use crate::assets::structs::world::World;
use crate::ldtk_json;
use bevy::{prelude::*, reflect::TypePath};
use std::collections::HashMap;
// use serde::Deserialize;
// use super::ldtk_level::LdtkLevel;
// use std::collections::HashMap;

// pub type WorldIid = String;
// pub type LevelIid = String;

// #[derive(Debug, TypePath, TypeUuid)]
// #[uuid = "e131e69d-f619-4fec-9fa5-71fef82f9c81"]
#[derive(Asset, Debug, TypePath)]
pub struct LdtkProject {
    // pub(crate) _bg_color: Color, // do we need this?
    // pub(crate) level_backgrounds: HashMap<LevelIid, Handle<Image>>,
    // pub(crate) level_file_handles: HashMap<LevelIid, Handle<LdtkLevel>>,
    // pub(crate) tilesets: HashMap<i64, Handle<Image>>,
    pub(crate) value: ldtk_json::LdtkJson,
    pub(crate) _worlds: HashMap<String, World>,
    // pub(crate) world_level_map: HashMap<WorldIid, Vec<LevelIid>>,
}

impl LdtkProject {
    pub fn json(&self) -> &ldtk_json::LdtkJson {
        &self.value
    }
    //
    // pub fn get_world_iids(&self) -> impl Iterator<Item = &WorldIid> {
    //     self.world_level_map.keys()
    // }
    //
    // pub fn get_level_iids(&self) -> impl Iterator<Item = &LevelIid> {
    //     self.world_level_map.values().flatten()
    // }
    //
    // pub fn has_level(&self, iid: LevelIid) -> bool {
    //     self.world_level_map
    //         .values()
    //         .any(|level_iids| level_iids.contains(&iid))
    // }
    //
    // pub fn get_level(&self, iid: LevelIid) -> Option<&ldtk_json::Level> {
    //     if self.value.worlds.is_empty() {
    //         self.value.levels.iter().collect::<Vec<&ldtk_json::Level>>()
    //     } else {
    //         self.value
    //             .worlds
    //             .iter()
    //             .flat_map(|world| &world.levels)
    //             .collect::<Vec<&ldtk_json::Level>>()
    //     }
    //     .iter()
    //     .find(|level| level.iid == iid)
    //     .copied()
    // }
}
