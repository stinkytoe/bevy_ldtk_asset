use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use thiserror::Error;

use crate::entity::EntityComponent;
use crate::field_instance::FieldInstanceValueAsError;
use crate::tileset_rectangle::TilesetRectangle;

#[derive(Debug, Error)]
pub enum EntityComponentTilesetError {
    #[error("No field instance with given identidier!")]
    BadIdentifier,
    #[error("The field instance with given identifier exists, but is not a Tile!")]
    NotATile(#[from] FieldInstanceValueAsError),
}

#[derive(SystemParam)]
pub struct EntityComponentTileset<'w, 's> {
    commands: Commands<'w, 's>,
    query_with_rect: Query<
        'w,
        's,
        (&'static EntityComponent, &'static mut TilesetRectangle),
        With<TilesetRectangle>,
    >,
    query_without_rect: Query<'w, 's, &'static EntityComponent, Without<TilesetRectangle>>,
}

impl EntityComponentTileset<'_, '_> {
    pub fn set_tileset_rectangle_to_field_instance(
        &mut self,
        entity: Entity,
        identifier: &str,
    ) -> Result<(), EntityComponentTilesetError> {
        let tile_stub = |entity_component: &EntityComponent,
                         identifier: &str|
         -> Result<TilesetRectangle, EntityComponentTilesetError> {
            Ok(entity_component
                .get_field_instance_by_identifier(identifier)
                .ok_or(EntityComponentTilesetError::BadIdentifier)?
                .as_tile()?
                .clone())
        };

        if let Ok((entity_component, mut tileset_rectangle)) = self.query_with_rect.get_mut(entity)
        {
            let tile = tile_stub(entity_component, identifier)?;

            *tileset_rectangle = tile.clone();
        } else if let Ok(entity_component) = self.query_without_rect.get(entity) {
            let tile = tile_stub(entity_component, identifier)?;

            self.commands.entity(entity).insert(tile.clone());
        }

        Ok(())
    }
}
