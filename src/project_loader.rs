use bevy_asset::io::Reader;
use bevy_asset::{AssetLoader, LoadContext};

use crate::project::Project;
use crate::result::LdtkResult;

#[derive(Default)]
pub(crate) struct ProjectLoader;

impl AssetLoader for ProjectLoader {
    type Asset = Project;
    type Settings = ();
    type Error = crate::error::LdtkError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> LdtkResult<Self::Asset> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let ldtk_project = serde_json::from_slice(&bytes)?;

        let project = Project::new(ldtk_project, load_context).await?;

        Ok(project)
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
