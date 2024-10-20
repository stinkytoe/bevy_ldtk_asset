#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    IidError(#[from] crate::iid::IidError),

    #[error(transparent)]
    ReadAssetBytesError(#[from] bevy::asset::ReadAssetBytesError),

    #[error("Failure importing ldtk file! {0}")]
    LdtkImportError(String),
}

pub type Result<T> = core::result::Result<T, Error>;
