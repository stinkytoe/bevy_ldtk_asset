use bevy::prelude::*;

use crate::ldtk;

pub use ldtk::WorldLayout;

#[derive(Asset, Clone, Debug, TypePath)]
pub struct WorldAsset {
    identifier: String,
    world_grid_size: Option<(i64, i64)>,
    world_layout: WorldLayout,
}

impl From<ldtk::LdtkJson> for WorldAsset {
    fn from(value: ldtk::LdtkJson) -> Self {
        Self {
            identifier: "World".to_string(),
            world_grid_size: if matches!(value.world_layout, Some(WorldLayout::GridVania)) {
                Some((
                    value
                        .world_grid_width
                        .expect("world_grid_width is 'None' in a GridVania layout?"),
                    value
                        .world_grid_height
                        .expect("world_grid_height is 'None' in a GridVania layout?"),
                ))
            } else {
                None
            },
            world_layout: value
                .world_layout
                .expect("World layout is 'None' in a single world context?"),
        }
    }
}

impl From<ldtk::World> for WorldAsset {
    fn from(value: ldtk::World) -> Self {
        Self {
            identifier: value.identifier.clone(),
            world_grid_size: if matches!(value.world_layout, Some(WorldLayout::GridVania)) {
                Some((value.world_grid_width, value.world_grid_height))
            } else {
                None
            },
            world_layout: value
                .world_layout
                .expect("World layout is 'None' in a multi world context?"),
        }
    }
}

impl WorldAsset {
    pub fn identifier(&self) -> &String {
        &self.identifier
    }

    pub fn get_world_layout(&self) -> &WorldLayout {
        &self.world_layout
    }

    pub fn get_world_grid_size(&self) -> Option<(i64, i64)> {
        self.world_grid_size
    }
}
