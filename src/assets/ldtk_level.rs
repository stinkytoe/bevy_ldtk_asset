use super::ldtk_project::LdtkProject;
use crate::ldtk_json;
use bevy::asset::{LoadContext, LoadState};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use std::path::PathBuf;

#[derive(Asset, TypePath, Debug)]
pub struct LdtkLevel {
    pub value: ldtk_json::Level,
    pub ldtk_project_directory: PathBuf,
    pub ldtk_extras_directory: PathBuf,
    #[dependency]
    pub project: Handle<LdtkProject>,
    #[dependency]
    pub bg_image: Option<Handle<Image>>,
}

impl LdtkLevel {
    pub fn new(
        value: ldtk_json::Level,
        ldtk_project_directory: PathBuf,
        ldtk_extras_directory: PathBuf,
        project: Handle<LdtkProject>,
        load_context: &mut LoadContext,
    ) -> Self {
        let bg_image = value
            .bg_rel_path
            .as_ref()
            .map(|bg_rel_path| load_context.load(ldtk_project_directory.join(bg_rel_path)));

        Self {
            value,
            ldtk_project_directory,
            ldtk_extras_directory,
            project,
            bg_image,
        }
    }

    pub fn is_loaded(&self, asset_server: &AssetServer) -> bool {
        matches!(
            asset_server.get_load_state(&self.project),
            Some(LoadState::Loaded)
        ) && (self.bg_image.is_none()
            || matches!(
                asset_server.get_load_state(self.bg_image.as_ref().unwrap()),
                Some(LoadState::Loaded)
            ))
    }
}
