use crate::{bundles::LdtkBundle, components::LdtkRoot, events::LdtkEvent};
use bevy::prelude::*;

pub(crate) fn update_load_world_event(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ldtk_event: EventReader<LdtkEvent>,
) {
    for event in ldtk_event.iter() {
        match event {
            LdtkEvent::LoadEverything { ldtk_project_file } => commands.spawn(LdtkBundle {
                // _project: asset_server.load(ldtk_project_file),
                root: LdtkRoot {
                    project: asset_server.load(ldtk_project_file),
                },
                ..default()
            }),
            LdtkEvent::LoadWorldAllLevels {
                ldtk_project_file: _,
                world_name: _,
            } => todo!(),
        };
    }
}
