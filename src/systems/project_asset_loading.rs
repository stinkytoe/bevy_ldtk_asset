use crate::{
    ldtk::project_component::ProjectComponent,
    prelude::{LdtkLevelBundle, ProjectAsset},
    resources::{LevelEntities, ProjectEntities},
};
use bevy::prelude::*;

pub(crate) fn process_project_loading(
    mut project_entities: ResMut<ProjectEntities>,
    mut ev_asset: EventReader<AssetEvent<ProjectAsset>>,
    level_query: Query<(Entity, &Handle<ProjectAsset>), With<ProjectComponent>>,
) {
    for ev in ev_asset.read() {
        debug!("process_project_loading ev: {ev:?}");
        if let AssetEvent::<ProjectAsset>::Added { id } = ev {
            if let Some((entity, handle)) = level_query
                .iter()
                .find(|(_entity, handle)| handle.id() == *id)
            {
                project_entities.to_load.insert((entity, handle.clone()));
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn projects_changed(
    mut commands: Commands,
    mut project_entities: ResMut<ProjectEntities>,
    mut level_entities: ResMut<LevelEntities>,
    mut asset_server: ResMut<AssetServer>,
    project_assets: Res<Assets<ProjectAsset>>,
) {
    if !project_entities.to_load.is_empty() {
        let to_load: Vec<_> = project_entities.to_load.drain().collect();
        to_load.iter().for_each(|(project_entity, project_handle)| {
            let ldtk_project = project_assets
                .get(project_handle)
                .expect("project handle is None?");

            if ldtk_project.is_loaded(&asset_server) {
                commands
                    .entity(*project_entity)
                    .insert(Name::from("LdtkProject"))
                    .with_children(|parent| {
                        ldtk_project.levels.iter().for_each(|level_handle| {
                            debug!("loading level handle: {level_handle:?}");
                            let level_entity = parent
                                .spawn(LdtkLevelBundle {
                                    level: level_handle.clone(),
                                    ..default()
                                })
                                .id();
                            level_entities
                                .to_load
                                .insert((level_entity, level_handle.clone()));
                            project_entities
                                .loaded
                                .insert((*project_entity, project_handle.clone()));
                        });
                    });
            } else {
                project_entities
                    .to_load
                    .insert((*project_entity, project_handle.clone()));
            }
        })
    }
}
