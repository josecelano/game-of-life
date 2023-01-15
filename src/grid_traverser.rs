use crate::{cell_coordinates::CellCoordinates, grid_size::GridSize};

pub struct GridTraverser {
    index: usize,
    grid_size: GridSize,
}

impl GridTraverser {
    pub fn new(grid_size: GridSize) -> Self {
        Self {
            index: 0,
            grid_size,
        }
    }

    fn current_row(&self) -> usize {
        self.index / self.grid_size.columns
    }

    fn current_column(&self) -> usize {
        if self.index < self.grid_size.columns - 1 {
            self.index
        } else {
            self.index % self.grid_size.columns
        }
    }
}

impl Iterator for GridTraverser {
    type Item = CellCoordinates;

    fn next(&mut self) -> Option<Self::Item> {
        if self.grid_size.number_of_cells() == 0 {
            return None;
        }

        if self.index >= self.grid_size.number_of_cells() {
            return None;
        }

        let current_coordinates = CellCoordinates::new(self.current_row(), self.current_column());

        self.index += 1;

        Some(current_coordinates)
    }
}

#[cfg(test)]
mod should_traverse_the_grid_cell_coordinates_row_wise {
    use crate::{
        cell_coordinates::CellCoordinates, grid_size::GridSize, grid_traverser::GridTraverser,
    };

    #[test]
    fn for_an_empty_grid() {
        let mut traverser = GridTraverser::new(GridSize::new(0, 0));
        assert_eq!(traverser.next(), None);
    }

    #[test]
    fn for_non_empty_grid() {
        let mut traverser = GridTraverser::new(GridSize::new(2, 3));
        assert_eq!(traverser.next(), Some(CellCoordinates::new(0, 0)));
        assert_eq!(traverser.next(), Some(CellCoordinates::new(0, 1)));
        assert_eq!(traverser.next(), Some(CellCoordinates::new(0, 2)));
        assert_eq!(traverser.next(), Some(CellCoordinates::new(1, 0)));
        assert_eq!(traverser.next(), Some(CellCoordinates::new(1, 1)));
        assert_eq!(traverser.next(), Some(CellCoordinates::new(1, 2)));
        assert_eq!(traverser.next(), None);
    }
}
