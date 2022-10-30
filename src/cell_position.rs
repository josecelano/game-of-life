pub struct CellPosition {
    pub row: usize,
    pub column: usize,
}

impl CellPosition {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
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
}
