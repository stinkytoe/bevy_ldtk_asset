use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use thiserror::Error;

use crate::entity::EntityComponent;
use crate::field_instance::FieldInstanceValueAsTileError;
use crate::tileset_rectangle::TilesetRectangle;

#[derive(Debug, Error)]
pub enum EntityComponentTilesetError {
    #[error("No field instance with given identidier!")]
    BadIdentifier,
    #[error("The field instance with given identifier exists, but is not a Tile!")]
    FieldInstanceValueAsTileError(#[from] FieldInstanceValueAsTileError),
}

#[derive(SystemParam)]
pub struct EntityComponentTileset<'w, 's> {
    commands: Commands<'w, 's>,
    query: Query<'w, 's, &'static EntityComponent>,
}

impl EntityComponentTileset<'_, '_> {
    pub fn set_tileset_rectangle_to_field_instance(
        &mut self,
        entity: Entity,
        identifier: &str,
    ) -> Result<(), EntityComponentTilesetError> {
        if let Ok(entity_component) = self.query.get(entity) {
            let tile = entity_component
                .get_field_instance_by_identifier(identifier)
                .ok_or(EntityComponentTilesetError::BadIdentifier)?
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
