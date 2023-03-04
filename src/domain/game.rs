use super::{
    cell::coordinates::Coordinates,
    grid::{
        functions::{next_generation::next_generation, overlap::overlap},
        Grid,
    },
    output::printer::Printer,
    settings::Settings,
    timer::Timer,
};

#[must_use]
pub fn play<P: Printer, T: Timer>(
    settings: &Settings,
    pattern: &Grid,
    console: &P,
    timer: &T,
) -> String {
    let back_grid = Grid::of_dead_cells(
        settings.back_grid_size.rows,
        settings.back_grid_size.columns,
    );

    // todo: put the pattern in the center of the background grid
    let mut grid = overlap(&back_grid, pattern, &Coordinates::new(13, 29));

    let mut output = String::new();

    for _iter in 0..settings.generations {
        output = grid.to_string();

        console.clear();
        console.print(&output);

        timer.wait(settings.generation_lifetime);

        grid = next_generation(&grid);
    }

    output
}
