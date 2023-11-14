use crate::assets::ldtk_level::LdtkLevel;
use bevy::prelude::*;

pub(crate) fn handle_new_level(
    mut commands: Commands,
    changed_level_query: Query<(Entity, &Handle<LdtkLevel>), Changed<Handle<LdtkLevel>>>,
    ldtk_level_assets: Res<Assets<LdtkLevel>>,
) {
    for (entity, level_handle) in changed_level_query.iter() {
        let level = {
            let Some(asset) = ldtk_level_assets.get(level_handle) else {
                error!("bad handle?");
                return;
            };

            asset
        };

        info!("level loaded: {:#?}", level.value.identifier);
        commands
            .entity(entity)
            .insert(Name::from(level.value.identifier.to_owned()));

        // let callback = |mut _commands: Commands| {};
        // commands.run_system(callback);
    }
}
