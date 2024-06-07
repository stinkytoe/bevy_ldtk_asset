use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::entity::EntityAsset;

pub type LdtkEntitiesQueryData<'a> = (Entity, &'a Handle<EntityAsset>);

#[derive(SystemParam)]
pub struct LdtkEntities<'w, 's> {
    ldtk_entity_query: Query<'w, 's, LdtkEntitiesQueryData<'static>>,
    ldtk_entity_added_query:
        Query<'w, 's, LdtkEntitiesQueryData<'static>, Added<Handle<EntityAsset>>>,
    entity_assets: Res<'w, Assets<EntityAsset>>,
}

impl<'w, 's> LdtkEntities<'w, 's> {
    pub fn iter(&'w self) -> impl Iterator<Item = LdtkEntitiesItem> {
        self.ldtk_entity_query
            .iter()
            .map(|(entity, handle)| LdtkEntitiesItem {
                entity,
                asset: self.entity_assets.get(handle).expect("bad handle?"),
            })
    }

    pub fn iter_added(&'w self) -> impl Iterator<Item = LdtkEntitiesItem> {
        self.ldtk_entity_added_query
            .iter()
            .map(|(entity, handle)| LdtkEntitiesItem {
                entity,
                asset: self.entity_assets.get(handle).expect("bad handle?"),
            })
    }
}

#[derive(Debug)]
pub struct LdtkEntitiesItem<'w> {
    pub entity: Entity,
    pub asset: &'w EntityAsset,
}

#[derive(Debug)]
pub struct LdtkEntitiesWithTag<'w, I>
where
    I: Iterator<Item = LdtkEntitiesItem<'w>>,
{
    iter: I,
    tag: &'w str,
}

impl<'w, I> Iterator for LdtkEntitiesWithTag<'w, I>
where
    I: Iterator<Item = LdtkEntitiesItem<'w>>,
{
    type Item = LdtkEntitiesItem<'w>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|item| {
            item.asset
                .tags
                .iter()
                .any(|inner_tag| inner_tag == self.tag)
        })
    }
}

pub trait LdtkEntitiesEx<'w>: Iterator<Item = LdtkEntitiesItem<'w>> + Sized {
    fn with_tag(self, tag: &'w str) -> LdtkEntitiesWithTag<'w, Self> {
        LdtkEntitiesWithTag { iter: self, tag }
    }

    fn by_identifier(mut self, identifier: &'w str) -> Option<LdtkEntitiesItem> {
        self.find(|item| item.asset.identifier == identifier)
    }

    fn by_iid(mut self, iid: &'w str) -> Option<LdtkEntitiesItem> {
        self.find(|item| item.asset.iid == iid)
    }
}

impl<'w, I: Iterator<Item = LdtkEntitiesItem<'w>>> LdtkEntitiesEx<'w> for I {}
