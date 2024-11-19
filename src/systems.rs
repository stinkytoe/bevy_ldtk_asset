#[cfg(feature = "asset_events_debug")]
pub(crate) mod asset_events_debug {
    use bevy_asset::AssetEvent;
    use bevy_ecs::event::EventReader;
    use bevy_log::debug;

    use crate::ldtk_asset_trait::LdtkAsset;

    pub(crate) fn asset_events_debug_output<Asset: LdtkAsset>(
        mut asset_events: EventReader<AssetEvent<Asset>>,
    ) {
        asset_events
            .read()
            .for_each(|event| debug!("AssetEvent: {event:?}"));
    }
}
