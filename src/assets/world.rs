use bevy::prelude::*;

use crate::{
    ldtk,
    traits::{HasIdentifier, SpawnsEntities},
};

/// An asset representing a world in an ldtk project
#[derive(Asset, Clone, Debug, TypePath)]
pub struct WorldAsset {
    pub(crate) identifier: String,
    world_grid_size: Option<(i64, i64)>,
    world_layout: ldtk::WorldLayout,
    level_identifiers: Vec<String>,
}

impl HasIdentifier for WorldAsset {
    fn identifier(&self) -> &String {
        &self.identifier
    }
}

impl SpawnsEntities for WorldAsset {
    fn spawn_entities(&self, commands: &mut Commands, entity: Entity) {
        commands
            .entity(entity)
            .insert((Name::from(self.identifier().as_str()),));
    }
}

impl WorldAsset {
    pub(crate) fn new_from_ldtk_json(value: &ldtk::LdtkJson) -> Self {
        Self {
            identifier: "World".to_string(),
            world_grid_size: if matches!(value.world_layout, Some(ldtk::WorldLayout::GridVania)) {
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
                .as_ref()
                .expect("World layout is 'None' in a single world context?")
                .clone(),
            level_identifiers: value
                .levels
                .iter()
                .map(|level| &level.identifier)
                .cloned()
                .collect(),
        }
    }
}

// impl From<&ldtk::LdtkJson> for WorldAsset {
//     fn from(value: &ldtk::LdtkJson) -> Self {
//         Self {
//             identifier: "World".to_string(),
//             world_grid_size: if matches!(value.world_layout, Some(ldtk::WorldLayout::GridVania)) {
//                 Some((
//                     value
//                         .world_grid_width
//                         .expect("world_grid_width is 'None' in a GridVania layout?"),
//                     value
//                         .world_grid_height
//                         .expect("world_grid_height is 'None' in a GridVania layout?"),
//                 ))
//             } else {
//                 None
//             },
//             world_layout: value
//                 .world_layout
//                 .as_ref()
//                 .expect("World layout is 'None' in a single world context?")
//                 .clone(),
//             level_identifiers: value
//                 .levels
//                 .iter()
//                 .map(|level| &level.identifier)
//                 .cloned()
//                 .collect(),
//         }
//     }
// }

impl From<&ldtk::World> for WorldAsset {
    fn from(value: &ldtk::World) -> Self {
        Self {
            identifier: value.identifier.clone(),
            world_grid_size: if matches!(value.world_layout, Some(ldtk::WorldLayout::GridVania)) {
                Some((value.world_grid_width, value.world_grid_height))
            } else {
                None
            },
            world_layout: value
                .world_layout
                .as_ref()
                .expect("World layout is 'None' in a multi world context?")
                .clone(),
            level_identifiers: value
                .levels
                .iter()
                .map(|level| &level.identifier)
                .cloned()
                .collect(),
        }
    }
}

impl WorldAsset {
    /// Returns the identifier of the world
    // pub fn identifier(&self) -> &String {
    //     &self.identifier
    // }

    /// The world layout as defined in the project
    pub fn get_world_layout(&self) -> &ldtk::WorldLayout {
        &self.world_layout
    }

    /// For GridVania layout, returns an option containing a tuple pair, representing
    /// the width and height of the grid size.
    ///
    /// For other layouts, returns None
    pub fn get_world_grid_size(&self) -> Option<(i64, i64)> {
        self.world_grid_size
    }

    /// Returns an iterator of all levels which belong to this world
    pub fn get_level_identifiers(&self) -> impl Iterator<Item = &String> {
        self.level_identifiers.iter()
    }

    /// Returns true of this world has a level with the given identifier,
    /// and false otherwise
    pub fn has_level_identifier(&self, identifier: String) -> bool {
        self.level_identifiers.contains(&identifier)
    }
}
