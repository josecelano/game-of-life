pub struct CellPosition {
    pub row: usize,
    pub column: usize,
}

impl CellPosition {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn is_left_top_corner(&self) -> bool {
        self.row == 0 && self.column == 0
    }
}

#[cfg(test)]
mod tests {

    use crate::cell_position::CellPosition;

    #[test]
    fn a_cell_position_are_the_coordinates_of_a_cell_in_a_grid_using_row_and_column() {
        let pos = CellPosition::new(0, 0);

        assert_eq!(pos.row, 0);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn the_cell_position_at_row_0_and_column_0_is_the_left_top_corner() {
        assert!(CellPosition::new(0, 0).is_left_top_corner())
    }
}
