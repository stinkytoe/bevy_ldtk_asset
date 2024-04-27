use bevy::prelude::*;

use crate::layer::LayerComponent;
use crate::project::ProjectAsset;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub enum LoadEntities {
    None,
    ByIdentifiers(Vec<String>),
    ByIids(Vec<String>),
    ByTags(Vec<String>),
    #[default]
    All,
}

#[derive(Clone, Component, Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub enum LoadEntityLayerSettings {
    ComponentOnly,
    #[default]
    Sprite,
}

#[derive(Bundle, Debug)]
pub(crate) struct EntityLayerBundle {
    pub(crate) project: Handle<ProjectAsset>,
    pub(crate) layer: LayerComponent,
    pub(crate) settings: LoadEntityLayerSettings,
    pub(crate) spatial: SpatialBundle,
}
