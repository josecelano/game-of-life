use crate::{cell::Cell, cell_coordinates::CellCoordinates, cell_row::CellRow, grid::Grid};

/// It overlaps a grid on top of another grid at a given cell position,
/// returning a new grid. It uses the left top corner of the front grid
/// for the overlapping position.
///
/// For example:
///
/// Back Grid:   Front Grid:
/// ```text
///  0 1 2 3 4     0 1 2
/// 0⬛⬛⬛⬛⬛   ⬜⬜⬜
/// 1⬛⬛⬛⬛⬛   ⬜⬜⬜
/// 2⬛⬛⬛⬛⬛   ⬜⬜⬜
/// 3⬛⬛⬛⬛⬛
/// 4⬛⬛⬛⬛⬛
/// ```
///
/// Overlapped at position (0,0):
///
/// ```text
///   0 1 2 3 4
/// 0⬜⬜⬜⬛⬛
/// 1⬜⬜⬜⬛⬛
/// 2⬜⬜⬜⬛⬛
/// 3⬛⬛⬛⬛⬛
/// 4⬛⬛⬛⬛⬛
/// ```
///
/// Overlapped at position (2,2):
///
/// ```text
///   0 1 2 3 4
/// 0⬛⬛⬛⬛⬛
/// 1⬛⬛⬛⬛⬛
/// 2⬛⬛⬜⬜⬜
/// 3⬛⬛⬜⬜⬜
/// 4⬛⬛⬜⬜⬜
/// ```
pub fn overlap(back_grid: &Grid, front_grid: &Grid, front_grid_position: &CellCoordinates) -> Grid {
    if back_grid.is_empty() {
        return Grid::new_empty();
    }

    if front_grid.is_empty() {
        return back_grid.clone();
    }

    if perfect_overlapping(back_grid, front_grid, front_grid_position) {
        return front_grid.clone();
    }

    if !back_grid.position_is_valid(front_grid_position) {
        panic!("Position out of back grid dimensions");
    }

    if !back_grid.position_is_valid(&front_grid_right_bottom_corner_coordinates(
        front_grid,
        front_grid_position,
    )) {
        panic!("Front grid does not fit in back grid at the given position");
    }

    Grid::new(calculate_new_rows(
        back_grid,
        front_grid,
        front_grid_position,
    ))
}

fn perfect_overlapping(
    back_grid: &Grid,
    front_grid: &Grid,
    front_grid_position: &CellCoordinates,
) -> bool {
    back_grid.has_same_dimensions(front_grid) && front_grid_position.is_left_top_corner()
}

/// Right bottom corner coordinates for the front grid using back grid coordinates origin
fn front_grid_right_bottom_corner_coordinates(
    front_grid: &Grid,
    front_grid_position: &CellCoordinates,
) -> CellCoordinates {
    front_grid_position.translate(front_grid.rows() - 1, front_grid.columns() - 1)
}

fn calculate_new_rows(
    back_grid: &Grid,
    front_grid: &Grid,
    front_grid_position: &CellCoordinates,
) -> Vec<CellRow> {
    let mut cell_rows = vec![];

    for row in 0..back_grid.rows() {
        cell_rows.push(calculate_new_row(
            row,
            back_grid,
            front_grid,
            front_grid_position,
        ))
    }

    cell_rows
}

fn calculate_new_row(
    row: usize,
    back_grid: &Grid,
    front_grid: &Grid,
    front_grid_position: &CellCoordinates,
) -> CellRow {
    let mut cells = vec![];
    for column in 0..back_grid.columns() {
        cells.push(calculate_new_cell(
            CellCoordinates::new(row, column),
            back_grid,
            front_grid,
            front_grid_position,
        ));
    }
    CellRow::with(cells)
}

fn calculate_new_cell(
    cell_coordinates: CellCoordinates,
    back_grid: &Grid,
    front_grid: &Grid,
    front_grid_position: &CellCoordinates,
) -> Cell {
    match overlapped_cell(&cell_coordinates, front_grid, front_grid_position) {
        None => back_grid.get_cell(cell_coordinates).clone(),
        Some(front_cell_pos) => front_grid.get_cell(front_cell_pos).clone(),
    }
}

fn overlapped_cell(
    cell_coordinates: &CellCoordinates,
    front_grid: &Grid,
    front_grid_position: &CellCoordinates,
) -> Option<CellCoordinates> {
    if !is_overlapped_cell(cell_coordinates, front_grid, front_grid_position) {
        return None;
    }

    Some(relative_front_grid_cell_coordinates(
        cell_coordinates,
        front_grid_position,
    ))
}

/// Return true is at the current back grid cell coordinates
/// there is also a cell in the front grid
fn is_overlapped_cell(
    cell_coordinates: &CellCoordinates,
    front_grid: &Grid,
    front_grid_position: &CellCoordinates,
) -> bool {
    cell_coordinates.row >= front_grid_position.row
        && cell_coordinates.column >= front_grid_position.column
        && cell_coordinates.row < (front_grid_position.row + front_grid.rows())
        && cell_coordinates.column < front_grid_position.column + front_grid.columns()
}

