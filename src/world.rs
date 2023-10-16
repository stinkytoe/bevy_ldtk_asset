use crate::{ldtk_json, level::Level};
use bevy::{asset::LoadContext, prelude::*};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct World {
    pub identifier: String,
    pub iid: String,
    pub levels: HashMap<String, Level>,
    pub world_grid_height: i64,
    pub world_grid_width: i64,
    pub world_layout: ldtk_json::WorldLayout,
}

impl World {
    pub fn new_from_ldtk_json(value: &ldtk_json::LdtkJson, load_context: &LoadContext) -> Self {
        World {
            identifier: { "(root)".to_owned() },
            iid: value.iid.clone(),
            levels: value
                .levels
                .iter()
                .map(|value| {
                    // let new_level = Level::from(value);
                    let new_level = Level::new(value, load_context);
                    debug!("Loaded level: {}", new_level.identifier);
                    debug!("    with iid: {}", new_level.iid);
                    (new_level.iid.clone(), new_level)
                })
                .collect(),
            world_grid_height: value.world_grid_height.unwrap_or_else(|| {
                debug!("Got None for worldGridHeight? Is this a multiworld? Using 256");
                256
            }),
            world_grid_width: value.world_grid_width.unwrap_or_else(|| {
                debug!("Got None for worldGridWidth? Is this a multiworld? Using 256");
                256
            }),
            world_layout: value
                .world_layout
                .as_ref()
                .unwrap_or_else(|| {
                    debug!("Got None for worldLayout? Is this a multiworld? Using 'Free'");
                    &ldtk_json::WorldLayout::Free
                })
                .clone(),
        }
    }

    pub fn new_from_ldtk_world(value: &ldtk_json::World, _load_context: &LoadContext) -> Self {
        World {
            identifier: value.identifier.clone(),
            iid: value.iid.clone(),
            levels: HashMap::default(),
            world_grid_height: value.world_grid_height,
            world_grid_width: value.world_grid_width,
            world_layout: value.world_layout.as_ref().unwrap_or_else(|| {
                debug!("Got None for worldLayout? Weird, if this is a multuworld this should be something. Using 'Free'");
                &ldtk_json::WorldLayout::Free}).clone(),
        }
    }
}
