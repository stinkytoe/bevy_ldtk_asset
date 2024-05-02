use bevy::prelude::*;
use bevy::utils::thiserror;
use thiserror::Error;

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
    images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result<(), NewEntityLayerBundleError> {
    for (entity, project_handle, layer_component, settings) in new_entity_layer_query.iter() {
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

        commands.entity(entity).with_children(|parent| {
            layer_instance_json
                .entity_instances
                .iter()
                .for_each(|entity_instance| {
                    let mut entity_commands =
                        parent.spawn(Name::from(entity_instance.identifier.as_str()));

                    // let transform = Transform::from_xyz(entity_instance.px, y, z);
                    match settings {
                        LoadEntityLayerSettings::ComponentOnly => {
                            entity_commands.insert(SpatialBundle {
                                // visibility: todo!(),
                                // inherited_visibility: todo!(),
                                // view_visibility: todo!(),
                                // transform: todo!(),
                                // global_transform: todo!(),
                                ..default()
                            })
                        }
                        LoadEntityLayerSettings::Sprite => entity_commands.insert(SpriteBundle {
                            // sprite: todo!(),
                            // transform: todo!(),
                            // global_transform: todo!(),
                            // texture: todo!(),
                            // visibility: todo!(),
                            // inherited_visibility: todo!(),
                            // view_visibility: todo!(),
                            ..default()
                        }),
                    };
                });
        });
    }

    Ok(())
}
