use bevy::prelude::*;

use crate::components::IidSet;

pub(crate) fn world_set_changed(ldtk_root: Query<(Entity, &IidSet), Changed<IidSet>>) {
    for (_, world_set) in ldtk_root.iter() {
        debug!("world set changed! got: {world_set:?}");
    }
}
