use std::fmt;

pub const LIVE: char = '⬜';
pub const DEAD: char = '⬛';

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CellState {
    Live,
    Dead,
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", display(self))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidCellStateCharRepresentation;

impl TryFrom<char> for CellState {
    type Error = InvalidCellStateCharRepresentation;

    fn try_from(state: char) -> Result<Self, Self::Error> {
        match state {
            LIVE => Ok(CellState::Live),
            DEAD => Ok(CellState::Dead),
            _ => Err(InvalidCellStateCharRepresentation),
        }
    }
}

fn display(cell_state: &CellState) -> String {
    match cell_state {
        CellState::Live => LIVE.to_string(),
        CellState::Dead => DEAD.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cell_state::{CellState, DEAD, LIVE};

    #[test]
    fn a_life_cell_state_should_be_converted_from_its_char_representation() {
        assert_eq!(CellState::try_from(LIVE).unwrap(), CellState::Live);
    }

    #[test]
    fn a_dead_cell_state_should_be_converted_from_its_char_representation() {
        assert_eq!(CellState::try_from(DEAD).unwrap(), CellState::Dead);
    }

    #[test]
    fn trying_to_convert_from_an_invalid_char_should_fail() {
        let state = CellState::try_from(' ');
        assert!(state.is_err());
    }

    #[test]
    fn a_cell_state_can_by_displayed() {
        assert_eq!(format!("{}", CellState::Live), "⬜");
        assert_eq!(format!("{}", CellState::Dead), "⬛");
    }
}
