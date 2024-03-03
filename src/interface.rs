use bevy::prelude::*;

use crate::prelude::*;

#[allow(missing_docs)]
pub type LevelAtPositionQuery<'a, 'b> =
    Query<'a, 'b, (&'static Transform, &'static Handle<LevelAsset>)>;

#[allow(missing_docs)]
pub fn levels_at_position(
    position: Vec2,
    levels: &Assets<LevelAsset>,
    levels_query: LevelAtPositionQuery,
) -> Vec<String> {
    levels_query
        .iter()
        .map(|(_transform, handle)| {
            let level = levels.get(handle).expect("bad level handle!");
            (_transform, level)
        })
        .filter(|(transform, level)| {
            let level_position = Vec2::new(1.0, -1.0) * transform.translation.truncate();
            let level_size = level.size();
            Rect::from_corners(level_position, level_position + level_size).contains(position)
        })
        .map(|(_transform, level)| level.identifier().clone())
        .collect()
}
