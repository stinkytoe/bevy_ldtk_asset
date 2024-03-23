use bevy::prelude::*;

use crate::prelude::LevelAsset;

/// A system parameter for querying which levels, if any,
/// contain a given location in world space.
pub type LevelsAtLocation<'world, 'state> = (
    Res<'world, Assets<LevelAsset>>,
    Query<'world, 'state, (&'static GlobalTransform, &'static Handle<LevelAsset>)>,
);

#[allow(missing_docs)]
pub trait LevelsAtLocationTrait {
    /// Search all of the loaded levels for any which contain the given location
    fn find(&self, location: Vec2) -> Vec<Handle<LevelAsset>>;
}

impl LevelsAtLocationTrait for LevelsAtLocation<'_, '_> {
    fn find(&self, location: Vec2) -> Vec<Handle<LevelAsset>> {
        let (levels, levels_query) = self;
        levels_query
            .iter()
            .filter(|(transform, handle)| {
                let level = levels.get(*handle).expect("bad level handle!");
                let level_position = transform.translation().truncate();
                let level_size = Vec2::new(1.0, -1.0) * level.size();
                Rect::from_corners(level_position, level_position + level_size).contains(location)
            })
            .map(|(_transform, handle)| handle.clone()) //level.identifier().to_string())
            .collect()
    }
}
