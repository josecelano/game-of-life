use crate::{cell_position::CellPosition, cell_row::CellRow, grid::Grid};

pub fn grid_overlap(back_grid: &Grid, front_grid: &Grid, left_top_position: &CellPosition) -> Grid {
    if back_grid.has_same_dimensions(front_grid) && left_top_position.is_left_top_corner() {
        return front_grid.clone();
    }

    // TODO: this only work when left_top_position in the left top corner

    let mut cell_rows = vec![];

    for row in 0..back_grid.rows() {
        let mut cells = vec![];
        for column in 0..back_grid.columns() {
            let cell_pos = CellPosition::new(row, column);

            if front_grid.position_is_valid(&cell_pos) {
                cells.push(front_grid.get_cell(cell_pos).clone());
            } else {
                cells.push(back_grid.get_cell(cell_pos).clone());
            }
        }
        cell_rows.push(CellRow::new(cells))
    }

    Grid::new(cell_rows)
}

#[cfg(test)]
mod tests {

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
}
