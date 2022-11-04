#[derive(Debug, PartialEq)]
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

    pub fn recalculate_to_origin(&self, new_origin: &CellCoordinates) -> Self {
        let front_grid_row = self.row - new_origin.row;
        let front_grid_column = self.column - new_origin.column;
        CellCoordinates::new(front_grid_row, front_grid_column)
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
    fn cell_coordinates_at_row_0_and_column_0_represent_the_left_top_corner_of_a_grid() {
        assert!(CellCoordinates::new(0, 0).is_left_top_corner());
    }

    #[test]
    fn cell_coordinates_can_be_recalculate_to_a_different_coordinates_origin() {
        /*
          Given a grid where coordinates origin is at row o column 0:

            0 1 2 3 4 5
          0 A
          1   B
          2     C
          3       D
          4         E
          5           F

          We can calculate what would be the cell coordinates for a given cell using a different origin.

          For example, for the cell (2,2) called C:

          - If we move the origin to (2,2) the new coordinates for C are (0,0)
          - If we move the origin to (1,1) the new coordinates for c are (1,1)
        */

        let cell_coordinates = CellCoordinates::new(2, 2);

        assert_eq!(
            cell_coordinates.recalculate_to_origin(&CellCoordinates::new(2, 2)),
            CellCoordinates::new(0, 0)
        );

        assert_eq!(
            cell_coordinates.recalculate_to_origin(&CellCoordinates::new(1, 1)),
            CellCoordinates::new(1, 1)
        );
    }
}
