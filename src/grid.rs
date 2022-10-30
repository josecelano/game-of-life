use crate::{cell::Cell, cell_position::CellPosition, cell_row::CellRow};

pub struct Grid {
    pub cell_rows: Vec<CellRow>,
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl Grid {
    pub fn new(cell_rows: Vec<CellRow>) -> Self {
        if !cell_rows.is_empty() {
            let first_row_len = cell_rows[0].len();
            cell_rows.iter().for_each(|cell_row| {
                if cell_row.len() != first_row_len {
                    panic!("Cell rows do not have the same length");
                }
            });
        }
        Self { cell_rows }
    }

    pub fn rows(&self) -> usize {
        self.cell_rows.len()
    }

    pub fn columns(&self) -> usize {
        if self.cell_rows.is_empty() {
            return 0;
        }
        self.cell_rows[0].len()
    }

    pub fn number_of_cells(&self) -> usize {
        self.rows() * self.columns()
    }

    pub fn is_empty(&self) -> bool {
        self.number_of_cells() == 0
    }

    pub fn get_cell(&self, pos: CellPosition) -> &Cell {
        self.cell_rows[pos.row].get_cell(pos.column)
    }

    pub fn live_neighbors_for(&self, row: usize, column: usize) -> u32 {
        if self.number_of_cells() == 1 {
            return 0;
        }

        let mut number_of_live_neighbors = 0;

        // Left top corner neighbor
        if row == 0
            || column == 0
            || self
                .get_cell(CellPosition::new(row - 1, column - 1))
                .is_live()
        {
            number_of_live_neighbors += 1;
        }

        // Top neighbor
        if row == 0 || self.get_cell(CellPosition::new(row - 1, column)).is_live() {
            number_of_live_neighbors += 1;
        }

        // Right top corner neighbor
        if row == 0
            || column == self.columns() - 1
            || self
                .get_cell(CellPosition::new(row - 1, column + 1))
                .is_live()
        {
            number_of_live_neighbors += 1;
        }

        // Left neighbor
        if column == 0 || self.get_cell(CellPosition::new(row, column - 1)).is_live() {
            number_of_live_neighbors += 1;
        }

        // Right neighbor
        if column == self.columns() - 1
            || self.get_cell(CellPosition::new(row, column + 1)).is_live()
        {
            number_of_live_neighbors += 1;
        }

        // Left bottom corner neighbor
        if row == self.rows() - 1
            || column == 0
            || self
                .get_cell(CellPosition::new(row + 1, column - 1))
                .is_live()
        {
            number_of_live_neighbors += 1;
        }

        // Bottom neighbor
        if row == self.rows() - 1 || self.get_cell(CellPosition::new(row + 1, column)).is_live() {
            number_of_live_neighbors += 1;
        }

        // Right Left bottom corner neighbor
        if row == self.rows() - 1
            || column == self.columns() - 1
            || self
                .get_cell(CellPosition::new(row + 1, column + 1))
                .is_live()
        {
            number_of_live_neighbors += 1;
        }

        number_of_live_neighbors
    }
}

#[cfg(test)]
mod tests {
    use crate::{cell::Cell, cell_row::CellRow, grid::Grid};

    #[test]
    fn the_default_grid_does_not_have_any_cell_row() {
        let grid = Grid::default();

        assert_eq!(grid.rows(), 0);
    }

    #[test]
    fn an_empty_grid_contains_no_cell_rows_or_columns() {
        let grid = Grid::new(vec![]);

        assert_eq!(grid.rows(), 0);
        assert_eq!(grid.columns(), 0);
    }

    #[test]
    fn a_1x1_grid_contains_one_cell_row_with_one_column() {
        let grid = Grid::new(vec![CellRow::new(vec![Cell::live()])]);

        assert_eq!(grid.rows(), 1);
        assert_eq!(grid.columns(), 1);
    }

    #[test]
    fn a_2x3_grid_contains_two_cell_rows_with_three_columns() {
        let grid = Grid::new(vec![
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
        ]);

        assert_eq!(grid.rows(), 2);
        assert_eq!(grid.columns(), 3);
    }

    #[test]
    #[should_panic]
    fn a_grid_should_only_contain_cell_rows_with_the_same_length() {
        Grid::new(vec![
            CellRow::new(vec![Cell::live()]),
            CellRow::new(vec![Cell::live(), Cell::live()]),
        ]);
    }

    #[test]
    fn a_2x3_grid_contains_six_cells() {
        let grid = Grid::new(vec![
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
        ]);

        assert_eq!(grid.number_of_cells(), 6);
    }

    #[test]
    fn a_grid_can_be_empty_if_it_does_not_contain_any_cell() {
        let grid = Grid::new(vec![]);

        assert!(grid.is_empty());
    }

    #[test]
    fn a_cell_in_a_1x1_grid_does_not_have_any_live_neighbors() {
        let grid = Grid::new(vec![CellRow::new(vec![Cell::live()])]);

        assert_eq!(grid.live_neighbors_for(0, 0), 0);
    }

    #[test]
    fn a_cell_in_the_center_of_a_3x3_grid_can_have_eight_live_neighbors() {
        let grid = Grid::new(vec![
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
        ]);

        assert_eq!(grid.live_neighbors_for(1, 1), 8);
    }

    fn all_live_3x3_grid() -> Grid {
        Grid::new(vec![
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
        ])
    }

    #[test]
    fn given_a_cell_without_neighbors_we_consider_them_live_neighbors() {
        let grid = all_live_3x3_grid();

        assert_eq!(grid.live_neighbors_for(0, 0), 8);
        assert_eq!(grid.live_neighbors_for(0, 1), 8);
        assert_eq!(grid.live_neighbors_for(0, 2), 8);
        assert_eq!(grid.live_neighbors_for(1, 0), 8);
        assert_eq!(grid.live_neighbors_for(1, 1), 8);
        assert_eq!(grid.live_neighbors_for(1, 2), 8);
        assert_eq!(grid.live_neighbors_for(2, 0), 8);
        assert_eq!(grid.live_neighbors_for(2, 1), 8);
        assert_eq!(grid.live_neighbors_for(2, 2), 8);
    }
}
