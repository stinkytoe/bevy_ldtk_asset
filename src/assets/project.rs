use std::path::PathBuf;

use bevy::{prelude::*, utils::HashMap};

use super::world::WorldAsset;

#[derive(Asset, Debug, TypePath)]
pub struct ProjectAsset {
    pub(crate) asset_path: PathBuf,
    pub(crate) base_directory: PathBuf,
    pub(crate) exports_directory: PathBuf,
    #[dependency]
    pub(crate) tilesets: Vec<Handle<Image>>,
    pub(crate) worlds: HashMap<String, Handle<WorldAsset>>,
}

impl ProjectAsset {
    pub fn world_identifiers(&self) -> impl Iterator<Item = &String> {
        self.worlds.keys()
    }

    pub fn world_handles(&self) -> impl Iterator<Item = &Handle<WorldAsset>> {
        self.worlds.values()
    }

    pub fn world_names_and_handles(&self) -> impl Iterator<Item = (&String, &Handle<WorldAsset>)> {
        self.worlds.iter()
    }

    pub fn get_world_by_identifier(&self, identifier: &str) -> Option<&Handle<WorldAsset>> {
        self.worlds.get(identifier)
    }

    pub fn tilesets(&self) -> impl Iterator<Item = &Handle<Image>> {
        self.tilesets.iter()
    }

    pub fn get_asset_path(&self) -> &PathBuf {
        &self.asset_path
    }

    pub fn ldtk_path_to_asset_path(&self, path: &str) -> PathBuf {
        self.base_directory.join(path)
    }

    pub fn ldtk_export_path_to_asset_path(&self, path: &str) -> PathBuf {
        self.exports_directory.join(path)
    }
}
