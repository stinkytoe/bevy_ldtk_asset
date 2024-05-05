use bevy::prelude::*;
use thiserror::Error;

use crate::entity::EntityComponent;
use crate::entity::EntityComponentError;
use crate::layer::LayerComponent;
use crate::layer::LoadEntityLayerSettings;
use crate::project::ProjectAsset;

#[derive(Debug, Error)]
pub(crate) enum NewEntityLayerBundleError {
    #[error("Bad project handle!")]
    BadProjectHandle,
    #[error("Bad level uid!")]
    BadLevelUid(i64),
    #[error("Missing Layer Component!")]
    MissingLayerComponent,
    #[error("EntityComponentError: {0}")]
    EntityComponentError(#[from] EntityComponentError),
}

pub(crate) fn new_entity_layer_bundle(
    mut commands: Commands,
    new_entity_layer_query: Query<
        (
            Entity,
            &Handle<ProjectAsset>,
            &LayerComponent,
            &LoadEntityLayerSettings,
        ),
        Added<LoadEntityLayerSettings>,
    >,
    project_assets: Res<Assets<ProjectAsset>>,
) -> Result<(), NewEntityLayerBundleError> {
    for (layer_entity, project_handle, layer_component, settings) in new_entity_layer_query.iter() {
        let project_asset = project_assets
            .get(project_handle)
            .ok_or(NewEntityLayerBundleError::BadProjectHandle)?;

        let level_json = project_asset
            .get_level_by_uid(layer_component.level_id())
            .ok_or(NewEntityLayerBundleError::BadLevelUid(
                layer_component.level_id(),
            ))?;

        let layer_instance_json = project_asset
            .get_layer_instance_by_level_layer_iid(&level_json.iid, layer_component.iid())
            .ok_or(NewEntityLayerBundleError::MissingLayerComponent)?;

        debug!("EntityLayerBundle loaded! {}", layer_component.identifier());

        for entity_instance in layer_instance_json.entity_instances.iter() {
            let entity_component: EntityComponent = entity_instance.try_into()?;

            let transform = Transform::from_translation(
                (entity_component.location() * Vec2::new(1.0, -1.0)).extend(0.0),
            );

            commands.entity(layer_entity).with_children(|parent| {
                let tileset_rectangle = entity_component.tile().cloned();

                let mut entity_commands = parent.spawn((
                    Name::from(entity_instance.identifier.as_str()),
                    project_handle.clone(),
                    entity_component,
                    SpatialBundle {
                        transform,
                        ..default()
                    },
                ));

                match (settings, tileset_rectangle) {
                    (LoadEntityLayerSettings::ComponentOnly, _)
                    | (LoadEntityLayerSettings::Sprite, None) => {}
                    (LoadEntityLayerSettings::Sprite, Some(tileset_rectangle)) => {
                        entity_commands.insert(tileset_rectangle);
                    }
                };
            });
        }

        commands
            .entity(layer_entity)
            .insert(Name::from(layer_component.identifier()));
    }

    Ok(())
}
