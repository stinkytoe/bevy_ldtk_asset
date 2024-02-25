use std::path::PathBuf;

use bevy::{prelude::*, utils::HashMap};

use super::world::LdtkWorld;

#[derive(Asset, Debug, TypePath)]
pub struct ProjectAsset {
    pub(crate) asset_path: PathBuf,
    pub(crate) base_directory: PathBuf,
    pub(crate) exports_directory: PathBuf,
    pub(crate) worlds: HashMap<String, Handle<LdtkWorld>>,
}
