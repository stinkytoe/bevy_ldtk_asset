use crate::ldtk_json;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use std::path::PathBuf;

#[derive(Asset, TypePath, Debug)]
pub struct LdtkLevel {
    pub value: ldtk_json::Level,
    pub ldtk_sub_files_dir: PathBuf,
    #[dependency]
    pub images: Vec<Handle<Image>>,
    #[dependency]
    pub bg_image: Option<Handle<Image>>,
}

impl LdtkLevel {
    pub fn new(
        value: ldtk_json::Level,
        ldtk_sub_files_dir: PathBuf,
        images: Vec<Handle<Image>>,
        bg_image: Option<Handle<Image>>,
    ) -> Self {
        Self {
            value,
            ldtk_sub_files_dir,
            images,
            bg_image,
        }
    }
}
