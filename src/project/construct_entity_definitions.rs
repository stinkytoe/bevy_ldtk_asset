use bevy_asset::{Handle, LoadContext};
use futures::future::try_join_all;

use crate::entity_definition::EntityDefinition;
use crate::ldtk::{self};
use crate::result::LdtkResult;
use crate::tileset_definition::TilesetDefinition;
use crate::uid::UidMap;

pub(super) async fn construct_entity_definitions(
    entity_definitions_json: Vec<ldtk::EntityDefinition>,
    tileset_definitions: &UidMap<Handle<TilesetDefinition>>,
    load_context: &mut LoadContext<'_>,
) -> LdtkResult<UidMap<Handle<EntityDefinition>>> {
    let entity_definitions =
        entity_definitions_json
            .into_iter()
            .map(|entity_definition_json| async {
                let uid = entity_definition_json.uid;

                let entity_definition =
                    EntityDefinition::new(entity_definition_json, tileset_definitions).await?;

                LdtkResult::Ok((uid, entity_definition))
            });

    Ok(try_join_all(entity_definitions)
        .await?
        .into_iter()
        .map(|(uid, entity_definition)| {
            let label = format!("entity_definition:{}", entity_definition.identifier);
            let handle = load_context.add_labeled_asset(label, entity_definition);
            (uid, handle)
        })
        .collect())
}
