use crate::{
    cell_coordinates::CellCoordinates, grid::Grid, grid_functions::overlap::overlap,
    grid_printer::render_grid, patters::glider::glider,
};

pub fn play() -> String {
    let grid = Grid::of_dead_cells(30, 60);
    let pattern = glider();
    render_grid(&overlap(&grid, &pattern, &CellCoordinates::new(13, 29)))
}
