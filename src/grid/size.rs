#[derive(Debug, PartialEq)]
pub struct Size {
    pub rows: usize,
    pub columns: usize,
}

impl Size {
    #[must_use]
    pub fn new(rows: usize, columns: usize) -> Self {
        Self { rows, columns }
    }

    #[must_use]
    pub fn number_of_cells(&self) -> usize {
        self.rows * self.columns
    }
}

#[cfg(test)]
mod should {
    use super::Size;

    #[test]
    fn have_rows_and_columns() {
        let grid_size = Size {
            rows: 1,
            columns: 2,
        };

        assert_eq!(grid_size.rows, 1);
        assert_eq!(grid_size.columns, 2);
    }

    #[test]
    fn return_the_number_of_cells() {
        let grid_size = Size {
            rows: 3,
            columns: 4,
        };

        assert_eq!(grid_size.number_of_cells(), 12);
    }
}
