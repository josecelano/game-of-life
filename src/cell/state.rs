use std::{error::Error, fmt};

pub const LIVE: char = '⬜';
pub const DEAD: char = '⬛';

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Live,
    Dead,
}

impl fmt::Display for State {
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
        write!(f, "Invalid char '{}' for cell state", self.invalid_char)
    }
}

impl TryFrom<char> for State {
    type Error = ParseCellStateFromCharError;

    fn try_from(state: char) -> Result<Self, Self::Error> {
        match state {
            LIVE => Ok(State::Live),
            DEAD => Ok(State::Dead),
            _ => Err(ParseCellStateFromCharError {
                invalid_char: state,
            }),
        }
    }
}

fn display(cell_state: &State) -> String {
    match cell_state {
        State::Live => LIVE.to_string(),
        State::Dead => DEAD.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cell::state::{State, DEAD, LIVE};

    #[test]
    fn it_should_be_displayed() {
        assert_eq!(format!("{}", State::Live), "⬜");
        assert_eq!(format!("{}", State::Dead), "⬛");
    }

    #[test]
    fn a_dead_cell_state_should_be_converted_from_its_char_representation() {
        assert_eq!(State::try_from(DEAD).unwrap(), State::Dead);
    }

    #[test]
    fn a_live_cell_state_should_be_converted_from_its_char_representation() {
        assert_eq!(State::try_from(LIVE).unwrap(), State::Live);
    }

    #[test]
    fn it_should_fail_trying_to_generate_a_cell_state_from_an_invalid_char() {
        let state = State::try_from('X');
        assert!(state.is_err());
    }

    #[test]
    fn given_it_fails_to_generate_the_cell_state_from_a_char_it_should_show_the_invalid_char_in_the_error(
    ) {
        let error = State::try_from('X').unwrap_err();
        assert_eq!(error.invalid_char, 'X');
    }
}
