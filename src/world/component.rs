use bevy::math::I64Vec2;
use bevy::prelude::*;

use crate::ldtk;
use crate::world::WorldLayout;

#[derive(Component, Debug)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct WorldComponent {
    identifier: String,
    iid: String,
    // NOTE: how are we handling levels?
    world_grid_size: I64Vec2,
    // #[reflect(ignore)]
    world_layout: Option<WorldLayout>,
}

impl WorldComponent {
    pub fn identifier(&self) -> &str {
        self.identifier.as_ref()
    }

    pub fn iid(&self) -> &str {
        self.iid.as_ref()
    }

    pub fn world_grid_size(&self) -> I64Vec2 {
        self.world_grid_size
    }

    pub fn world_layout(&self) -> &Option<WorldLayout> {
        &self.world_layout
    }
}

impl From<&ldtk::World> for WorldComponent {
    fn from(value: &ldtk::World) -> Self {
        Self {
            identifier: value.identifier.clone(),
            iid: value.iid.clone(),
            world_grid_size: (value.world_grid_width, value.world_grid_height).into(),
            world_layout: value.world_layout.clone(),
        }
    }
}
