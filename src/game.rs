use std::{thread, time::Duration};

use crate::{
    cell_coordinates::CellCoordinates,
    grid::Grid,
    grid_functions::{next_generation::next_generation, overlap::overlap},
    grid_printer::render_grid,
    patters::glider::glider,
};

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn play(generations: i64, generation_duration: Duration) -> String {
    // TODO: add argument closure console_printer and remove the print
    // in order to test this function for each iteration if we want.

    let back_grid = Grid::of_dead_cells(30, 60);

    let pattern = glider();

    let mut grid = overlap(&back_grid, &pattern, &CellCoordinates::new(13, 29));

    let mut output = "".to_string();

    for _iter in 0..generations {
        clear_screen();
        output = render_grid(&grid);
        print!("{}", &output);
        grid = next_generation(&grid);
        thread::sleep(generation_duration);
    }

    output
}
