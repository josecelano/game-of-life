pub mod coordinates;
pub mod row;
pub mod state;

use crate::cell::state::State;
use std::fmt;

use self::state::ParseCellStateFromCharError;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cell {
    state: State,
}

/// # Panics
///
/// Will fail if the char is not a char representing one of the cell states.
#[must_use]
pub fn c(state: char) -> Cell {
    let state = State::try_from(state)
        .expect("the char should be a valid char representation for the cell state");
    Cell::new(state)
}

impl TryFrom<char> for Cell {
    type Error = ParseCellStateFromCharError;

    fn try_from(state: char) -> Result<Self, Self::Error> {
        match State::try_from(state) {
            Ok(state) => Ok(Cell::new(state)),
            Err(error) => Err(error),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.state)
    }
}

impl Cell {
    #[must_use]
    pub fn live() -> Self {
        Self::new(State::Live)
    }

    #[must_use]
    pub fn dead() -> Self {
        Self::new(State::Dead)
    }

    fn new(state: State) -> Self {
        Self { state }
    }

    #[must_use]
    pub fn is_live(&self) -> bool {
        self.state == State::Live
    }

    #[must_use]
    pub fn is_dead(&self) -> bool {
        self.state == State::Dead
    }

    #[must_use]
    pub fn state(&self) -> State {
        self.state.clone()
    }
}

#[cfg(test)]
mod tests {

    use crate::cell::{
        c,
        state::{DEAD, LIVE},
        Cell, State,
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
        assert!(Cell::new(State::Live).is_live());
        assert!(Cell::new(State::Dead).is_dead());
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
