// Include deps that will be available in every puzzle

pub use crate::Puzzle;
pub use anyhow::Error;
pub use itertools::Itertools;

#[allow(unused_imports)]
pub use crate::{grid_from_vec_vec, FBGrid, GridCoord2, GridDistance2, TravelDirection};

#[allow(unused_imports)]
pub use std::str::FromStr;
#[allow(unused_imports)]
pub use std::string::ToString;

#[allow(unused_imports)]
pub use strum::IntoEnumIterator;
pub use strum_macros::{Display, EnumIter, EnumString};
