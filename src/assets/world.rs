use bevy::prelude::*;

use crate::{
    ldtk,
    prelude::{LevelBundle, LoadParameters},
    traits::{HasIdentifier, SpawnsEntities},
};

use super::{level::LevelAsset, project::ProjectAsset};

/// An asset representing a world in an ldtk project
#[derive(Asset, Clone, Debug, TypePath)]
pub struct WorldAsset {
    identifier: String,
    project_handle: Handle<ProjectAsset>,
    world_grid_size: Option<(i64, i64)>,
    world_layout: ldtk::WorldLayout,
    level_identifiers: Vec<String>,
}

impl WorldAsset {
    pub(crate) fn new_from_ldtk_json(
        value: &ldtk::LdtkJson,
        project_handle: Handle<ProjectAsset>,
    ) -> Self {
        Self {
            identifier: "World".to_string(),
            project_handle,
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

    pub(crate) fn new_from_ldtk_world(
        value: &ldtk::World,
        project_handle: Handle<ProjectAsset>,
    ) -> Self {
        Self {
            identifier: value.identifier.clone(),
            project_handle,
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

    /// Returns a handle to the project which defines this world
    pub fn project_handle(&self) -> &Handle<ProjectAsset> {
        &self.project_handle
    }
}

impl WorldAsset {
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

impl HasIdentifier for WorldAsset {
    fn identifier(&self) -> &String {
        &self.identifier
    }
}

impl SpawnsEntities for WorldAsset {
    fn spawn_entities(
        &self,
        commands: &mut Commands,
        entity: Entity,
        projects: &Assets<ProjectAsset>,
        _worlds: &Assets<WorldAsset>,
        _levels: &Assets<LevelAsset>,
    ) {
        let project = projects
            .get(self.project_handle.clone_weak())
            .expect("No project for this world?");

        commands
            .entity(entity)
            .insert(Name::from(self.identifier().as_str()));

        self.level_identifiers
            .iter()
            .filter_map(|identifier| project.levels().get(identifier))
            .for_each(|level_asset| {
                let level_entity = commands
                    .spawn(LevelBundle {
                        level: level_asset.clone(),
                        load_parameters: LoadParameters::Everything,
                        ..default()
                    })
                    .id();
                commands.entity(entity).add_child(level_entity);
            });
    }
}
