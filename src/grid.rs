use crate::{cell::Cell, cell_row::CellRow};

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

    pub fn get_cell(&self, row: usize, column: usize) -> &Cell {
        self.cell_rows[row].get_cell(column)
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
}
