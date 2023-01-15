use std::fmt;

const LIVE: char = '⬜';
const DEAD: char = '⬛';

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cell {
    live: bool,
}

// Cell from char constructor
pub fn c(state: char) -> Cell {
    match state {
        LIVE => Cell::live(),
        DEAD => Cell::dead(),
        _ => panic!("Invalid char representation for cell {}", state),
    }
}

impl From<char> for Cell {
    fn from(state: char) -> Self {
        c(state)
    }
}

/// Char representation of Cell
fn display(cell: &Cell) -> String {
    match cell.is_live() {
        true => LIVE.to_string(),
        false => DEAD.to_string(),
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", display(self))
    }
}

impl Cell {
    pub fn live() -> Self {
        Self { live: true }
    }

    pub fn dead() -> Self {
        Self { live: false }
    }

    pub fn is_live(&self) -> bool {
        self.live
    }

    pub fn is_dead(&self) -> bool {
        !self.is_live()
    }
}

#[cfg(test)]
mod tests {
    use crate::cell::{c, Cell, DEAD, LIVE};

    #[test]
    fn a_cell_could_be_live() {
        let cell = Cell::live();

        assert!(cell.is_live());
    }

    #[test]
    fn a_cell_could_be_dead() {
        let cell = Cell::dead();

        assert!(cell.is_dead());
    }

    #[test]
    fn a_cell_could_be_displayed_as_a_char() {
        assert_eq!(format!("{}", Cell::live()), LIVE.to_string());
        assert_eq!(format!("{}", Cell::dead()), DEAD.to_string());
    }

    #[test]
    fn a_cell_should_be_created_from_its_char_representation() {
        assert_eq!(c(LIVE), Cell::live());
        assert_eq!(c(DEAD), Cell::dead());
    }

    #[test]
    fn a_cell_should_be_converted_from_its_char_representation() {
        assert_eq!(Cell::from(LIVE), Cell::live());
        assert_eq!(Cell::from(DEAD), Cell::dead());

        let live_cell: Cell = LIVE.into();
        let dead_cell: Cell = DEAD.into();

        assert_eq!(live_cell, Cell::live());
        assert_eq!(dead_cell, Cell::dead());
    }
}
