use crate::output::Printer;
use crate::{cell::coordinates::Coordinates, grid::size::Size};
use std::{thread, time::Duration};

use crate::{
    grid::functions::{next_generation::next_generation, overlap::overlap},
    grid::Grid,
};

#[must_use]
pub fn play<T: Printer>(
    generations: i64,
    generation_duration: Duration,
    back_grid_size: &Size,
    pattern: &Grid,
    console: &T,
) -> String {
    let back_grid = Grid::of_dead_cells(back_grid_size.rows, back_grid_size.columns);

    // todo: put the pattern in the center of the background grid
    let mut grid = overlap(&back_grid, pattern, &Coordinates::new(13, 29));

    let mut output = String::new();

    for _iter in 0..generations {
        console.clear();

        output = grid.to_string();

        console.print(&output);

        grid = next_generation(&grid);

        thread::sleep(generation_duration);
    }

    output
}
