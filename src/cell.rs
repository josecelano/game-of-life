#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cell {
    live: bool,
}

/// Cell from char representation
pub fn c(state: char) -> Cell {
    match state {
        '⬜' => Cell::live(),
        '⬛' => Cell::dead(),
        _ => panic!("Invalid cell state"),
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
}

#[cfg(test)]
mod tests {
    use crate::cell::Cell;

    #[test]
    fn a_cell_could_be_live() {
        let cell = Cell::live();

        assert!(cell.is_live());
    }

    #[test]
    fn a_cell_could_be_dead() {
        let cell = Cell::dead();

        assert!(!cell.is_live());
    }
}
