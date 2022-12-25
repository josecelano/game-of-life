use crate::{cell::Cell, cell_coordinates::CellCoordinates, cell_row::CellRow};

#[derive(PartialEq, Debug, Clone)]
pub struct Grid {
    pub cell_rows: Vec<CellRow>,
}

#[derive(Clone, Copy)]
enum NeighborPosition {
    LeftTop,
    Top,
    RightTop,
    Left,
    Right,
    LeftBottom,
    Bottom,
    RightBottom,
}

fn neighbor_positions() -> Vec<NeighborPosition> {
    vec![
        NeighborPosition::LeftTop,
        NeighborPosition::Top,
        NeighborPosition::RightTop,
        NeighborPosition::Left,
        NeighborPosition::Right,
        NeighborPosition::LeftBottom,
        NeighborPosition::Bottom,
        NeighborPosition::RightBottom,
    ]
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl Grid {
    pub fn new(cell_rows: Vec<CellRow>) -> Self {
        if !cell_rows.is_empty() {
            let first_row_len = cell_rows[0].len();
            cell_rows.iter().for_each(|cell_row| {
                if cell_row.len() != first_row_len {
                    panic!("Cell rows do not have the same length");
                }
            });
        }
        Self { cell_rows }
    }

    pub fn new_empty() -> Self {
        Self { cell_rows: vec![] }
    }

    pub fn of_dead_cells(rows: usize, columns: usize) -> Self {
        Self {
            cell_rows: vec![CellRow::of_dead_cells(columns); rows],
        }
    }

    pub fn of_live_cells(rows: usize, columns: usize) -> Self {
        Self {
            cell_rows: vec![CellRow::of_live_cells(columns); rows],
        }
    }

    pub fn rows(&self) -> usize {
        self.cell_rows.len()
    }

    pub fn columns(&self) -> usize {
        if self.cell_rows.is_empty() {
            return 0;
        }
        self.cell_rows[0].len()
    }

    pub fn number_of_cells(&self) -> usize {
        self.rows() * self.columns()
    }

    pub fn is_empty(&self) -> bool {
        self.number_of_cells() == 0
    }

    pub fn get_cell(&self, pos: CellCoordinates) -> &Cell {
        self.cell_rows[pos.row].get_cell(pos.column)
    }

    pub fn number_of_live_neighbors_for(&self, cell_coordinates: CellCoordinates) -> usize {
        if self.number_of_cells() == 1 {
            return 0;
        }

        let neighbors = self.get_neighbors(&cell_coordinates);

        let live_neighbors =
            neighbors
                .iter()
                .fold(0, |counter, neighbor| match neighbor.is_live() {
                    true => counter + 1,
                    false => counter,
                });

        live_neighbors
    }

    pub fn has_same_dimensions(&self, other: &Self) -> bool {
        self.rows() == other.rows() && self.columns() == other.columns()
    }

    pub fn position_is_valid(&self, cell_coordinates: &CellCoordinates) -> bool {
        cell_coordinates.row < self.rows() && cell_coordinates.column < self.columns()
    }

    fn get_neighbors(&self, cell_coordinates: &CellCoordinates) -> Vec<&Cell> {
        neighbor_positions()
            .iter()
            .filter_map(|neighbor| self.get_neighbor(cell_coordinates, *neighbor))
            .collect()
    }

    fn get_neighbor(
        &self,
        cell_coordinates: &CellCoordinates,
        position: NeighborPosition,
    ) -> Option<&Cell> {
        match position {
            NeighborPosition::LeftTop => self.left_top_neighbor(cell_coordinates),
            NeighborPosition::Top => self.top_neighbor(cell_coordinates),
            NeighborPosition::RightTop => self.right_top_neighbor(cell_coordinates),
            NeighborPosition::Left => self.left_neighbor(cell_coordinates),
            NeighborPosition::Right => self.right_neighbor(cell_coordinates),
            NeighborPosition::LeftBottom => self.left_bottom_neighbor(cell_coordinates),
            NeighborPosition::Bottom => self.bottom_neighbor(cell_coordinates),
            NeighborPosition::RightBottom => self.right_bottom_neighbor(cell_coordinates),
        }
    }

    fn left_top_neighbor(&self, cell_coordinates: &CellCoordinates) -> Option<&Cell> {
        if !self.has_left_top_neighbor(cell_coordinates) {
            return None;
        }

        Some(self.get_cell(CellCoordinates::new(
            cell_coordinates.row - 1,
            cell_coordinates.column - 1,
        )))
    }

    fn has_left_top_neighbor(&self, cell_coordinates: &CellCoordinates) -> bool {
        !self.is_left_top_corner(cell_coordinates)
    }

    fn is_left_top_corner(&self, cell_coordinates: &CellCoordinates) -> bool {
        cell_coordinates.row == 0 || cell_coordinates.column == 0
    }

    fn top_neighbor(&self, cell_coordinates: &CellCoordinates) -> Option<&Cell> {
        if !self.has_top_neighbor(cell_coordinates) {
            return None;
        }

        Some(self.get_cell(CellCoordinates::new(
            cell_coordinates.row - 1,
            cell_coordinates.column,
        )))
    }

    fn has_top_neighbor(&self, cell_coordinates: &CellCoordinates) -> bool {
        !self.is_top_row(cell_coordinates)
    }

    fn is_top_row(&self, cell_coordinates: &CellCoordinates) -> bool {
        cell_coordinates.row == 0
    }

    fn right_top_neighbor(&self, cell_coordinates: &CellCoordinates) -> Option<&Cell> {
        if !self.has_right_top_neighbor(cell_coordinates) {
            return None;
        }

        Some(self.get_cell(CellCoordinates::new(
            cell_coordinates.row - 1,
            cell_coordinates.column + 1,
        )))
    }

    fn has_right_top_neighbor(&self, cell_coordinates: &CellCoordinates) -> bool {
        !self.is_right_top_corner(cell_coordinates)
    }

    fn is_right_top_corner(&self, cell_coordinates: &CellCoordinates) -> bool {
        cell_coordinates.row == 0 || cell_coordinates.column == self.columns() - 1
    }

    fn left_neighbor(&self, cell_coordinates: &CellCoordinates) -> Option<&Cell> {
        if !self.has_left_neighbor(cell_coordinates) {
            return None;
        }

        Some(self.get_cell(CellCoordinates::new(
            cell_coordinates.row,
            cell_coordinates.column - 1,
        )))
    }

    fn has_left_neighbor(&self, cell_coordinates: &CellCoordinates) -> bool {
        !self.is_left_column(cell_coordinates)
    }

    fn is_left_column(&self, cell_coordinates: &CellCoordinates) -> bool {
        cell_coordinates.column == 0
    }

    fn right_neighbor(&self, cell_coordinates: &CellCoordinates) -> Option<&Cell> {
        if !self.has_right_neighbor(cell_coordinates) {
            return None;
        }

        Some(self.get_cell(CellCoordinates::new(
            cell_coordinates.row,
            cell_coordinates.column + 1,
        )))
    }

    fn has_right_neighbor(&self, cell_coordinates: &CellCoordinates) -> bool {
        !self.is_right_column(cell_coordinates)
    }

    fn is_right_column(&self, cell_coordinates: &CellCoordinates) -> bool {
        cell_coordinates.column == self.columns() - 1
    }

    fn left_bottom_neighbor(&self, cell_coordinates: &CellCoordinates) -> Option<&Cell> {
        if !self.has_left_bottom_neighbor(cell_coordinates) {
            return None;
        }

        Some(self.get_cell(CellCoordinates::new(
            cell_coordinates.row + 1,
            cell_coordinates.column - 1,
        )))
    }

    fn has_left_bottom_neighbor(&self, cell_coordinates: &CellCoordinates) -> bool {
        !self.is_left_bottom_corner(cell_coordinates)
    }

    fn is_left_bottom_corner(&self, cell_coordinates: &CellCoordinates) -> bool {
        cell_coordinates.row == self.rows() - 1 || cell_coordinates.column == 0
    }

    fn bottom_neighbor(&self, cell_coordinates: &CellCoordinates) -> Option<&Cell> {
        if !self.has_bottom_neighbor(cell_coordinates) {
            return None;
        }

        Some(self.get_cell(CellCoordinates::new(
            cell_coordinates.row + 1,
            cell_coordinates.column,
        )))
    }

    fn has_bottom_neighbor(&self, cell_coordinates: &CellCoordinates) -> bool {
        !self.is_bottom_row(cell_coordinates)
    }

    fn is_bottom_row(&self, cell_coordinates: &CellCoordinates) -> bool {
        cell_coordinates.row == self.rows() - 1
    }

    fn right_bottom_neighbor(&self, cell_coordinates: &CellCoordinates) -> Option<&Cell> {
        if !self.has_right_bottom_neighbor(cell_coordinates) {
            return None;
        }

        Some(self.get_cell(CellCoordinates::new(
            cell_coordinates.row + 1,
            cell_coordinates.column + 1,
        )))
    }

    fn has_right_bottom_neighbor(&self, cell_coordinates: &CellCoordinates) -> bool {
        !self.is_right_bottom_corner(cell_coordinates)
    }

    fn is_right_bottom_corner(&self, cell_coordinates: &CellCoordinates) -> bool {
        cell_coordinates.row == self.rows() - 1 || cell_coordinates.column == self.columns() - 1
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cell::{c, Cell},
        cell_coordinates::CellCoordinates,
        cell_row::CellRow,
        grid::Grid,
    };

    #[test]
    fn the_default_grid_does_not_have_any_cell_row() {
        let grid = Grid::default();

        assert_eq!(grid.rows(), 0);
    }

    #[test]
    fn an_empty_grid_contains_no_cell_rows_or_columns() {
        let grid = Grid::new_empty();

        assert_eq!(grid.rows(), 0);
        assert_eq!(grid.columns(), 0);
    }

    #[test]
    fn a_1x1_grid_contains_one_cell_row_with_one_column() {
        let grid = Grid::new(vec![CellRow::new(vec![Cell::live()])]);

        assert_eq!(grid.rows(), 1);
        assert_eq!(grid.columns(), 1);
    }

    #[test]
    fn a_2x3_grid_contains_two_cell_rows_with_three_columns() {
        let grid = Grid::new(vec![
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
        ]);

        assert_eq!(grid.rows(), 2);
        assert_eq!(grid.columns(), 3);
    }

    #[test]
    #[should_panic]
    fn a_grid_should_only_contain_cell_rows_with_the_same_length() {
        Grid::new(vec![
            CellRow::new(vec![Cell::live()]),
            CellRow::new(vec![Cell::live(), Cell::live()]),
        ]);
    }

    #[test]
    fn a_2x3_grid_contains_six_cells() {
        let grid = Grid::new(vec![
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
        ]);

        assert_eq!(grid.number_of_cells(), 6);
    }

    #[test]
    fn a_grid_can_be_empty_if_it_does_not_contain_any_cell() {
        let grid = Grid::new(vec![]);

        assert!(grid.is_empty());
    }

    #[test]
    fn a_cell_in_a_1x1_grid_does_not_have_any_live_neighbors() {
        let grid = Grid::new(vec![CellRow::new(vec![Cell::live()])]);

        assert_eq!(
            grid.number_of_live_neighbors_for(CellCoordinates::new(0, 0)),
            0
        );
    }

    #[test]
    fn a_cell_in_the_center_of_a_3x3_grid_can_have_eight_live_neighbors() {
        let grid = Grid::new(vec![
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
        ]);

        assert_eq!(
            grid.number_of_live_neighbors_for(CellCoordinates::new(1, 1)),
            8
        );
    }

    fn all_live_3x3_grid() -> Grid {
        Grid::new(vec![
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
        ])
    }

    #[test]
    fn given_a_cell_without_neighbors_we_consider_them_dead_neighbors() {
        let grid = all_live_3x3_grid();

        assert_eq!(
            grid.number_of_live_neighbors_for(CellCoordinates::new(0, 0)),
            3
        );
        assert_eq!(
            grid.number_of_live_neighbors_for(CellCoordinates::new(0, 1)),
            5
        );
        assert_eq!(
            grid.number_of_live_neighbors_for(CellCoordinates::new(0, 2)),
            3
        );
        assert_eq!(
            grid.number_of_live_neighbors_for(CellCoordinates::new(1, 0)),
            5
        );
        // Cell (1,1) has all the neighbors
        assert_eq!(
            grid.number_of_live_neighbors_for(CellCoordinates::new(1, 2)),
            5
        );
        assert_eq!(
            grid.number_of_live_neighbors_for(CellCoordinates::new(2, 0)),
            3
        );
        assert_eq!(
            grid.number_of_live_neighbors_for(CellCoordinates::new(2, 1)),
            5
        );
        assert_eq!(
            grid.number_of_live_neighbors_for(CellCoordinates::new(2, 2)),
            3
        );
    }

    #[test]
    fn it_should_calculate_the_number_of_live_neighbors() {
        let grid = Grid::new(vec![
            CellRow::new(vec![c('⬜'), c('⬛'), c('⬛')]),
            CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
            CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
        ]);

        assert_eq!(
            grid.number_of_live_neighbors_for(CellCoordinates::new(1, 1)),
            1
        );

        let grid = Grid::new(vec![
            CellRow::new(vec![c('⬜'), c('⬜'), c('⬛')]),
            CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
            CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
        ]);

        assert_eq!(
            grid.number_of_live_neighbors_for(CellCoordinates::new(1, 1)),
            2
        );

        let grid = Grid::new(vec![
            CellRow::new(vec![c('⬜'), c('⬜'), c('⬜')]),
            CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
            CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
        ]);

        assert_eq!(
            grid.number_of_live_neighbors_for(CellCoordinates::new(1, 1)),
            3
        );

        let grid = Grid::new(vec![
            CellRow::new(vec![c('⬜'), c('⬜'), c('⬜')]),
            CellRow::new(vec![c('⬜'), c('⬛'), c('⬛')]),
            CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
        ]);

        assert_eq!(
            grid.number_of_live_neighbors_for(CellCoordinates::new(1, 1)),
            4
        );

        let grid = Grid::new(vec![
            CellRow::new(vec![c('⬜'), c('⬜'), c('⬜')]),
            CellRow::new(vec![c('⬜'), c('⬛'), c('⬜')]),
            CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
        ]);

        assert_eq!(
            grid.number_of_live_neighbors_for(CellCoordinates::new(1, 1)),
            5
        );

        let grid = Grid::new(vec![
            CellRow::new(vec![c('⬜'), c('⬜'), c('⬜')]),
            CellRow::new(vec![c('⬜'), c('⬛'), c('⬜')]),
            CellRow::new(vec![c('⬜'), c('⬛'), c('⬛')]),
        ]);

        assert_eq!(
            grid.number_of_live_neighbors_for(CellCoordinates::new(1, 1)),
            6
        );

        let grid = Grid::new(vec![
            CellRow::new(vec![c('⬜'), c('⬜'), c('⬜')]),
            CellRow::new(vec![c('⬜'), c('⬛'), c('⬜')]),
            CellRow::new(vec![c('⬜'), c('⬜'), c('⬛')]),
        ]);

        assert_eq!(
            grid.number_of_live_neighbors_for(CellCoordinates::new(1, 1)),
            7
        );

        let grid = Grid::new(vec![
            CellRow::new(vec![c('⬜'), c('⬜'), c('⬜')]),
            CellRow::new(vec![c('⬜'), c('⬛'), c('⬜')]),
            CellRow::new(vec![c('⬜'), c('⬜'), c('⬜')]),
        ]);

        assert_eq!(
            grid.number_of_live_neighbors_for(CellCoordinates::new(1, 1)),
            8
        );
    }

    #[test]
    fn there_is_a_short_way_to_build_a_grid_of_only_dead_cells() {
        assert_eq!(
            Grid::of_dead_cells(1, 1),
            Grid::new(vec![CellRow::of_dead_cells(1)])
        );
    }

    #[test]
    fn there_is_a_short_way_to_build_a_grid_of_only_live_cells() {
        assert_eq!(
            Grid::of_live_cells(1, 1),
            Grid::new(vec![CellRow::of_live_cells(1)])
        );
    }

    #[test]
    fn two_grids_have_the_same_dimensions_if_they_have_the_same_amount_of_rows_and_columns() {
        assert!(Grid::of_dead_cells(1, 1).has_same_dimensions(&Grid::of_dead_cells(1, 1)));
        assert!(!Grid::of_dead_cells(1, 1).has_same_dimensions(&Grid::of_dead_cells(1, 2)));
    }

    #[test]
    fn it_can_validate_is_the_grid_contains_a_cell_at_a_given_coordinates() {
        assert!(Grid::of_dead_cells(10, 10).position_is_valid(&CellCoordinates::new(0, 0)));
        assert!(Grid::of_dead_cells(10, 10).position_is_valid(&CellCoordinates::new(0, 9)));
        assert!(Grid::of_dead_cells(10, 10).position_is_valid(&CellCoordinates::new(9, 0)));
        assert!(Grid::of_dead_cells(10, 10).position_is_valid(&CellCoordinates::new(9, 9)));

        assert!(!Grid::of_dead_cells(10, 10).position_is_valid(&CellCoordinates::new(10, 0)));
        assert!(!Grid::of_dead_cells(10, 10).position_is_valid(&CellCoordinates::new(0, 10)));
    }
}
