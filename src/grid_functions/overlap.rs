use crate::{cell::Cell, cell_position::CellPosition, cell_row::CellRow, grid::Grid};

/// It overlaps a grid on top of another grid at a given cell position,
/// returning a new grid. It uses the left top corner of the front grid
/// for the overlapping position.
///
/// For example:
///
/// Back Grid:   Front Grid:
///  0 1 2 3 4     0 1 2
/// 0⬛⬛⬛⬛⬛   ⬜⬜⬜
/// 1⬛⬛⬛⬛⬛   ⬜⬜⬜
/// 2⬛⬛⬛⬛⬛   ⬜⬜⬜
/// 3⬛⬛⬛⬛⬛
/// 4⬛⬛⬛⬛⬛
///
/// Overlapped at position (0,0):
///
/// Result:
///   0 1 2 3 4
/// 0⬜⬜⬜⬛⬛
/// 1⬜⬜⬜⬛⬛
/// 2⬜⬜⬜⬛⬛
/// 3⬛⬛⬛⬛⬛
/// 4⬛⬛⬛⬛⬛
///
/// Overlapped at position (2,2):
///
/// Result:
///   0 1 2 3 4
/// 0⬛⬛⬛⬛⬛
/// 1⬛⬛⬛⬛⬛
/// 2⬛⬛⬜⬜⬜
/// 3⬛⬛⬜⬜⬜
/// 4⬛⬛⬜⬜⬜
pub fn grid_overlap(back_grid: &Grid, front_grid: &Grid, position: &CellPosition) -> Grid {
    if back_grid.is_empty() {
        return Grid::new_empty();
    }

    if front_grid.is_empty() {
        return back_grid.clone();
    }

    if !back_grid.position_is_valid(position) {
        // TODO: we have other options:
        // - Try to overlap part of the front grid is possible.
        // - Return None (Option<Grid>)
        panic!("Position out of back grid dimensions");
    }

    if back_grid.has_same_dimensions(front_grid) && position.is_left_top_corner() {
        return front_grid.clone();
    }

    let mut cell_rows = vec![];

    for row in 0..back_grid.rows() {
        let mut cells = vec![];
        for column in 0..back_grid.columns() {
            let back_grid_cell_pos = CellPosition::new(row, column);

            // If we are in a cell overlapped by the front grid
            if row >= position.row
                && column >= position.column
                && row < (position.row + front_grid.rows())
                && column < position.column + front_grid.columns()
            {
                // Calculate the cell position relative to the front grid
                let front_grid_row = row - position.row;
                let front_grid_column = column - position.column;
                let front_grid_cell_pos = CellPosition::new(front_grid_row, front_grid_column);

                cells.push(front_grid.get_cell(front_grid_cell_pos).clone());
            } else {
                cells.push(back_grid.get_cell(back_grid_cell_pos).clone());
            }
        }
        cell_rows.push(CellRow::new(cells))
    }

    Grid::new(cell_rows)
}

#[cfg(test)]
mod tests {
    use crate::{cell_position::CellPosition, grid::Grid, grid_functions::overlap::grid_overlap};

    #[test]
    #[should_panic]
    fn it_should_fail_when_the_overlapping_position_is_not_a_position_inside_the_back_grid() {
        let back_grid = Grid::of_dead_cells(5, 5);
        let front_grid = Grid::of_live_cells(5, 5);

        // Row out of range
        grid_overlap(&back_grid, &front_grid, &CellPosition::new(6, 2));

        // Column out of range
        grid_overlap(&back_grid, &front_grid, &CellPosition::new(5, 6));
    }

    mod overlapping_on_the_left_top_corner {
        use crate::{
            cell::Cell, cell_position::CellPosition, cell_row::CellRow, grid::Grid,
            grid_functions::overlap::grid_overlap,
        };

        fn left_top_corner() -> CellPosition {
            CellPosition::new(0, 0)
        }

        #[test]
        fn two_default_grids_returns_the_default_grid() {
            let back_grid = Grid::default();
            let front_grid = Grid::default();

            assert_eq!(
                grid_overlap(&back_grid, &front_grid, &left_top_corner()),
                Grid::default()
            );
        }

        #[test]
        fn two_empty_grids_returns_an_empty_grid() {
            let back_grid = Grid::new_empty();
            let front_grid = Grid::default();

            assert_eq!(
                grid_overlap(&back_grid, &front_grid, &left_top_corner()),
                Grid::new_empty()
            );
        }

        #[test]
        fn two_identical_grids_returns_the_same_grid() {
            let back_grid = Grid::of_dead_cells(1, 1);
            let front_grid = Grid::of_dead_cells(1, 1);

            assert_eq!(
                grid_overlap(&back_grid, &front_grid, &left_top_corner()),
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
                grid_overlap(&back_grid, &front_grid, &left_top_corner()),
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
            cell::c, cell_position::CellPosition, cell_row::CellRow, grid::Grid,
            grid_functions::overlap::grid_overlap,
        };

        #[test]
        fn smaller_grid_into_a_bigger_one() {
            let back_grid = Grid::of_dead_cells(5, 5);
            let front_grid = Grid::of_live_cells(5, 5);

            assert_eq!(
                grid_overlap(&back_grid, &front_grid, &CellPosition::new(2, 2)),
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
