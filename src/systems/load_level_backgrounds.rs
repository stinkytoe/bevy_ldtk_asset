// use bevy::prelude::*;
//
// use crate::{
//     ldtk_bundle::{LdtkBackgroundLoader, LdtkBackgrounds, LdtkRoot},
//     ldtk_project::LdtkProject,
// };

// pub fn check_for_levels_which_need_images_loaded(
//     // mut _ev_asset: EventReader<AssetEvent<Image>>,
//     // mut _assets: ResMut<Assets<Image>>,
//     _ldtk_query: Query<(Entity, &LdtkRoot)>,
// ) {
// }
//
// pub fn load_level_backgrounds(
//     mut _commands: Commands,
//     // asset_server: Res<AssetServer>,
//     _ldtk_assets: Res<Assets<LdtkProject>>,
//     mut _ldtk_query: Query<(
//         Entity,
//         &LdtkRoot,
//         &mut LdtkBackgroundLoader,
//         &mut LdtkBackgrounds,
//     )>,
// ) {
//     // for (entity, LdtkRoot { root }, mut background_loader, mut backgrounds) in ldtk_query.iter_mut()
//     // {
//     //     let Some(ldtk_project) = ldtk_assets.get(root) else {
//     //         return;
//     //     };
//     //
//     //     match background_loader.as_ref() {
//     //         LdtkBackgroundLoader::Uninitialized => {
//     //             *background_loader = LdtkBackgroundLoader::Initialized {
//     //                 backgrounds: Vec::default(),
//     //             }
//     //         }
//     //         LdtkBackgroundLoader::Initialized { backgrounds } => todo!(),
//     //         // LdtkBackgroundLoader::Uninitialized => {
//     //         //     // background_loader = LdtkBackgroundLoader::Initialized {
//     //         //     //     backgrounds Vec::default() },
//     //         //     //     ldtk_project.worlds.iter().map(|(_, world)|
//     //         //     //     world
//     //         //     //         .levels
//     //         //     //         .iter()
//     //         //     //         .map(|(_, level)| level.bg_rel_path.clone())
//     //         //     //         .collect::Option<String>(),
//     //         //     //     ),
//     //         //     // },
//     //         // }
//     //         // LdtkBackgroundLoader::Initialized { backgrounds } => todo!(),
//     //     }
//     // }
// }
