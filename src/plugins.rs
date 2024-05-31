use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use crate::common_components::CommonComponentsPlugin;
use crate::entity::EntityPlugin;
use crate::layer::LayerPlugin;
use crate::level::LevelPlugin;
use crate::project::defs::TilesetRectanglePlugin;
use crate::project::ProjectPlugin;
use crate::world::WorldPlugin;

#[derive(Debug, Default)]
pub struct LdtkLevelsPlugins;

impl PluginGroup for LdtkLevelsPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CommonComponentsPlugin)
            .add(EntityPlugin)
            .add(LayerPlugin)
            .add(LevelPlugin)
            .add(ProjectPlugin)
            .add(TilesetRectanglePlugin)
            .add(WorldPlugin)
    }
}
