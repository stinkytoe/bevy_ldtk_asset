use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use thiserror::Error;

use crate::entity::EntityComponent;
use crate::field_instance::FieldInstanceValueAsTileError;
use crate::tileset_rectangle::TilesetRectangle;

#[derive(Debug, Error)]
pub enum EntityComponentQueryError {
    #[error("No field instance with given identidier!")]
    BadIdentifier,
    #[error("The field instance with given identifier exists, but is not a Tile!")]
    FieldInstanceValueAsTileError(#[from] FieldInstanceValueAsTileError),
}

#[derive(SystemParam)]
pub struct EntityComponentQuery<'w, 's> {
    commands: Commands<'w, 's>,
    added_ec_query: Query<'w, 's, (Entity, &'static EntityComponent), Added<EntityComponent>>,
    with_ec_query: Query<'w, 's, (Entity, &'static EntityComponent)>,
}

impl<'w> EntityComponentQuery<'w, '_> {
    pub fn with_tag(
        &'w self,
        tag: &'static str,
    ) -> impl Iterator<Item = (Entity, &EntityComponent)> + 'w {
        self.with_ec_query
            .iter()
            .filter(|(_, entity_component)| entity_component.has_tag(tag))
    }

    pub fn new_with_tag(
        &'w self,
        tag: &'static str,
    ) -> impl Iterator<Item = (Entity, &EntityComponent)> + 'w {
        self.added_ec_query
            .iter()
            .filter(|(_, entity_component)| entity_component.has_tag(tag))
    }

    pub fn set_tile_to_field_instance(
        &mut self,
        entity: Entity,
        identifier: &str,
    ) -> Result<(), EntityComponentQueryError> {
        if let Ok((_, entity_component)) = self.with_ec_query.get(entity) {
            let tile = entity_component
                .get_field_instance_by_identifier(identifier)
                .ok_or(EntityComponentQueryError::BadIdentifier)?
                .as_tile()?
                .clone();

            self.commands
                .entity(entity)
                .remove::<TilesetRectangle>()
                .insert(tile);
        };

        Ok(())
    }
}
