use std::path::Path;
use std::str::FromStr;

use bevy::asset::{Asset, Handle, LoadContext};
use bevy::log::trace;
use bevy::math::Vec2;
use bevy::reflect::Reflect;
use bevy::tasks::block_on;

use crate::iid::{Iid, IidMap};
use crate::ldtk;
use crate::ldtk_asset_trait::LdtkAsset;
use crate::ldtk_path::ldtk_path_to_bevy_path;
use crate::level::Level;
use crate::project_loader::ProjectContext;

#[derive(Debug, Reflect)]
pub enum WorldLayout {
    Free,
    GridVania(Vec2),
    LinearHorizontal,
    LinearVertical,
}

impl WorldLayout {
    fn new(
        layout: &Option<ldtk::WorldLayout>,
        world_grid_width: i64,
        world_grid_height: i64,
    ) -> crate::Result<Self> {
        match layout {
            Some(ldtk::WorldLayout::GridVania) => Ok(Self::GridVania(
                (world_grid_width as f32, world_grid_height as f32).into(),
            )),
            Some(ldtk::WorldLayout::Free) => Ok(Self::Free),
            Some(ldtk::WorldLayout::LinearHorizontal) => Ok(Self::LinearHorizontal),
            Some(ldtk::WorldLayout::LinearVertical) => Ok(Self::LinearVertical),
            // TODO: WTF is up with this?
            None => todo!(),
        }
    }
}

#[derive(Asset, Debug, Reflect)]
pub struct World {
    pub identifier: String,
    pub iid: Iid,
    pub world_layout: WorldLayout,
    pub levels: IidMap<Handle<Level>>,
}

impl World {
    pub(crate) fn new(
        ldtk_world: &ldtk::World,
        load_context: &mut LoadContext,
        project_context: &ProjectContext,
    ) -> crate::Result<Self> {
        let identifier = ldtk_world.identifier.clone();
        let iid = Iid::from_str(&ldtk_world.iid)?;
        let world_layout = WorldLayout::new(
            &ldtk_world.world_layout,
            ldtk_world.world_grid_width,
            ldtk_world.world_grid_height,
        )?;

        let levels_iter = if project_context.external_levels {
            &ldtk_world
                .levels
                .iter()
                .map(|ldtk_level| -> crate::Result<ldtk::Level> {
                    let external_rel_path = ldtk_level.external_rel_path.as_ref().ok_or(
                        crate::Error::LdtkImportError(
                            "external_rel_path is None when external_levels is true!".to_string(),
                        ),
                    )?;

                    trace!("Attempting to load external level from path: {external_rel_path}");

                    let ldtk_path = Path::new(external_rel_path);
                    let bevy_path =
                        ldtk_path_to_bevy_path(project_context.project_directory, ldtk_path);
                    let bytes = block_on(async { load_context.read_asset_bytes(bevy_path).await })?;
                    let level: ldtk::Level = serde_json::from_slice(&bytes)?;
                    Ok(level)
                })
                .collect::<crate::Result<_>>()?
        } else {
            &ldtk_world.levels
        };

        let levels = levels_iter
            .iter()
            .map(|ldtk_level| {
                //let world_iid = Iid::from_str(&ldtk_world.iid)?;
                //let world_label = ldtk_world.identifier.clone();
                //let world = World::new(
                //    ldtk_world,
                //    load_context,
                //    &project_directory,
                //    ldtk_project.external_levels,
                //)?
                let level_iid = Iid::from_str(&ldtk_level.iid)?;
                let level_label = format!("{}/{}", ldtk_world.identifier, ldtk_level.identifier);
                let level = Level::new(ldtk_level, load_context, project_context)?.into();
                let handle = load_context.add_loaded_labeled_asset(level_label, level);
                Ok((level_iid, handle))
            })
            .collect::<crate::Result<_>>()?;

        //let levels = IidMap::default();

        Ok(World {
            identifier,
            iid,
            world_layout,
            levels,
        })
    }
}

impl LdtkAsset for World {
    fn iid(&self) -> Iid {
        self.iid
    }

    fn identifier(&self) -> &str {
        &self.identifier
    }

    //fn parent_path(&self) -> bevy::asset::AssetPath {
    //    AssetPath::from(&self.parent_path)
    //}
    //
    //fn children_paths(&self) -> impl Iterator<Item = bevy::asset::AssetPath> {
    //    self.children_paths.iter().map(AssetPath::from)
    //}
}
