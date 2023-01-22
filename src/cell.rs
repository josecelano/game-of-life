use std::fmt;

use crate::cell_state::CellState;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cell {
    state: CellState,
}

pub fn c(state: char) -> Cell {
    match CellState::try_from(state) {
        Ok(state) => Cell::new(state),
        Err(_) => panic!("Invalid char representation for cell {}", state),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidCellCharRepresentation;

impl TryFrom<char> for Cell {
    type Error = InvalidCellCharRepresentation;

    fn try_from(state: char) -> Result<Self, Self::Error> {
        match CellState::try_from(state) {
            Ok(state) => Ok(Cell::new(state)),
            _ => Err(InvalidCellCharRepresentation),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.state)
    }
}

impl Cell {
    pub fn live() -> Self {
        Self::new(CellState::Live)
    }

    pub fn dead() -> Self {
        Self::new(CellState::Dead)
    }

    fn new(state: CellState) -> Self {
        Self { state }
    }

    pub fn is_live(&self) -> bool {
        self.state == CellState::Live
    }

    pub fn is_dead(&self) -> bool {
        self.state == CellState::Dead
    }

    pub fn state(&self) -> CellState {
        self.state.to_owned()
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        cell::{c, Cell, CellState},
        cell_state::{DEAD, LIVE},
    };

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
    fn a_cell_can_be_created_using_its_state() {
        assert!(Cell::new(CellState::Live).is_live());
        assert!(Cell::new(CellState::Dead).is_dead());
    }

    #[test]
    fn a_cell_could_be_displayed_as_a_single_char_string() {
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
        assert_eq!(Cell::try_from(LIVE).unwrap(), Cell::live());
        assert_eq!(Cell::try_from(DEAD).unwrap(), Cell::dead());

        let live_cell: Cell = LIVE.try_into().unwrap();
        let dead_cell: Cell = DEAD.try_into().unwrap();

        assert_eq!(live_cell, Cell::live());
        assert_eq!(dead_cell, Cell::dead());
    }
}
