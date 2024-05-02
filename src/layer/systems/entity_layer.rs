use bevy::prelude::*;
use bevy::utils::thiserror;
use thiserror::Error;

use crate::layer::LayerComponent;
use crate::layer::LoadEntityLayerSettings;
use crate::prelude::EntityComponent;
use crate::prelude::EntityComponentError;
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
    // images: ResMut<Assets<Image>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
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
            commands.entity(layer_entity).with_children(|parent| {
                let mut entity_commands =
                    parent.spawn(Name::from(entity_instance.identifier.as_str()));

                // let transform = Transform::from_xyz(entity_instance.px, y, z);
                let transform = Transform::from_translation(
                    (entity_component.location() * Vec2::new(1.0, -1.0)).extend(0.0),
                );

                match settings {
                    LoadEntityLayerSettings::ComponentOnly => {
                        entity_commands.insert((
                            entity_component,
                            SpatialBundle {
                                // visibility: todo!(),
                                // inherited_visibility: todo!(),
                                // view_visibility: todo!(),
                                transform,
                                // global_transform: todo!(),
                                ..default()
                            },
                        ));
                    }
                    LoadEntityLayerSettings::Sprite => {
                        entity_commands.insert((
                            entity_component,
                            SpriteBundle {
                                sprite: Sprite {
                                    // color: (),
                                    // flip_x: (),
                                    // flip_y: (),
                                    // custom_size: (),
                                    // rect: (),
                                    // anchor: (),
                                    ..default()
                                },
                                transform,
                                // global_transform: todo!(),
                                // texture: todo!(),
                                // visibility: todo!(),
                                // inherited_visibility: todo!(),
                                // view_visibility: todo!(),
                                ..default()
                            },
                        ));
                    }
                };
            });
        }
    }

    Ok(())
}