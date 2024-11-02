use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::tasks::block_on;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Init,
    Run,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: Level::WARN,
                filter: "bevy_ldtk_asset=debug,example=trace".into(),
                ..default()
            }),
            //WorldInspectorPlugin::default(),
            BevyLdtkAssetPlugin,
        ))
        .init_state::<AppState>()
        .add_systems(Startup, startup)
        .add_systems(Update, wait_for_project.run_if(in_state(AppState::Init)))
        .register_type::<Iid>()
        .run();
}

#[derive(Component, Debug, Reflect)]
pub struct LdtkProject {
    project: Handle<ldtk_asset::Project>,
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(LdtkProject {
        project: asset_server.load::<ldtk_asset::Project>("ldtk/top_down.ldtk"),
    });
}

fn wait_for_project(
    asset_server: Res<AssetServer>,
    projects: Res<Assets<ldtk_asset::Project>>,
    query: Query<&LdtkProject>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    query.iter().for_each(|project| {
        //block_on(async { asset_server.wait_for_asset(&project.project).await }).unwrap();

        let _project = projects.get(project.project.id()).unwrap();

        info!("project loaded!");

        next_state.set(AppState::Run);
    });
}
