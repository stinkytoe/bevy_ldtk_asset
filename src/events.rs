use bevy::prelude::*;

#[derive(Event)]
pub enum LdtkEvent {
    LoadEverything {
        ldtk_project_file: String,
    },
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