fn relative_front_grid_cell_coordinates(
    cell_coordinates: &CellCoordinates,
    front_grid_position: &CellCoordinates,
) -> CellCoordinates {
    cell_coordinates.recalculate_to_origin(front_grid_position)
}

#[cfg(test)]
mod tests {
    use crate::{cell_coordinates::CellCoordinates, grid::Grid, grid_functions::overlap::overlap};

    #[test]
    #[should_panic]
    fn it_should_fail_when_the_overlapping_position_is_not_a_position_inside_the_back_grid() {
        let back_grid = Grid::of_dead_cells(5, 5);
        let front_grid = Grid::of_live_cells(5, 5);

        // Row out of range
        overlap(&back_grid, &front_grid, &CellCoordinates::new(6, 2));

        // Column out of range
        overlap(&back_grid, &front_grid, &CellCoordinates::new(5, 6));
    }

    #[test]
    #[should_panic]
    fn it_should_fail_when_the_front_grid_does_not_fit_inside_the_back_grid_because_of_row_dimension(
    ) {
        let back_grid = Grid::of_dead_cells(2, 2);
        let front_grid = Grid::of_dead_cells(2, 2);

        // Second row of front grid does not fit
        overlap(&back_grid, &front_grid, &CellCoordinates::new(1, 0));
    }

    #[test]
    #[should_panic]
    fn it_should_fail_when_the_front_grid_does_not_fit_inside_the_back_grid_because_of_column_dimension(
    ) {
        let back_grid = Grid::of_dead_cells(2, 2);
        let front_grid = Grid::of_dead_cells(2, 2);

        // Second columns of the front gird does not fit
        overlap(&back_grid, &front_grid, &CellCoordinates::new(0, 1));
    }

    mod overlapping_on_the_left_top_corner {
        use crate::{
            cell::Cell, cell_coordinates::CellCoordinates, cell_row::CellRow, grid::Grid,
            grid_functions::overlap::overlap,
        };

        fn left_top_corner() -> CellCoordinates {
            CellCoordinates::new(0, 0)
        }

        #[test]
        fn two_default_grids_returns_the_default_grid() {
            let back_grid = Grid::default();
            let front_grid = Grid::default();

            assert_eq!(
                overlap(&back_grid, &front_grid, &left_top_corner()),
                Grid::default()
            );
        }

        #[test]
        fn two_empty_grids_returns_an_empty_grid() {
            let back_grid = Grid::new_empty();
            let front_grid = Grid::default();

            assert_eq!(
                overlap(&back_grid, &front_grid, &left_top_corner()),
                Grid::new_empty()
            );
        }

        #[test]
        fn two_identical_grids_returns_the_same_grid() {
            let back_grid = Grid::of_dead_cells(1, 1);
            let front_grid = Grid::of_dead_cells(1, 1);

            assert_eq!(
                overlap(&back_grid, &front_grid, &left_top_corner()),
                Grid::of_dead_cells(1, 1)
            );
        }

        #[test]
        fn smaller_grid_into_a_bigger_one() {
            let back_grid = Grid::new(vec![
                CellRow::new(vec![Cell::dead(), Cell::dead(), Cell::dead()]),
                CellRow::new(vec![Cell::dead(), Cell::dead(), Cell::dead()]),
                CellRow::new(vec![Cell::dead(), Cell::dead(), Cell::dead()]),
            ]);
            let front_grid = Grid::new(vec![
                CellRow::new(vec![Cell::live(), Cell::live()]),
                CellRow::new(vec![Cell::live(), Cell::live()]),
            ]);

            assert_eq!(
                overlap(&back_grid, &front_grid, &left_top_corner()),
                Grid::new(vec![
                    CellRow::new(vec![Cell::live(), Cell::live(), Cell::dead()]),
                    CellRow::new(vec![Cell::live(), Cell::live(), Cell::dead()]),
                    CellRow::new(vec![Cell::dead(), Cell::dead(), Cell::dead()]),
                ])
            );
        }
    }

    mod overlapping_not_on_the_left_top_corner {
        use crate::{
            cell::c, cell_coordinates::CellCoordinates, cell_row::CellRow, grid::Grid,
            grid_functions::overlap::overlap,
        };

        #[test]
        fn smaller_front_grid_that_fits_into_the_back_grid() {
            let back_grid = Grid::of_dead_cells(5, 5);
            let front_grid = Grid::of_live_cells(3, 3);

            assert_eq!(
                overlap(&back_grid, &front_grid, &CellCoordinates::new(2, 2)),
                Grid::new(vec![
                    //                   0        1        2        3         4
                    CellRow::new(vec![c('⬛'), c('⬛'), c('⬛'), c('⬛'), c('⬛'),]), // 0
                    CellRow::new(vec![c('⬛'), c('⬛'), c('⬛'), c('⬛'), c('⬛'),]), // 1
                    CellRow::new(vec![c('⬛'), c('⬛'), c('⬜'), c('⬜'), c('⬜'),]), // 2
                    CellRow::new(vec![c('⬛'), c('⬛'), c('⬜'), c('⬜'), c('⬜'),]), // 3
                    CellRow::new(vec![c('⬛'), c('⬛'), c('⬜'), c('⬜'), c('⬜'),]), // 4
                ])
            );
        }
    }
}
