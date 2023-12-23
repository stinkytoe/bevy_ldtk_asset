use std::path::PathBuf;

use crate::ldtk_json;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use serde::Deserialize;

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct LdtkLevel {
    pub value: ldtk_json::Level,
    pub ldtk_sub_files_dir: PathBuf,
    // pub ldtk_project_dir: PathBuf,
}

// impl LdtkLevel {
//     pub fn new(
//         value: ldtk_json::Level,
//         // ldtk_project_dir: PathBuf,
//         ldtk_sub_files_dir: PathBuf,
//     ) -> Self {
//         // debug!("loaded LdtkLevel with dir: {dir:?}");
//         // debug!(
//         // 	"for fun: {:?}",
//         // 	dir.join("../").join("../basic_tileset_and_assets_standard")
//         // );
//         Self {
//             value,
//             // ldtk_project_dir,
//             ldtk_sub_files_dir,
//         }
//     }
// }
