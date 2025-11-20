use std::path::Path;

use bevy_asset::{Handle, LoadContext};
use bevy_platform::collections::HashMap;
use futures::future::try_join_all;

use crate::enum_definition::EnumDefinition;
use crate::ldtk;
use crate::result::LdtkResult;
use crate::tileset_definition::TilesetDefinition;
use crate::uid::UidMap;

pub(super) async fn construct_enum_definitions(
    enum_definitions_json: Vec<ldtk::EnumDefinition>,
    tileset_definitions: &UidMap<Handle<TilesetDefinition>>,
    project_directory: &Path,
    load_context: &mut LoadContext<'_>,
) -> LdtkResult<HashMap<String, Handle<EnumDefinition>>> {
    let enum_definitions = enum_definitions_json
        .into_iter()
        .map(|enum_definition_json| async {
            let identifier = enum_definition_json.identifier.clone();

            let enum_definition =
                EnumDefinition::new(enum_definition_json, tileset_definitions, project_directory)
                    .await?;

            LdtkResult::Ok((identifier, enum_definition))
        });

    Ok(try_join_all(enum_definitions)
        .await?
        .into_iter()
        .map(|(uid, enum_definition)| {
            let enum_definition_label = format!("enum_definition:{}", enum_definition.identifier);
            let handle = load_context.add_labeled_asset(enum_definition_label, enum_definition);
            (uid, handle)
        })
        .collect())
}
