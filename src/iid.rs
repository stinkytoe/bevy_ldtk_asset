//! Interface for handling Iid fields in LDtk
//!
//! This is a thin wrapper over the [uuid](https://crates.io/crates/uuid) crate, with
//! [Iid] being a re-export of [uuid::Uuid], and the [iid]! macro being a re-export of of
//! [uuid::Uuid].
//!
//! I chose to wrap these values for tow reasons:
//! * To match the nomenclature of LDtk
//! * I had originally written my own implementation, but switched to the superior [uuid] crate
//!   after I realized it offered better features while remaining compatable with LDtk.

#![allow(missing_docs)]

use bevy_utils::HashMap;
use bevy_utils::HashSet;

pub use uuid::Uuid as Iid;

pub use uuid::uuid;

#[macro_export]
macro_rules! iid {
    ($iid:expr) => {
        $crate::uuid!($iid)
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
