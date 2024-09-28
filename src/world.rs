use std::str::FromStr;

use bevy::asset::Asset;
use bevy::math::Vec2;
use bevy::reflect::Reflect;

use crate::error::Error;
use crate::iid::Iid;
use crate::ldtk;
use crate::ldtk_asset_trait::LdtkAsset;

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
    ) -> Result<Self, Error> {
        match layout {
            Some(ldtk::WorldLayout::GridVania) => Ok(Self::GridVania(
                (world_grid_width as f32, world_grid_height as f32).into(),
            )),
            Some(ldtk::WorldLayout::Free) => Ok(Self::Free),
            Some(ldtk::WorldLayout::LinearHorizontal) => Ok(Self::LinearHorizontal),
            Some(ldtk::WorldLayout::LinearVertical) => Ok(Self::LinearVertical),
            None => todo!(),
        }
    }
}

#[derive(Asset, Debug, Reflect)]
pub struct World {
    pub identifier: String,
    pub iid: Iid,
    pub world_layout: WorldLayout,
}

impl World {
    pub(crate) fn new(value: &ldtk::World) -> Result<Self, Error> {
        let identifier = value.identifier.clone();
        let iid = Iid::from_str(&value.iid)?;
        let world_layout = WorldLayout::new(
            &value.world_layout,
            value.world_grid_width,
            value.world_grid_height,
        )?;

        Ok(World {
            identifier,
            iid,
            world_layout,
        })
    }
}

impl LdtkAsset for World {
    fn iid(&self) -> Iid {
        self.iid
    }
}
