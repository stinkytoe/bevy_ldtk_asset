use bevy::math::Vec2;
use bevy::sprite::Anchor;

use crate::error::Error;

pub(crate) fn bevy_anchor_from_ldtk(pivot: &[f64]) -> Result<Anchor, Error> {
    if pivot.len() != 2 {
        return Err(Error::LdtkImportError(format!(
            "Unable to parse pivot input to bevy Anchor! given: {:?}",
            pivot
        )));
    }

    Ok(match (pivot[0] as f32, pivot[1] as f32) {
        (0.0, 0.0) => Anchor::TopLeft,
        (0.0, 0.5) => Anchor::CenterLeft,
        (0.0, 1.0) => Anchor::BottomLeft,
        (0.5, 0.0) => Anchor::TopCenter,
        (0.5, 0.5) => Anchor::Center,
        (0.5, 1.0) => Anchor::BottomCenter,
        (1.0, 0.0) => Anchor::TopRight,
        (1.0, 0.5) => Anchor::CenterRight,
        (1.0, 1.0) => Anchor::BottomRight,
        (x, y) => Anchor::Custom(Vec2::new(x - 0.5, 0.5 - y)),
    })
}
