pub(crate) mod world_set_changed;

use bevy::prelude::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash, SystemSet)]
pub struct LdtkSet;
