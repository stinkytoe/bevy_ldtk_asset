#![allow(missing_docs)]

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    UuidError(#[from] uuid::Error),

    #[error(transparent)]
    ReadAssetBytesError(#[from] bevy_asset::ReadAssetBytesError),

    #[error("Failure importing ldtk file! {0}")]
    LdtkImportError(String),

    #[error("Duplicate Iid error! {0}")]
    DuplicateIidError(crate::iid::Iid),
}

#[macro_export]
macro_rules! ldtk_import_error {
    ($($args:tt)*) => {
        $crate::error::Error::LdtkImportError(format!($($args)*))
    };
}
