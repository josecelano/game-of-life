pub struct CellCoordinates {
    pub row: usize,
    pub column: usize,
}

impl CellCoordinates {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn is_left_top_corner(&self) -> bool {
        self.row == 0 && self.column == 0
    }
}

#[cfg(test)]
mod tests {

    use crate::cell_coordinates::CellCoordinates;

    #[test]
    fn cell_coordinates_contain_a_row_and_column() {
        let pos = CellCoordinates::new(0, 0);

        assert_eq!(pos.row, 0);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn coordinates_at_row_0_and_column_0_represent_the_left_top_corner_of_a_grid() {
        assert!(CellCoordinates::new(0, 0).is_left_top_corner())
    }
}
