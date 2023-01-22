use crate::cell::row::Row;
use crate::{cell::c, grid::Grid};

pub fn glider() -> Grid {
    Grid::new(vec![
        Row::new(vec![c('⬛'), c('⬜'), c('⬛')]),
        Row::new(vec![c('⬛'), c('⬛'), c('⬜')]),
        Row::new(vec![c('⬜'), c('⬜'), c('⬜')]),
    ])
}
