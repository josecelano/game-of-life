use crate::cell_row::CellRow;
use crate::{cell::c, grid::Grid};

pub fn glider() -> Grid {
    Grid::new(vec![
        CellRow::new(vec![c('⬛'), c('⬜'), c('⬛')]),
        CellRow::new(vec![c('⬛'), c('⬛'), c('⬜')]),
        CellRow::new(vec![c('⬜'), c('⬜'), c('⬜')]),
    ])
}
