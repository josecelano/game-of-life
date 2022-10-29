use crate::cell::Cell;

pub struct Grid {
    pub rows: u32,
    pub columns: u32,
    pub cells: Vec<Cell>,
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(0, 0, vec![])
    }
}

impl Grid {
    pub fn new(rows: u32, columns: u32, cells: Vec<Cell>) -> Self {
        Self {
            rows,
            columns,
            cells,
        }
    }

    pub fn rows(&self) -> u32 {
        self.rows
    }

    pub fn columns(&self) -> u32 {
        self.columns
    }

    pub fn get_cell(&self, row: u32, column: u32) -> &Cell {
        let cell_pos = row * self.columns + column;
        &self.cells[cell_pos as usize]
    }
}

#[cfg(test)]
mod tests {
    use crate::{cell::Cell, grid::Grid};

    #[test]
    fn a_grid_has_rows_and_columns() {
        let grid = Grid::default();

        assert_eq!(grid.rows(), 0);
        assert_eq!(grid.columns(), 0);
    }

    #[test]
    fn a_1x1_grid_contains_one_cell_in_position_0_0() {
        let grid = Grid {
            rows: 1,
            columns: 1,
            cells: vec![Cell::live()],
        };

        assert!(grid.get_cell(0, 0).is_live());
    }
}
