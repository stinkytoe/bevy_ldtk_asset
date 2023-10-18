use bevy::prelude::*;

#[derive(Event)]
pub enum LdtkEvent {
    LoadWorldAllLevels {
        ldtk_project_file: String,
        world_name: String,
    },
    // LoadWorldLevel {
    //     ldtk_project_file: String,
    //     world_name: String,
    //     level_name: String,
    // },
}
