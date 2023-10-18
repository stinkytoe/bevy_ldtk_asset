use crate::{ldtk_json, level::Level};
use bevy::{asset::LoadContext, prelude::*};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub(crate) struct World {
    pub(crate) _identifier: String,
    pub(crate) _iid: String,
    pub(crate) _levels: HashMap<String, Level>,
    pub(crate) _world_grid_height: i64,
    pub(crate) _world_grid_width: i64,
    pub(crate) _world_layout: ldtk_json::WorldLayout,
}

impl World {
    pub(crate) fn new_from_ldtk_json(
        value: &ldtk_json::LdtkJson,
        load_context: &LoadContext,
    ) -> Self {
        debug!("Loading world data from project root.");
        debug!("Since we're constructing from the old style, one world representation,");
        debug!("we'll use (root) as the identifier since one isn't supplied.");
        debug!("Loading world: (root)");
        debug!("     with iid: {}", value.iid);
        World {
            _identifier: "(root)".to_owned(),
            _iid: value.iid.clone(),
            _levels: value
                .levels
                .iter()
                .map(|value| {
                    // let new_level = Level::from(value);
                    let new_level = Level::new(value, load_context);
                    (new_level.iid.clone(), new_level)
                })
                .collect(),
            _world_grid_height: value.world_grid_height.unwrap_or_else(|| {
                debug!("Got None for worldGridHeight? Is this a multiworld? Using 256");
                256
            }),
            _world_grid_width: value.world_grid_width.unwrap_or_else(|| {
                debug!("Got None for worldGridWidth? Is this a multiworld? Using 256");
                256
            }),
            _world_layout: value
                .world_layout
                .as_ref()
                .unwrap_or_else(|| {
                    debug!("Got None for worldLayout? Is this a multiworld? Using 'Free'");
                    &ldtk_json::WorldLayout::Free
                })
                .clone(),
        }
    }

    pub(crate) fn new_from_ldtk_world(
        value: &ldtk_json::World,
        _load_context: &LoadContext,
    ) -> Self {
        debug!("Loading world: {}", value.identifier);
        debug!("     with iid: {}", value.iid);
        World {
            _identifier: value.identifier.clone(),
            _iid: value.iid.clone(),
            _levels: HashMap::default(),
            _world_grid_height: value.world_grid_height,
            _world_grid_width: value.world_grid_width,
            _world_layout: value.world_layout.as_ref().unwrap_or_else(|| {
                debug!("Got None for worldLayout? Weird, if this is a multuworld this should be something. Using 'Free'");
                &ldtk_json::WorldLayout::Free}).clone(),
        }
    }
}
