use std::{fmt, str::FromStr};

use crate::{cell::Cell, cell_state::ParseCellStateFromCharError};
use std::fmt::Write;

#[derive(PartialEq, Debug, Clone)]
pub struct CellRow {
    cells: Vec<Cell>,
}

impl FromStr for CellRow {
    type Err = ParseCellStateFromCharError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut cells_row = vec![];

        for c in text.trim().chars() {
            match Cell::try_from(c) {
                Ok(cell) => cells_row.push(cell),
                Err(error) => return Err(error),
            };
        }

        Ok(CellRow::new(cells_row))
    }
}

impl fmt::Display for CellRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        for cell in &self.cells {
            write!(&mut output, "{}", &cell).unwrap();
        }

        write!(f, "{}", output)
    }
}

impl CellRow {
    pub fn new(cells: Vec<Cell>) -> Self {
        Self { cells }
    }

    pub fn with(cells: Vec<Cell>) -> Self {
        Self::new(cells)
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
    use crate::cell::{c, Cell};

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
    fn it_can_validate_if_a_given_position_is_valid_in_the_row_starting_at_zero() {
        // Valid positions
        assert!(CellRow::of_dead_cells(10).position_is_valid(0));
        assert!(CellRow::of_dead_cells(10).position_is_valid(9));

        // Invalid positions
        assert!(!CellRow::of_dead_cells(1).position_is_valid(10));
    }

    #[test]
    fn it_should_be_displayed() {
        assert_eq!(
            format!("{}", CellRow::new(vec![c('⬜'), c('⬛'), c('⬛')])),
            "⬜⬛⬛"
        );
    }

    #[test]
    fn it_should_be_generated_from_a_string() {
        assert_eq!(
            "⬛⬜".parse::<CellRow>().unwrap(),
            CellRow::new(vec![c('⬛'), c('⬜')])
        );
    }

    #[test]
    fn it_should_fail_trying_to_generate_it_from_an_invalid_string() {
        assert!("X".parse::<CellRow>().is_err());
    }

    #[test]
    fn given_it_fails_to_generate_the_cell_row_from_a_string_it_should_show_the_invalid_char_in_the_error(
    ) {
        assert_eq!("X".parse::<CellRow>().unwrap_err().invalid_char, 'X');
    }
}
