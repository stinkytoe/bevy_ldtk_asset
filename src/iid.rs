use bevy::reflect::Reflect;
use bevy::utils::HashMap;
use bevy::utils::HashSet;
use std::fmt::Debug;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IidError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("Format error on input: {0}")]
    FormatError(String),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect)]
pub struct Iid {
    value: u128,
}

impl Iid {
    pub fn as_u128(&self) -> u128 {
        self.value
    }
}

impl From<Iid> for u128 {
    fn from(value: Iid) -> Self {
        value.as_u128()
    }
}

impl FromStr for Iid {
    type Err = IidError;

    // TODO: We can do better validation here,
    // and possibly optimization
    fn from_str(s: &str) -> Result<Self, IidError> {
        if s.len() != 36 {
            return Err(IidError::FormatError(s.to_owned()));
        }

        let s: String = s.split('-').flat_map(|s| s.chars()).collect();

        Ok(Self {
            value: u128::from_str_radix(&s, 16)?,
        })
    }
}

impl TryInto<Iid> for &str {
    type Error = IidError;

    fn try_into(self) -> Result<Iid, Self::Error> {
        Iid::from_str(self)
    }
}

impl Display for Iid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // e41ad760-25d0-11ef-bd94-e3313b674128
        // e41ad76025d011efbd94e3313b674128
        let s = format!("{:032x}", self.value);
        let s = format!(
            "{}-{}-{}-{}-{}",
            &s[0..8],
            &s[8..12],
            &s[12..16],
            &s[16..20],
            &s[20..32]
        );
        write!(f, "{}", s)
    }
}

impl Debug for Iid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

#[allow(unused)]
pub type IidMap<T> = HashMap<Iid, T>;

#[allow(unused)]
pub type IidSet = HashSet<Iid>;

mod test {
    #[test]
    fn test_from_str() {
        use super::Iid;
        use std::str::FromStr;

        let x = Iid::from_str("e41ad760-25d0-11ef-bd94-e3313b674128").unwrap();
        assert_eq!(x.as_u128(), 0xe41ad76025d011efbd94e3313b674128);

        let x = Iid::from_str("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz");
        assert!(x.is_err());
    }

    #[test]
    fn test_to_string() {
        use super::Iid;
        use std::str::FromStr;

        let x = Iid::from_str("e41ad760-25d0-11ef-bd94-e3313b674128").unwrap();
        assert_eq!(x.to_string(), "e41ad760-25d0-11ef-bd94-e3313b674128");
    }
}
