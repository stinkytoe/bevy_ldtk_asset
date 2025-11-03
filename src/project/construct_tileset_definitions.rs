use std::path::Path;

use bevy_asset::Handle;
use bevy_asset::LoadContext;
use futures::future::try_join_all;

use crate::ldtk;
use crate::ldtk_path::ldtk_path_to_bevy_path;
use crate::result::Result;
use crate::tileset_definition::TilesetDefinition;
use crate::uid::UidMap;

pub(super) async fn construct_tileset_definitions(
    project_directory: &Path,
    tileset_definitions: Vec<ldtk::TilesetDefinition>,
    load_context: &mut LoadContext<'_>,
) -> Result<UidMap<Handle<TilesetDefinition>>> {
    let tileset_definition_images = tileset_definitions
        .iter()
        .filter_map(|ldtk_tileset_definition| ldtk_tileset_definition.rel_path.clone())
        .map(|rel_path| {
            let tileset_image = {
                let bevy_path = ldtk_path_to_bevy_path(project_directory, &rel_path);
                load_context.load(bevy_path)
            };

            (rel_path, tileset_image)
        })
        .collect();

    let tileset_definitions =
        tileset_definitions
            .into_iter()
            .map(|tileset_definition_json| async {
                let uid = tileset_definition_json.uid;

                let tileset_definition =
                    TilesetDefinition::new(tileset_definition_json, &tileset_definition_images)
                        .await?;

                Result::Ok((uid, tileset_definition))
            });

    Ok(try_join_all(tileset_definitions)
        .await?
        .into_iter()
        .map(|(uid, tileset_definition)| {
            let tileset_definition_label =
                format!("tileset_definition:{}", tileset_definition.identifier);
            let tileset_definition =
                load_context.add_labeled_asset(tileset_definition_label, tileset_definition);
            (uid, tileset_definition)
        })
        .collect())
}
