use bevy_math::Vec2;
use bevy_sprite::Anchor;

use crate::ldtk_import_error;

pub(crate) fn bevy_anchor_from_ldtk(pivot: &[f64]) -> crate::Result<Anchor> {
    (pivot.len() == 2)
        .then(|| match (pivot[0] as f32, pivot[1] as f32) {
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
        .ok_or(ldtk_import_error!(
            "Unable to parse pivot input to bevy Anchor! given: {:?}",
            pivot
        ))
}
