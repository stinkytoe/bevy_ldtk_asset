use bevy_math::Vec2;
use bevy_sprite::Anchor;

use crate::ldtk_import_error;

pub(crate) fn bevy_anchor_from_ldtk(pivot: &[f64]) -> crate::Result<Anchor> {
    (pivot.len() == 2)
        .then(|| match (pivot[0] as f32, pivot[1] as f32) {
            (0.0, 0.0) => Anchor::TOP_LEFT,
            (0.0, 0.5) => Anchor::CENTER_LEFT,
            (0.0, 1.0) => Anchor::BOTTOM_LEFT,
            (0.5, 0.0) => Anchor::TOP_CENTER,
            (0.5, 0.5) => Anchor::CENTER,
            (0.5, 1.0) => Anchor::BOTTOM_CENTER,
            (1.0, 0.0) => Anchor::TOP_RIGHT,
            (1.0, 0.5) => Anchor::CENTER_RIGHT,
            (1.0, 1.0) => Anchor::BOTTOM_RIGHT,
            (x, y) => Anchor::from(Vec2::new(x - 0.5, 0.5 - y)),
        })
        .ok_or(ldtk_import_error!(
            "Unable to parse pivot input to bevy Anchor! given: {pivot:?}",
        ))
}
