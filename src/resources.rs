use crate::prelude::LdtkLevel;
use bevy::{prelude::*, utils::HashSet};

#[derive(Debug, Default, Resource)]
pub(crate) struct LdtkLevels {
	pub(crate) to_load: HashSet<(Entity, Handle<LdtkLevel>)>,
	pub(crate) loaded: HashSet<(Entity, Handle<LdtkLevel>)>,
	// pub(crate) to_unload: HashSet<Entity>,
}
