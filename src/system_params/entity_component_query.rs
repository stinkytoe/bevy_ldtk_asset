use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::entity::EntityComponent;
use crate::prelude::FieldInstance;
use crate::tileset_rectangle::TilesetRectangle;

#[derive(SystemParam)]
pub struct EntityComponentQuery<'w, 's> {
    commands: Commands<'w, 's>,
    added_ec_query: Query<'w, 's, (Entity, &'static EntityComponent), Added<EntityComponent>>,
    ec_query: Query<'w, 's, (Entity, &'static EntityComponent)>,
    with_tileset_rectangle: Query<'w, 's, &'static mut TilesetRectangle, With<EntityComponent>>,
}

impl<'w> EntityComponentQuery<'w, '_> {
    pub fn with_identifier(
        &'w self,
        identifier: &'static str,
    ) -> Option<(Entity, &EntityComponent)> {
        self.ec_query
            .iter()
            .find(move |(_, entity_component)| entity_component.identifier() == identifier)
    }

    pub fn just_added_with_identifier(
        &'w self,
        identifier: &'static str,
    ) -> Option<(Entity, &EntityComponent)> {
        self.added_ec_query
            .iter()
            .find(move |(_, entity_component)| entity_component.identifier() == identifier)
    }

    pub fn with_iid(&'w self, iid: &'static str) -> Option<(Entity, &EntityComponent)> {
        self.ec_query
            .iter()
            .find(move |(_, entity_component)| entity_component.iid() == iid)
    }

    pub fn just_added_with_iid(&'w self, iid: &'static str) -> Option<(Entity, &EntityComponent)> {
        self.added_ec_query
            .iter()
            .find(move |(_, entity_component)| entity_component.iid() == iid)
    }

    pub fn with_tag(
        &'w self,
        tag: &'static str,
    ) -> impl Iterator<Item = (Entity, &EntityComponent)> + 'w {
        self.ec_query
            .iter()
            .filter(|(_, entity_component)| entity_component.has_tag(tag))
    }

    pub fn just_added_with_tag(
        &'w self,
        tag: &'static str,
    ) -> impl Iterator<Item = (Entity, &EntityComponent)> + 'w {
        self.added_ec_query
            .iter()
            .filter(|(_, entity_component)| entity_component.has_tag(tag))
    }

    pub fn all(&'w self) -> impl Iterator<Item = (Entity, &EntityComponent)> + 'w {
        self.ec_query.iter()
    }

    pub fn just_added(&'w self) -> impl Iterator<Item = (Entity, &EntityComponent)> + 'w {
        self.added_ec_query.iter()
    }

    pub fn get_field_instance(&self, entity: Entity, identifier: &str) -> Option<&FieldInstance> {
        self.ec_query
            .get(entity)
            .ok()
            .and_then(|(_, entity_component)| entity_component.get_field_instance(identifier))
    }

    pub fn set_tile(&mut self, entity: Entity, tile: TilesetRectangle) {
        if let Ok(mut tileset_rectangle) = self.with_tileset_rectangle.get_mut(entity) {
            *tileset_rectangle = tile;
        } else {
            self.commands.entity(entity).insert(tile);
        }
    }
}
