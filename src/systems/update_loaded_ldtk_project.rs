use crate::prelude::LdtkProject;
use bevy::prelude::*;

pub fn update_loaded_ldtk_project(
    mut _commands: Commands,
    mut ldtk_load_events: EventReader<AssetEvent<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    asset_server: Res<AssetServer>,
) {
    for event in ldtk_load_events.read() {
        let LdtkProject {
            value: _,
            _worlds: _,
        } = {
            let AssetEvent::LoadedWithDependencies { id } = event else {
                return;
            };
            ldtk_project_assets
                .get(asset_server.get_id_handle(*id).unwrap())
                .unwrap()
        };

        // for (_world_iid, _world) in worlds {
        //     _world_iid;
        //     commands.spawn(WorldBundle::default());
        // }
    }
}
