use std::str::FromStr;

use crate::grid::Grid;

/// # Panics
/// 
/// Will panic if it provider string grid patter cannot be parsed.
#[must_use]
pub fn glider() -> Grid {
    Grid::from_str(
        "⬛⬜⬛
         ⬛⬛⬜
         ⬜⬜⬜",
    )
    .unwrap()
}
