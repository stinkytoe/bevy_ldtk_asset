use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::entity::EntityComponent;
use crate::prelude::FieldInstance;
use crate::tileset_rectangle::TilesetRectangle;

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

    pub fn added_with_tag(
        &'w self,
        tag: &'static str,
    ) -> impl Iterator<Item = (Entity, &EntityComponent)> + 'w {
        self.added_ec_query
            .iter()
            .filter(|(_, entity_component)| entity_component.has_tag(tag))
    }

    pub fn get_field_instance(&self, entity: Entity, identifier: &str) -> Option<&FieldInstance> {
        self.with_ec_query
            .get(entity)
            .ok()
            .and_then(|(_, entity_component)| entity_component.get_field_instance(identifier))
    }

    pub fn set_tile(&mut self, entity: Entity, tile: TilesetRectangle) {
        self.commands
            .entity(entity)
            .remove::<TilesetRectangle>()
            .insert(tile);
    }
}
