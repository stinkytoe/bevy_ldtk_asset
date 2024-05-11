use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::level::LevelComponent;

#[derive(SystemParam)]
pub struct LevelComponentQuery<'w, 's> {
    // commands: Commands<'w, 's>,
    levels_query: Query<'w, 's, (Entity, &'static LevelComponent)>,
    levels_transform_query: Query<'w, 's, &'static Transform>,
}

impl<'w> LevelComponentQuery<'w, '_> {
    pub fn with_identifier(&self, identifier: &str) -> Option<(Entity, &LevelComponent)> {
        self.levels_query
            .iter()
            .find(|(_, level_component)| level_component.identifier() == identifier)
    }

    pub fn with_iid(&self, iid: &str) -> Option<(Entity, &LevelComponent)> {
        self.levels_query
            .iter()
            .find(|(_, level_component)| level_component.iid() == iid)
    }

    pub fn levels_at_world_location(
        &'w self,
        location: Vec2,
    ) -> impl Iterator<Item = (Entity, &LevelComponent)> + 'w {
        self.levels_query
            .iter()
            .filter_map(|(entity, level_component)| {
                self.levels_transform_query
                    .get(entity)
                    .ok()
                    .map(|transform| (entity, level_component, transform))
            })
            .filter(move |(_, level_component, transform)| {
                let top_right = transform.translation.truncate();
                let bottom_left = top_right + Vec2::new(1.0, -1.0) * level_component.size();
                Rect::from_corners(top_right, bottom_left).contains(location)
            })
            .map(|(entity, level_component, _)| (entity, level_component))
    }
}
