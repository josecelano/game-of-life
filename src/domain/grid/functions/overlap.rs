use crate::domain::cell::row::Row;
use crate::domain::cell::Cell;
use crate::domain::{cell::coordinates::Coordinates, grid::Grid};

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
///
/// # Panics
///
/// Will panic if the front grid does not fit totally inside the back grid at the given position.
#[must_use]
pub fn overlap(back_grid: &Grid, front_grid: &Grid, front_grid_position: &Coordinates) -> Grid {
    if back_grid.is_empty() {
        return Grid::new_empty();
    }

    if front_grid.is_empty() {
        return back_grid.clone();
    }

    if perfect_overlapping(back_grid, front_grid, front_grid_position) {
        return front_grid.clone();
    }

    assert!(
        back_grid.position_is_valid(front_grid_position),
        "Position for front grid is out of back grid dimensions"
    );

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
    front_grid_position: &Coordinates,
) -> bool {
    back_grid.has_same_dimensions(front_grid) && front_grid_position.is_left_top_corner()
}

/// Right bottom corner coordinates for the front grid using back grid coordinates origin
fn front_grid_right_bottom_corner_coordinates(
    front_grid: &Grid,
    front_grid_position: &Coordinates,
) -> Coordinates {
    front_grid_position.translate(front_grid.rows() - 1, front_grid.columns() - 1)
}

fn calculate_new_rows(
    back_grid: &Grid,
    front_grid: &Grid,
    front_grid_position: &Coordinates,
) -> Vec<Row> {
    let mut cell_rows = vec![];

    for row in 0..back_grid.rows() {
        cell_rows.push(calculate_new_row(
            row,
            back_grid,
            front_grid,
            front_grid_position,
        ));
    }

    cell_rows
}

fn calculate_new_row(
    row: usize,
    back_grid: &Grid,
    front_grid: &Grid,
    front_grid_position: &Coordinates,
) -> Row {
    let mut cells = vec![];
    for column in 0..back_grid.columns() {
        cells.push(calculate_new_cell(
            Coordinates::new(row, column),
            back_grid,
            front_grid,
            front_grid_position,
        ));
    }
    Row::with(cells)
}

fn calculate_new_cell(
    cell_coordinates: Coordinates,
    back_grid: &Grid,
    front_grid: &Grid,
    front_grid_position: &Coordinates,
) -> Cell {
    match overlapped_cell(&cell_coordinates, front_grid, front_grid_position) {
        None => back_grid.get_cell(&cell_coordinates).clone(),
        Some(front_cell_pos) => front_grid.get_cell(&front_cell_pos).clone(),
    }
}

fn overlapped_cell(
    cell_coordinates: &Coordinates,
    front_grid: &Grid,
    front_grid_position: &Coordinates,
) -> Option<Coordinates> {
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
    cell_coordinates: &Coordinates,
    front_grid: &Grid,
    front_grid_position: &Coordinates,
) -> bool {
    cell_coordinates.row >= front_grid_position.row
        && cell_coordinates.column >= front_grid_position.column
        && cell_coordinates.row < (front_grid_position.row + front_grid.rows())
        && cell_coordinates.column < front_grid_position.column + front_grid.columns()
}

fn relative_front_grid_cell_coordinates(
    cell_coordinates: &Coordinates,
    front_grid_position: &Coordinates,
) -> Coordinates {
    cell_coordinates.recalculate_to_origin(front_grid_position)
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::cell::coordinates::Coordinates, domain::grid::functions::overlap::overlap,
        domain::grid::Grid,
    };

    #[test]
    #[should_panic]
    fn it_should_fail_when_the_overlapping_position_is_not_a_position_inside_the_back_grid() {
        let back_grid = Grid::of_dead_cells(5, 5);
        let front_grid = Grid::of_live_cells(5, 5);

        // Row out of range
        let _ = overlap(&back_grid, &front_grid, &Coordinates::new(6, 2));

        // Column out of range
        let _ = overlap(&back_grid, &front_grid, &Coordinates::new(5, 6));
    }

    #[test]
    #[should_panic]
    fn it_should_fail_when_the_front_grid_does_not_fit_inside_the_back_grid_because_of_row_dimension(
    ) {
        let back_grid = Grid::of_dead_cells(2, 2);
        let front_grid = Grid::of_dead_cells(2, 2);

        // Second row of front grid does not fit
        let _ = overlap(&back_grid, &front_grid, &Coordinates::new(1, 0));
    }

    #[test]
    #[should_panic]
    fn it_should_fail_when_the_front_grid_does_not_fit_inside_the_back_grid_because_of_column_dimension(
    ) {
        let back_grid = Grid::of_dead_cells(2, 2);
        let front_grid = Grid::of_dead_cells(2, 2);

        // Second columns of the front gird does not fit
        let _ = overlap(&back_grid, &front_grid, &Coordinates::new(0, 1));
    }

    mod overlapping_on_the_left_top_corner {
        use std::str::FromStr;

        use crate::{
            domain::cell::coordinates::Coordinates, domain::grid::functions::overlap::overlap,
            domain::grid::Grid,
        };

        fn left_top_corner() -> Coordinates {
            Coordinates::new(0, 0)
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
            let back_grid = Grid::from_str(
                "⬛⬛⬛
                 ⬛⬛⬛
                 ⬛⬛⬛",
            )
            .unwrap();

            let front_grid = Grid::from_str(
                "⬜⬜
                 ⬜⬜",
            )
            .unwrap();

            assert_eq!(
                overlap(&back_grid, &front_grid, &left_top_corner()),
                Grid::from_str(
                    "⬜⬜⬛
                     ⬜⬜⬛
                     ⬛⬛⬛",
                )
                .unwrap()
            );
        }
    }

    mod overlapping_not_on_the_left_top_corner {
        use std::str::FromStr;

        use crate::{
            domain::cell::coordinates::Coordinates, domain::grid::functions::overlap::overlap,
            domain::grid::Grid,
        };

        #[test]
        fn smaller_front_grid_that_fits_into_the_back_grid() {
            let back_grid = Grid::of_dead_cells(5, 5);
            let front_grid = Grid::of_live_cells(3, 3);

            assert_eq!(
                overlap(&back_grid, &front_grid, &Coordinates::new(2, 2)),
                Grid::from_str(
                    "⬛⬛⬛⬛⬛
                     ⬛⬛⬛⬛⬛
                     ⬛⬛⬜⬜⬜
                     ⬛⬛⬜⬜⬜
                     ⬛⬛⬜⬜⬜",
                )
                .unwrap()
            );
        }
    }
}
