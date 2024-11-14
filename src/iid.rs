use std::fmt::Debug;
use std::num::ParseIntError;
use thiserror::Error;

use bevy_utils::HashMap;
use bevy_utils::HashSet;

#[derive(Debug, Error)]
pub enum IidError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),

    #[error("Format error on input: {0}")]
    FormatError(String),
}

pub use uuid::Uuid as Iid;

#[macro_export]
macro_rules! iid {
    ($iid:expr) => {
        uuid!(iid)
    };
}
pub use iid;

pub type IidMap<T> = HashMap<Iid, T>;
pub type IidSet = HashSet<Iid>;

mod test {
    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_from_str() {
        use super::Iid;
        use std::str::FromStr;

        let x = Iid::from_str("e41ad760-25d0-11ef-bd94-e3313b674128").unwrap();
        assert_eq!(x.as_u128(), 0xe41ad76025d011efbd94e3313b674128);

        let x = Iid::from_str("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz");
        assert!(x.is_err());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_to_string() {
        use super::Iid;
        use std::str::FromStr;

        let x = Iid::from_str("e41ad760-25d0-11ef-bd94-e3313b674128").unwrap();
        assert_eq!(x.to_string(), "e41ad760-25d0-11ef-bd94-e3313b674128");
    }
}
