#[cfg(feature = "asset_events_debug")]
pub(crate) mod asset_events_debug {
    use bevy_asset::AssetEvent;
    use bevy_ecs::event::EventReader;
    use bevy_log::debug;

    use crate::ldtk_asset_traits::LdtkAsset;
    use crate::project::Project;

    pub(crate) fn project_asset_events_debug_output(
        mut project_events: EventReader<AssetEvent<Project>>,
    ) {
        project_events
            .read()
            .for_each(|event| debug!("AssetEvent: {event:?}"));
    }

    pub(crate) fn ldtk_asset_events_debug_output<A: LdtkAsset>(
        mut events: EventReader<AssetEvent<A>>,
    ) {
        events
            .read()
            .for_each(|event| debug!("AssetEvent: {event:?}"));
    }
}
