use super::ldtk_project::LdtkProject;
use crate::ldtk_json;
use bevy::asset::{LoadContext, LoadState};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use std::path::PathBuf;

/// The asset which represents an LDtk level instance.
#[derive(Asset, TypePath, Debug)]
pub struct LdtkLevel {
    /// The rust representation of the LDtk level JSON definition [ldtk_json::Level]
    pub value: ldtk_json::Level,
    /// The directory where the ldtk file resides. Use this with `.join(...)`
    /// for the path of an asset referenced in the LDtk JSON, to get it's path
    /// relative to the Bevy assets folder.
    pub ldtk_project_directory: PathBuf,
    /// The directory where 'extras' are stored for a given LDtk project. i.e. external
    /// levels, exported backgrounds and layer images, etc
    pub ldtk_extras_directory: PathBuf,
    /// A handle to the project which this level belongs in. A [LdtkLevel] is only valid if
    /// loaded from a ldtk project file [LdtkProject], even if the project is configured to
    /// have external level files.
    #[dependency]
    pub project: Handle<LdtkProject>,
    /// An optional handle to the defined background image, if any, for the level.
    #[dependency]
    pub bg_image: Option<Handle<Image>>,
}

impl LdtkLevel {
    pub(crate) fn new(
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

    pub(crate) fn is_loaded(&self, asset_server: &AssetServer) -> bool {
        // TODO: This is ugly!
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
