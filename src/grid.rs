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

        assert_eq!(neighbors.len(), 8);

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
        Some(self.get_cell(self.cell_coordinate_translate(cell_coordinates, -1, -1)))
    }

    fn top_neighbor(&self, cell_coordinates: &CellCoordinates) -> Option<&Cell> {
        Some(self.get_cell(self.cell_coordinate_translate(cell_coordinates, -1, 0)))
    }

    fn right_top_neighbor(&self, cell_coordinates: &CellCoordinates) -> Option<&Cell> {
        Some(self.get_cell(self.cell_coordinate_translate(cell_coordinates, -1, 1)))
    }

    fn left_neighbor(&self, cell_coordinates: &CellCoordinates) -> Option<&Cell> {
        Some(self.get_cell(self.cell_coordinate_translate(cell_coordinates, 0, -1)))
    }

    fn right_neighbor(&self, cell_coordinates: &CellCoordinates) -> Option<&Cell> {
        Some(self.get_cell(self.cell_coordinate_translate(cell_coordinates, 0, 1)))
    }

    fn left_bottom_neighbor(&self, cell_coordinates: &CellCoordinates) -> Option<&Cell> {
        Some(self.get_cell(self.cell_coordinate_translate(cell_coordinates, 1, -1)))
    }

    fn bottom_neighbor(&self, cell_coordinates: &CellCoordinates) -> Option<&Cell> {
        Some(self.get_cell(self.cell_coordinate_translate(cell_coordinates, 1, 0)))
    }

    fn right_bottom_neighbor(&self, cell_coordinates: &CellCoordinates) -> Option<&Cell> {
        Some(self.get_cell(self.cell_coordinate_translate(cell_coordinates, 1, 1)))
    }

    /// It handles toroidal array positions
    fn cell_coordinate_translate(
        &self,
        cell_coordinates: &CellCoordinates,
        row: i64,
        column: i64,
    ) -> CellCoordinates {
        let mut new_row = cell_coordinates.row as i64 + row;
        let mut new_column = cell_coordinates.column as i64 + column;

        if new_row < 0 {
            new_row = self.rows() as i64 - 1;
        }

        if new_row > self.rows() as i64 - 1 {
            new_row = 0;
        }

        if new_column < 0 {
            new_column = self.columns() as i64 - 1;
        }

        if new_column > self.columns() as i64 - 1 {
            new_column = 0;
        }

        CellCoordinates::new(new_row as usize, new_column as usize)
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

    mod the_edges_are_stitched_together {
        use crate::{cell::c, cell_coordinates::CellCoordinates, cell_row::CellRow, grid::Grid};

        #[test]
        fn no_left_top_neighbor_case() {
            // Case 1: left top corner
            assert!(Grid::new(vec![
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬜')]),
            ])
            .left_top_neighbor(&CellCoordinates::new(0, 0))
            .unwrap()
            .is_live());

            // Case 2: top row
            assert!(Grid::new(vec![
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬜'), c('⬛'), c('⬛')]),
            ])
            .left_top_neighbor(&CellCoordinates::new(0, 1))
            .unwrap()
            .is_live());

            // Case 3: left column
            assert!(Grid::new(vec![
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬜')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
            ])
            .left_top_neighbor(&CellCoordinates::new(1, 0))
            .unwrap()
            .is_live());
        }

        #[test]
        fn no_top_neighbor_case() {
            assert!(Grid::new(vec![
                CellRow::new(vec![c('⬛')]),
                CellRow::new(vec![c('⬛')]),
                CellRow::new(vec![c('⬜')]),
            ])
            .top_neighbor(&CellCoordinates::new(0, 0))
            .unwrap()
            .is_live());
        }

        #[test]
        fn no_right_top_neighbor_case() {
            // Case 1: right top corner
            assert!(Grid::new(vec![
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬜'), c('⬛'), c('⬛')]),
            ])
            .right_top_neighbor(&CellCoordinates::new(0, 2))
            .unwrap()
            .is_live());

            // Case 2: top row
            assert!(Grid::new(vec![
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬜')]),
            ])
            .right_top_neighbor(&CellCoordinates::new(0, 1))
            .unwrap()
            .is_live());

            // Case 3: right column
            assert!(Grid::new(vec![
                CellRow::new(vec![c('⬜'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
            ])
            .right_top_neighbor(&CellCoordinates::new(1, 2))
            .unwrap()
            .is_live());
        }

        #[test]
        fn no_left_neighbor_case() {
            assert!(
                Grid::new(vec![CellRow::new(vec![c('⬛'), c('⬛'), c('⬜')]),])
                    .left_neighbor(&CellCoordinates::new(0, 0))
                    .unwrap()
                    .is_live()
            );
        }

        #[test]
        fn no_right_neighbor_case() {
            assert!(
                Grid::new(vec![CellRow::new(vec![c('⬜'), c('⬛'), c('⬛')]),])
                    .right_neighbor(&CellCoordinates::new(0, 2))
                    .unwrap()
                    .is_live()
            );
        }

        #[test]
        fn no_left_bottom_neighbor_case() {
            // Case 1: left bottom corner
            assert!(Grid::new(vec![
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬜')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
            ])
            .left_bottom_neighbor(&CellCoordinates::new(2, 0))
            .unwrap()
            .is_live());

            // Case 2: bottom row
            assert!(Grid::new(vec![
                CellRow::new(vec![c('⬜'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
            ])
            .left_bottom_neighbor(&CellCoordinates::new(2, 1))
            .unwrap()
            .is_live());

            // Case 3: left column
            assert!(Grid::new(vec![
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬜')]),
            ])
            .left_bottom_neighbor(&CellCoordinates::new(1, 0))
            .unwrap()
            .is_live());
        }

        #[test]
        fn no_bottom_neighbor_case() {
            assert!(Grid::new(vec![
                CellRow::new(vec![c('⬜')]),
                CellRow::new(vec![c('⬛')]),
                CellRow::new(vec![c('⬛')]),
            ])
            .bottom_neighbor(&CellCoordinates::new(2, 0))
            .unwrap()
            .is_live());
        }

        #[test]
        fn no_right_bottom_neighbor_case() {
            // Case 1: right bottom corner
            assert!(Grid::new(vec![
                CellRow::new(vec![c('⬜'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
            ])
            .right_bottom_neighbor(&CellCoordinates::new(2, 2))
            .unwrap()
            .is_live());

            // Case 2: bottom row
            assert!(Grid::new(vec![
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬜')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
            ])
            .right_bottom_neighbor(&CellCoordinates::new(2, 1))
            .unwrap()
            .is_live());

            // Case 3: right column
            assert!(Grid::new(vec![
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬛'), c('⬛'), c('⬛')]),
                CellRow::new(vec![c('⬜'), c('⬛'), c('⬛')]),
            ])
            .right_bottom_neighbor(&CellCoordinates::new(1, 2))
            .unwrap()
            .is_live());
        }
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
