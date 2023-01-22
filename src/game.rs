use crate::cell::coordinates::Coordinates;
use std::{thread, time::Duration};

use crate::{
    grid::functions::{next_generation::next_generation, overlap::overlap},
    grid::patters::glider::glider,
    grid::Grid,
};

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn play(generations: i64, generation_duration: Duration) -> String {
    // TODO: add argument closure console_printer and remove the print
    // in order to test this function for each iteration if we want.

    let back_grid = Grid::of_dead_cells(30, 60);

    let pattern = glider();

    let mut grid = overlap(&back_grid, &pattern, &Coordinates::new(13, 29));

    let mut output = "".to_string();

    for _iter in 0..generations {
        clear_screen();
        output = grid.to_string();
        print!("{}", &output);
        grid = next_generation(&grid);
        thread::sleep(generation_duration);
    }

    output
}
