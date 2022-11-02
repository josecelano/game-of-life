use crate::cell::Cell;

#[derive(PartialEq, Debug, Clone)]
pub struct CellRow {
    cells: Vec<Cell>,
}

impl CellRow {
    pub fn new(cells: Vec<Cell>) -> Self {
        Self { cells }
    }

    pub fn of_dead_cells(length: usize) -> Self {
        Self {
            cells: vec![Cell::dead(); length],
        }
    }

    pub fn of_live_cells(length: usize) -> Self {
        Self {
            cells: vec![Cell::live(); length],
        }
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

    pub fn position_is_valid(&self, pos: usize) -> bool {
        pos < self.cells.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::cell::Cell;

    use super::CellRow;

    #[test]
    fn it_contains_ordered_cells() {
        let cell_row = CellRow::new(vec![Cell::live()]);

        assert_eq!(*cell_row.get_cell(0), Cell::live());
    }

    #[test]
    fn it_contains_a_fixed_amount_of_cells() {
        let cell_row = CellRow::new(vec![Cell::live()]);

        assert_eq!(cell_row.len(), 1);
    }

    #[test]
    fn there_is_a_short_way_to_build_a_cell_row_of_only_dead_cells() {
        assert_eq!(CellRow::of_dead_cells(1), CellRow::new(vec![Cell::dead()]));
    }

    #[test]
    fn there_is_a_short_way_to_build_a_cell_row_of_only_live_cells() {
        assert_eq!(CellRow::of_live_cells(1), CellRow::new(vec![Cell::live()]));
    }

    #[test]
    fn it_can_validate_a_cell_position() {
        assert!(CellRow::of_dead_cells(10).position_is_valid(0));
        assert!(CellRow::of_dead_cells(10).position_is_valid(9));

        assert!(!CellRow::of_dead_cells(1).position_is_valid(10));
    }
}
