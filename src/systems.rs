#[cfg(feature = "asset_messages_debug")]
pub(crate) mod asset_messages_debug {
    use bevy_asset::AssetEvent;
    use bevy_ecs::message::MessageReader;
    use bevy_log::debug;

    use crate::ldtk_asset_trait::LdtkAsset;

    pub(crate) fn asset_messages_debug_output<Asset: LdtkAsset>(
        mut asset_events: MessageReader<AssetEvent<Asset>>,
    ) {
        asset_events
            .read()
            .for_each(|event| debug!("AssetEvent: {event:?}"));
    }
}
