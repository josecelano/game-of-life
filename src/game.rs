use crate::cell::coordinates::Coordinates;
use crate::output::Printer;
use std::{thread, time::Duration};

use crate::{
    grid::functions::{next_generation::next_generation, overlap::overlap},
    grid::patterns::glider::glider,
    grid::Grid,
};

#[must_use]
pub fn play<T: Printer>(generations: i64, generation_duration: Duration, console: &T) -> String {
    let back_grid = Grid::of_dead_cells(30, 60);

    let pattern = glider();

    let mut grid = overlap(&back_grid, &pattern, &Coordinates::new(13, 29));

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
