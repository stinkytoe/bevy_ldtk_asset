use bevy::prelude::*;
use thiserror::Error;

use crate::project::defs::IntGridValue;

#[derive(Debug, Error)]
pub enum NewIntGridError {
    #[error("Bad int grid value!")]
    BadIntGridValue,
}

#[derive(Component, Debug, Reflect)]
pub struct IntGrid {
    pub values: Vec<Option<IntGridValue>>,
}

impl IntGrid {
    pub(crate) fn new(
        int_grid_csv: &[i64],
        int_grid_values: &[IntGridValue],
    ) -> Result<Self, NewIntGridError> {
        Ok(Self {
            values: int_grid_csv
                .iter()
                .map(|value| {
                    if *value == 0 {
                        Ok(None)
                    } else {
                        Ok(Some(
                            int_grid_values
                                .iter()
                                .find(|int_grid_value| int_grid_value.value == *value)
                                .ok_or(NewIntGridError::BadIntGridValue)?
                                .clone(),
                        ))
                    }
                })
                .collect::<Result<Vec<_>, NewIntGridError>>()?,
        })
    }
}
