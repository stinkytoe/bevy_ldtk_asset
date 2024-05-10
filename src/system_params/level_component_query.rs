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
    pub fn levels_at_world_location(
        &'w self,
        location: Vec2,
    ) -> impl Iterator<Item = (Entity, &LevelComponent)> + 'w {
        self.levels_query
            .iter()
            .filter_map(
                |(entity, level_component)| match self.levels_transform_query.get(entity) {
                    Ok(transform) => Some((entity, level_component, transform)),
                    Err(_) => None,
                },
            )
            .filter_map(move |(entity, level_component, transform)| {
                let top_right = transform.translation.truncate();
                let bottom_left = top_right + Vec2::new(1.0, -1.0) * level_component.size();

                if Rect::from_corners(top_right, bottom_left).contains(location) {
                    Some((entity, level_component))
                } else {
                    None
                }
            })
    }
}
