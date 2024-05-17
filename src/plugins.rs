use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use crate::entity::EntityPlugin;
use crate::layer::LayerPlugin;
use crate::level::LevelPlugin;
use crate::project::ProjectPlugin;
use crate::tileset_rectangle::TilesetRectanglePlugin;
use crate::world::WorldPlugin;

#[derive(Debug, Default)]
pub struct LdtkLevelsPlugins;

impl PluginGroup for LdtkLevelsPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(EntityPlugin)
            .add(LayerPlugin)
            .add(LevelPlugin)
            .add(ProjectPlugin)
            .add(TilesetRectanglePlugin)
            .add(WorldPlugin)
    }
}
