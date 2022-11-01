use crate::{cell::c, cell_row::CellRow, grid::Grid, grid_printer::render_grid};

pub fn play() -> String {
    let grid = Grid::new(vec![
        CellRow::new(vec![c('⬜'), c('⬜'), c('⬜')]),
        CellRow::new(vec![c('⬜'), c('⬜'), c('⬜')]),
        CellRow::new(vec![c('⬜'), c('⬜'), c('⬜')]),
    ]);

    render_grid(&grid)
}
