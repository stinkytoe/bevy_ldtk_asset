use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::entity::EntityAsset;

#[derive(SystemParam)]
pub struct LdtkEntities<'w, 's> {
    ldtk_entity_query: Query<'w, 's, (Entity, &'static Handle<EntityAsset>)>,
    ldtk_entity_added_query:
        Query<'w, 's, (Entity, &'static Handle<EntityAsset>), Added<Handle<EntityAsset>>>,
    entity_assets: Res<'w, Assets<EntityAsset>>,
}

impl<'w, 's> LdtkEntities<'w, 's> {
    pub fn iter(
        &'w self,
    ) -> LdtkEntitiesIterator<'w, 's, impl Iterator<Item = (Entity, &'w Handle<EntityAsset>)>> {
        LdtkEntitiesIterator {
            inner: self.ldtk_entity_query.iter(),
            ldtk_entities: self,
        }
    }

    pub fn iter_added(
        &'w self,
    ) -> LdtkEntitiesIterator<'w, 's, impl Iterator<Item = (Entity, &'w Handle<EntityAsset>)>> {
        LdtkEntitiesIterator {
            inner: self.ldtk_entity_added_query.iter(),
            ldtk_entities: self,
        }
    }
}

pub struct LdtkEntitiesIterator<'w, 's, I>
where
    I: Iterator<Item = (Entity, &'w Handle<EntityAsset>)> + Sized,
{
    inner: I,
    ldtk_entities: &'w LdtkEntities<'w, 's>,
}

impl<'w, 's, I> Iterator for LdtkEntitiesIterator<'w, 's, I>
where
    I: Iterator<Item = (Entity, &'w Handle<EntityAsset>)>,
{
    type Item = (Entity, &'w Handle<EntityAsset>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'w, 's, I> LdtkEntitiesIterator<'w, 's, I>
where
    I: Iterator<Item = (Entity, &'w Handle<EntityAsset>)> + 'w,
{
    pub fn new(inner: I, ldtk_entities: &'s LdtkEntities) -> Self {
        Self {
            inner,
            ldtk_entities,
        }
    }

    #[inline]
    pub fn with_identifier(mut self, identifier: &str) -> Option<Entity> {
        self.inner
            .find(|(_, handle)| {
                self.ldtk_entities
                    .entity_assets
                    .get(*handle)
                    .expect("bad handle?")
                    .identifier
                    == identifier
            })
            .map(|(entity, _)| entity)
    }

    #[inline]
    pub fn with_iid(mut self, iid: &str) -> Option<Entity> {
        self.inner
            .find(|(_, handle)| {
                self.ldtk_entities
                    .entity_assets
                    .get(*handle)
                    .expect("bad handle?")
                    .iid
                    == iid
            })
            .map(|(entity, _)| entity)
    }

    #[inline]
    pub fn has_tag(self, tag: &'w str) -> impl Iterator<Item = Entity> + 'w {
        self.inner
            .filter(move |(_, handle)| {
                self.ldtk_entities
                    .entity_assets
                    .get(*handle)
                    .expect("bad handle?")
                    .tags
                    .iter()
                    .any(|inner_tag| inner_tag == tag)
            })
            .map(|(entity, _)| entity)
    }
}
