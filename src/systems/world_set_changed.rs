use bevy::prelude::*;

use crate::{
    components::WorldSet,
    prelude::{LdtkProject, LdtkRoot},
};

pub(crate) fn world_set_changed(
    ldtk_root: Query<(Entity, &LdtkRoot, &WorldSet), Changed<WorldSet>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    image_assets: Res<Assets<Image>>,
) {
    for (_entity, root, world_set) in ldtk_root.iter() {
        debug!("world set changed! got: {world_set:?}");
        match world_set {
            WorldSet::All => (),
            WorldSet::Only(_) => (),
        };

        // match ldtk_project_assets.get(&root.project) {
        //     Some(project) => debug!("got a project! {:#?}", project.json()),
        //     None => debug!("No project, still loading?"),
        // };

        let Some(project) = ldtk_project_assets.get(&root.project) else {
            debug!("No project, still loading?");
            return;
        };

        for level_background in project.level_backgrounds.values() {
            match image_assets.get(level_background) {
                Some(image) => debug!("background image loaded! {:#?}", image),
                None => debug!("No image, still loading?"),
            };
        }
    }
}
