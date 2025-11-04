use bevy_asset::Handle;
use bevy_asset::LoadContext;
use futures::future::try_join_all;

use crate::layer_definition::LayerDefinition;
use crate::ldtk;
use crate::result::Result;
use crate::tileset_definition::TilesetDefinition;
use crate::uid::UidMap;

pub(super) async fn construct_layer_definitions(
    layer_definitions: Vec<ldtk::LayerDefinition>,
    tileset_definitions: &UidMap<Handle<TilesetDefinition>>,
    load_context: &mut LoadContext<'_>,
) -> Result<UidMap<Handle<LayerDefinition>>> {
    let layer_definitions = layer_definitions
        .into_iter()
        .map(|layer_definition_json| async {
            let uid = layer_definition_json.uid;

            let layer_definition =
                LayerDefinition::new(layer_definition_json, tileset_definitions).await?;

            Result::Ok((uid, layer_definition))
        });

    Ok(try_join_all(layer_definitions)
        .await?
        .into_iter()
        .map(|(uid, layer_definition)| {
            let layer_definition_label =
                format!("layer_definition:{}", layer_definition.identifier);
            let layer_definition =
                load_context.add_labeled_asset(layer_definition_label, layer_definition);
            (uid, layer_definition)
        })
        .collect())
}
