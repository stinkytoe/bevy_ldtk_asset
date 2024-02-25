use std::path::PathBuf;

use bevy::{prelude::*, utils::HashMap};

use super::world::LdtkWorld;

#[derive(Asset, Debug, TypePath)]
pub struct ProjectAsset {
    pub(crate) asset_path: PathBuf,
    pub(crate) base_directory: PathBuf,
    pub(crate) exports_directory: PathBuf,
    #[dependency]
    pub(crate) tilesets: Vec<Handle<Image>>,
    pub(crate) worlds: HashMap<String, Handle<LdtkWorld>>,
}

impl ProjectAsset {
    pub fn worlds(&self) -> impl Iterator<Item = &String> {
        self.worlds.keys()
    }

    pub fn world_handles(&self) -> impl Iterator<Item = &Handle<LdtkWorld>> {
        self.worlds.values()
    }

    pub fn tilesets(&self) -> impl Iterator<Item = &Handle<Image>> {
        self.tilesets.iter()
    }
}
