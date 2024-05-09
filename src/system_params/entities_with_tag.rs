use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::entity::EntityComponent;

#[derive(SystemParam)]
pub struct NewEntitiesWithTag<'w, 's> {
    query: Query<'w, 's, (Entity, &'static EntityComponent), Added<EntityComponent>>,
}

impl<'w> NewEntitiesWithTag<'w, '_> {
    pub fn with_tag(
        &'w self,
        tag: &'static str,
    ) -> impl Iterator<Item = (Entity, &EntityComponent)> + 'w {
        self.query
            .iter()
            .filter(|(_, entity_component)| entity_component.has_tag(tag))
    }
}

#[derive(SystemParam)]
pub struct AllEntitiesWithTag<'w, 's> {
    query: Query<'w, 's, (Entity, &'static EntityComponent)>,
}

impl<'w> AllEntitiesWithTag<'w, '_> {
    pub fn with_tag(
        &'w self,
        tag: &'static str,
    ) -> impl Iterator<Item = (Entity, &EntityComponent)> + 'w {
        self.query
            .iter()
            .filter(|(_, entity_component)| entity_component.has_tag(tag))
    }
}
