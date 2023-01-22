use std::{error::Error, fmt};

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
pub struct ParseCellStateFromCharError {
    pub invalid_char: char,
}

impl Error for ParseCellStateFromCharError {}

impl fmt::Display for ParseCellStateFromCharError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            format!("Invalid char '{}' for cell state", self.invalid_char)
        )
    }
}

impl TryFrom<char> for CellState {
    type Error = ParseCellStateFromCharError;

    fn try_from(state: char) -> Result<Self, Self::Error> {
        match state {
            LIVE => Ok(CellState::Live),
            DEAD => Ok(CellState::Dead),
            _ => Err(ParseCellStateFromCharError {
                invalid_char: state,
            }),
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
    fn it_should_be_displayed() {
        assert_eq!(format!("{}", CellState::Live), "⬜");
        assert_eq!(format!("{}", CellState::Dead), "⬛");
    }

    #[test]
    fn a_dead_cell_state_should_be_converted_from_its_char_representation() {
        assert_eq!(CellState::try_from(DEAD).unwrap(), CellState::Dead);
    }

    #[test]
    fn a_live_cell_state_should_be_converted_from_its_char_representation() {
        assert_eq!(CellState::try_from(LIVE).unwrap(), CellState::Live);
    }

    #[test]
    fn it_should_fail_trying_to_generate_a_cell_state_from_an_invalid_char() {
        let state = CellState::try_from('X');
        assert!(state.is_err());
    }

    #[test]
    fn given_it_fails_to_generate_the_cell_state_from_a_char_it_should_show_the_invalid_char_in_the_error(
    ) {
        let error = CellState::try_from('X').unwrap_err();
        assert_eq!(error.invalid_char, 'X');
    }
}
