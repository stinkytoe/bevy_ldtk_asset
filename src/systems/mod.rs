pub(crate) mod update_loaded_ldtk_project;

use bevy::prelude::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash, SystemSet)]
pub struct LdtkSet;
