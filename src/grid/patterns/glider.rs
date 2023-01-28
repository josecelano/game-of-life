use std::str::FromStr;

use crate::grid::Grid;

pub fn glider() -> Grid {
    Grid::from_str(
        "⬛⬜⬛
         ⬛⬛⬜
         ⬜⬜⬜",
    )
    .unwrap()
}
