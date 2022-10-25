use grid::Grid;
use grid_printer::print_grid;

pub mod cell;
pub mod grid;
pub mod grid_printer;

fn main() {
    let grid = Grid::new(1, 1);

    print_grid(&grid);
}
