use cell::Cell;
use grid::Grid;
use grid_printer::print_grid;

pub mod cell;
pub mod grid;
pub mod grid_printer;

fn main() {
    let grid = Grid::new(
        3,
        3,
        vec![
            Cell::live(),
            Cell::live(),
            Cell::live(),
            Cell::live(),
            Cell::live(),
            Cell::live(),
            Cell::live(),
            Cell::live(),
            Cell::live(),
        ],
    );

    print_grid(&grid);
}
