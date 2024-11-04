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
    ReadAssetBytesError(#[from] bevy_asset::ReadAssetBytesError),

    #[error("Failure importing ldtk file! {0}")]
    LdtkImportError(String),
}

#[macro_export]
macro_rules! ldtk_import_error {
    ($($args:tt)*) => {
        $crate::error::Error::LdtkImportError(format!($($args)*))
    };
}

pub type Result<T> = core::result::Result<T, Error>;
