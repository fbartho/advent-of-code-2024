// Include deps that will be available in every puzzle

pub use crate::Puzzle;
pub use anyhow::Error;
pub use itertools::Itertools;

pub use crate::grid_from_vec_vec;
#[allow(unused_imports)]
pub use crate::FBGrid;
pub use crate::GridCoord2;
pub use crate::TravelDirection;

#[allow(unused_imports)]
pub use std::str::FromStr;
#[allow(unused_imports)]
pub use std::string::ToString;

#[allow(unused_imports)]
pub use strum::IntoEnumIterator;
pub use strum_macros::{Display, EnumIter, EnumString};
