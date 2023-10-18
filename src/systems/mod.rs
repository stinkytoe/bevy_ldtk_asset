pub(crate) mod load_world_event;

use bevy::prelude::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash, SystemSet)]
pub struct LdtkSet;
