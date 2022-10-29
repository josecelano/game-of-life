use crate::cell::Cell;

pub struct CellRow {
    cells: Vec<Cell>,
}

impl CellRow {
    pub fn new(cells: Vec<Cell>) -> Self {
        Self { cells }
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn get_cell(&self, pos: usize) -> &Cell {
        &self.cells[pos]
    }
}

#[cfg(test)]
mod tests {
    use crate::cell::Cell;

    use super::CellRow;

    #[test]
    fn a_cell_row_contains_ordered_cells() {
        let cell_row = CellRow::new(vec![Cell::live()]);

        assert_eq!(*cell_row.get_cell(0), Cell::live());
    }

    #[test]
    fn a_cell_row_contains_a_fixed_amount_of_cells() {
        let cell_row = CellRow::new(vec![Cell::live()]);

        assert_eq!(cell_row.len(), 1);
    }
}
