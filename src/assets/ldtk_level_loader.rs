use anyhow::Result;
use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
};

use crate::ldtk_json;

use super::ldtk_level::LdtkLevel;

pub(crate) struct LdtkLevelLoader;

impl AssetLoader for LdtkLevelLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<()>> {
        Box::pin(async move {
            debug!(
                "Loading LDtk level file: {}",
                load_context.path().to_str().unwrap_or_default()
            );

            let value: ldtk_json::Level = serde_json::from_slice(bytes)?;

            // load_context.set_default_asset(LoadedAsset::new(LdtkLevel {
            //     _level: Level::new(&value, load_context),
            // }));

            load_context.set_default_asset(LoadedAsset::new(LdtkLevel { _level: value }));

            debug!(
                "Loading LDtk level file: {} success!",
                load_context.path().to_str().unwrap_or_default()
            );

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtkl"]
    }
}
