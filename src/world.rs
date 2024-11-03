use std::path::Path;
use std::str::FromStr;

use bevy_asset::{Asset, Handle, LoadContext};
use bevy_log::trace;
use bevy_math::Vec2;
use bevy_reflect::Reflect;
use bevy_tasks::block_on;

use crate::iid::{Iid, IidMap};
use crate::label::ProjectAssetPath;
use crate::ldtk;
use crate::ldtk_asset_traits::{HasChildren, LdtkAsset};
use crate::ldtk_path::ldtk_path_to_bevy_path;
use crate::level::Level;
use crate::project_loader::{ProjectContext, ProjectDefinitionContext};

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
    pub(crate) fn create_handle_pair(
        ldtk_world: &ldtk::World,
        project_asset_path: &ProjectAssetPath,
        load_context: &mut LoadContext,
        project_context: &ProjectContext,
        project_definition_context: &ProjectDefinitionContext,
    ) -> crate::Result<(Iid, Handle<Self>)> {
        let identifier = ldtk_world.identifier.clone();
        let iid = Iid::from_str(&ldtk_world.iid)?;
        let world_layout = WorldLayout::new(
            &ldtk_world.world_layout,
            ldtk_world.world_grid_width,
            ldtk_world.world_grid_height,
        )?;

        let world_asset_path = project_asset_path.to_world_asset_path(&identifier);

        // TODO: I think we can clean this up and remove some allocations while avoiding the
        // temporary binding. Possibly with Either:: or similar.
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
            .enumerate()
            .map(|(index, ldtk_level)| {
                Level::create_handle_pair(
                    ldtk_level,
                    index,
                    &world_asset_path,
                    load_context,
                    project_context,
                    project_definition_context,
                )
            })
            .collect::<crate::Result<_>>()?;

        let world = World {
            identifier,
            iid,
            world_layout,
            levels,
        }
        .into();

        let handle =
            load_context.add_loaded_labeled_asset(world_asset_path.to_asset_label(), world);

        Ok((iid, handle))
    }
}

impl LdtkAsset for World {
    fn identifier(&self) -> &str {
        &self.identifier
    }

    fn iid(&self) -> Iid {
        self.iid
    }
}

impl HasChildren for World {
    type Child = Level;

    fn children(&self) -> impl Iterator<Item = &Handle<Self::Child>> {
        self.levels.values()
    }
}
