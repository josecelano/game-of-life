use crate::{grid_printer::render_grid, patters::glider::glider};

pub fn play() -> String {
    let grid = glider();
    render_grid(&grid)
}
