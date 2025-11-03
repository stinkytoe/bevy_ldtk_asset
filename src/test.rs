// use bevy_app::{App, TaskPoolPlugin};
// use bevy_asset::io::embedded::GetAssetServer as _;
// use bevy_asset::{AssetLoader, AssetPlugin, Handle, LoadState};
// use bevy_ecs::component::Component;
// use bevy_tasks::block_on;
//
// use crate::plugin::BevyLdtkAssetPlugin;
// use crate::world::World as LdtkWorld;
//
// #[test]
// #[allow(clippy::unwrap_used)]
// fn test_me() {
//     let mut app = App::new();
//
//     app.add_plugins(TaskPoolPlugin::default());
//     app.add_plugins(AssetPlugin::default());
//     app.add_plugins(BevyLdtkAssetPlugin);
//
//     let handle = app
//         .get_asset_server()
//         .load::<LdtkWorld>("ldtk/empty_single_world.ldtk#World");
//
//     #[derive(Component)]
//     struct MyComponent {
//         handle: Handle<LdtkWorld>,
//     }
//
//     let asset_loader = block_on(
//         app.world_mut()
//             .get_asset_server()
//             .get_asset_loader_with_asset_type::<LdtkWorld>(),
//     )
//     .unwrap();
//
//     // asset_loader.load(reader, meta, load_context)
//
//     // let _ = AssetLoader::<LdtkWorld>::
//
//     app.world_mut().spawn(MyComponent {
//         handle: handle.clone(),
//     });
//
//     while let Some((a, _b, _c)) = app.get_asset_server().get_load_states(handle.id()) {
//         // if let LoadState::Failed(_) = a {
//         //     break;
//         // }
//
//         if let LoadState::Loading = a {
//             break;
//         }
//
//         app.update();
//     }
// }
