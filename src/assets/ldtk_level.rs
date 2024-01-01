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
    /// A table of all references layer definitions. This table is in the same order as the
    /// self.value.layer_instances member and can be used to get the definition of a layer
    /// instance with the same index
    pub layer_definitions: Vec<ldtk_json::LayerDefinition>,
    /// A handle to the project which this level belongs in. A [LdtkLevel] is only valid if
    /// loaded from a ldtk project file [LdtkProject], even if the project is configured to
    /// have external level files.
    #[dependency]
    pub project_handle: Handle<LdtkProject>,
    /// An optional handle to the defined background image, if any, for the level.
    #[dependency]
    pub bg_image: Option<Handle<Image>>,
}

impl LdtkLevel {
    pub(crate) fn new(
        value: ldtk_json::Level,
        ldtk_project_directory: PathBuf,
        ldtk_extras_directory: PathBuf,
        project_json: &ldtk_json::LdtkJson,
        project_handle: Handle<LdtkProject>,
        load_context: &mut LoadContext,
    ) -> Self {
        let layer_definitions = value
            .layer_instances
            .as_ref()
            .map(|layer_instances| {
                layer_instances
                    .iter()
                    .map(|layer_instance| {
                        project_json
                            .defs
                            .layers
                            .iter()
                            .find(|layer_definition| {
                                layer_definition.uid == layer_instance.layer_def_uid
                            })
                            .expect("could not find a layer definition for a given layer instance!")
                    })
                    .cloned()
                    .collect::<Vec<ldtk_json::LayerDefinition>>()
            })
            .expect("could not build table of layer definitions!");

        let bg_image = value
            .bg_rel_path
            .as_ref()
            .map(|bg_rel_path| load_context.load(ldtk_project_directory.join(bg_rel_path)));

        Self {
            value,
            ldtk_project_directory,
            ldtk_extras_directory,
            layer_definitions,
            project_handle,
            bg_image,
        }
    }

    pub(crate) fn is_loaded(&self, asset_server: &AssetServer) -> bool {
        // TODO: This is ugly!
        matches!(
            asset_server.get_load_state(&self.project_handle),
            Some(LoadState::Loaded)
        ) && (self.bg_image.is_none()
            || matches!(
                asset_server.get_load_state(self.bg_image.as_ref().unwrap()),
                Some(LoadState::Loaded)
            ))
    }

    /// In level space, finds the top-most int grid at the given coordinate
    /// and returns it as `Some(..)`, or None if no int grid value at that coordinate
    pub fn get_int_grid_value_at_level_coord(
        &self,
        coord: Vec2,
    ) -> Option<&ldtk_json::IntGridValueDefinition> {
        // TODO this is kind of deep, consider refactor? maybe an in-place lambda just for clarity?
        self.value
            .layer_instances
            .as_ref()
            .and_then(|layer_instances| {
                layer_instances
                    .iter()
                    .enumerate()
                    .find_map(|(layer_index, layer_instance)| {
                        let layer_definition = &self.layer_definitions[layer_index];

                        let x_coord = coord.x.floor() as i64 / layer_definition.grid_size;
                        let y_coord = (-coord.y).floor() as i64 / layer_definition.grid_size;

                        if !(0..layer_definition.grid_size).contains(&x_coord)
                            || !(0..layer_definition.grid_size).contains(&y_coord)
                        {
                            return None;
                        }

                        let grid_index = (x_coord
                            + y_coord * self.value.px_wid / layer_definition.grid_size)
                            as usize;

                        match layer_instance.int_grid_csv.get(grid_index).copied() {
                            Some(0) => None,
                            Some(v) => Some((layer_index, v)),
                            None => None,
                        }
                    })
            })
            .and_then(|(layer_index, int_grid_index)| {
                self.layer_definitions
                    .get(layer_index)
                    .and_then(|layer_definition| {
                        layer_definition
                            .int_grid_values
                            .iter()
                            .find(|int_grid_value| int_grid_value.value == int_grid_index)
                    })
            })
    }
}
