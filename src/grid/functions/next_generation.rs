/// Calculate the next generation of cells for the grid
use crate::{
    cell::{coordinates::Coordinates, row::Row, state::State, Cell},
    grid::{CellInfo, Grid},
};

pub fn next_generation(grid: &Grid) -> Grid {
    let mut cell_rows = vec![];

    for row in 0..grid.rows() {
        let mut cells_row = vec![];
        for column in 0..grid.columns() {
            cells_row.push(new_cell_applying_rule_b3_s23(
                &grid.get_cell_info(&Coordinates::new(row, column)),
            ));
        }
        cell_rows.push(Row::new(cells_row))
    }

    Grid::new(cell_rows)
}

/// Game of Life standard.
///
/// Rule-string notation: B3/S23
///
/// A cell:
/// - is born if it has exactly three neighbours,
/// - survives if it has two or three living neighbours,
/// and dies otherwise.
fn new_cell_applying_rule_b3_s23(cell_info: &CellInfo) -> Cell {
    match cell_info.state {
        State::Live => match cell_info.number_of_live_neighbors {
            2 => Cell::live(),
            3 => Cell::live(),
            _ => Cell::dead(),
        },
        State::Dead => match cell_info.number_of_live_neighbors {
            3 => Cell::live(),
            _ => Cell::dead(),
        },
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        cell::coordinates::Coordinates, grid::functions::next_generation::next_generation,
        grid::Grid,
    };

    #[test]
    fn the_next_generation_of_cells_in_an_empty_grid_is_an_empty_grid() {
        let grid = Grid::new_empty();

        assert_eq!(next_generation(&grid), Grid::new_empty());
    }

    #[test]
    fn any_live_cell_with_fewer_than_two_live_neighbours_dies() {
        let grid = Grid::from_str(
            "⬛⬛⬛
             ⬛⬜⬛
             ⬛⬛⬛",
        )
        .unwrap();

        assert!(next_generation(&grid)
            .get_cell(&Coordinates::new(1, 1))
            .is_dead());

        let grid = Grid::from_str(
            "⬛⬜⬛
             ⬛⬜⬛
             ⬛⬛⬛",
        )
        .unwrap();

        assert!(next_generation(&grid)
            .get_cell(&Coordinates::new(1, 1))
            .is_dead());
    }

    #[test]
    fn any_live_cell_with_two_or_three_live_neighbours_survives() {
        let grid = Grid::from_str(
            "⬛⬛⬛
             ⬜⬜⬜
             ⬛⬛⬛",
        )
        .unwrap();

        assert!(next_generation(&grid)
            .get_cell(&Coordinates::new(1, 1))
            .is_live());

        let grid = Grid::from_str(
            "⬛⬜⬛
             ⬜⬜⬜
             ⬛⬛⬛",
        )
        .unwrap();

        assert!(next_generation(&grid)
            .get_cell(&Coordinates::new(1, 1))
            .is_live());
    }

    #[test]
    fn any_live_cell_with_more_than_three_live_neighbours_dies() {
        let grid = Grid::from_str(
            "⬛⬜⬛
             ⬜⬜⬜
             ⬛⬜⬛",
        )
        .unwrap();

        assert!(next_generation(&grid)
            .get_cell(&Coordinates::new(1, 1))
            .is_dead());

        let grid = Grid::from_str(
            "⬜⬜⬜
             ⬜⬜⬜
             ⬜⬜⬜",
        )
        .unwrap();

        assert!(next_generation(&grid)
            .get_cell(&Coordinates::new(1, 1))
            .is_dead());
    }

    #[test]
    fn any_dead_cell_with_exactly_three_live_neighbours_becomes_a_live_cell() {
        let grid = Grid::from_str(
            "⬛⬛⬛
             ⬜⬛⬜
             ⬛⬜⬛",
        )
        .unwrap();

        assert!(next_generation(&grid)
            .get_cell(&Coordinates::new(1, 1))
            .is_live());

        let grid = Grid::from_str(
            "⬛⬜⬛
             ⬜⬛⬜
             ⬛⬜⬛",
        )
        .unwrap();

        assert!(next_generation(&grid)
            .get_cell(&Coordinates::new(1, 1))
            .is_dead());
    }

    #[test]
    fn grid_edges_are_stitched_together() {
        let grid = Grid::from_str(
            "⬛⬛⬛⬛⬛
             ⬛⬛⬛⬛⬛
             ⬛⬜⬜⬜⬛",
        )
        .unwrap();

        let expected_grid = Grid::from_str(
            "⬛⬛⬜⬛⬛
             ⬛⬛⬜⬛⬛
             ⬛⬛⬜⬛⬛",
        )
        .unwrap();

        let actual_grid = next_generation(&grid);

        assert_eq!(actual_grid, expected_grid);
    }
}
