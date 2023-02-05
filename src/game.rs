use crate::cell::coordinates::Coordinates;
use crate::output::Printer;
use crate::settings::Settings;
use std::thread;

use crate::{
    grid::functions::{next_generation::next_generation, overlap::overlap},
    grid::Grid,
};

#[must_use]
pub fn play<T: Printer>(settings: &Settings, pattern: &Grid, console: &T) -> String {
    let back_grid = Grid::of_dead_cells(
        settings.back_grid_size.rows,
        settings.back_grid_size.columns,
    );

    // todo: put the pattern in the center of the background grid
    let mut grid = overlap(&back_grid, pattern, &Coordinates::new(13, 29));

    let mut output = String::new();

    for _iter in 0..settings.generations {
        console.clear();

        output = grid.to_string();

        console.print(&output);

        grid = next_generation(&grid);

        thread::sleep(settings.generation_duration);
    }

    output
}
