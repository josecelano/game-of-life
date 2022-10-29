use cell::c;
use cell_row::CellRow;
use grid::Grid;
use grid_printer::print_grid;

pub mod cell;
pub mod cell_row;
pub mod grid;
pub mod grid_printer;

fn main() {
    let grid = Grid::new(vec![
        CellRow::new(vec![c('⬜'), c('⬜'), c('⬜')]),
        CellRow::new(vec![c('⬜'), c('⬜'), c('⬜')]),
        CellRow::new(vec![c('⬜'), c('⬜'), c('⬜')]),
    ]);

    print_grid(&grid);
}
