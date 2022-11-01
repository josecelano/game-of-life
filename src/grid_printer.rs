use crate::{cell_position::CellPosition, grid::Grid};
use std::fmt::Write;

pub fn print_grid(grid: &Grid) {
    println!("{}", render_grid(grid));
}

pub fn render_grid(grid: &Grid) -> String {
    let mut output = String::new();
    for row in 0..grid.rows() {
        for column in 0..grid.columns() {
            let cell_char = match grid.get_cell(CellPosition::new(row, column)).is_live() {
                true => "⬜".to_string(),
                false => "⬛".to_string(),
            };
            write!(&mut output, "{}", cell_char).unwrap();
        }
        writeln!(&mut output).unwrap();
    }
    output.to_string()
}

#[cfg(test)]
mod tests {
    use crate::{cell::Cell, cell_row::CellRow, grid::Grid, grid_printer::render_grid};

    #[test]
    fn it_should_print_an_empty_grid() {
        let grid = Grid::default();

        assert_eq!(render_grid(&grid), "");
    }

    #[test]
    fn it_should_print_a_grid_with_only_one_live_cell() {
        let grid = Grid::new(vec![CellRow::new(vec![Cell::live()])]);

        assert_eq!(render_grid(&grid), "⬜\n");
    }

    #[test]
    fn it_should_print_a_grid_with_only_one_dead_cell() {
        let grid = Grid::new(vec![CellRow::new(vec![Cell::dead()])]);

        assert_eq!(render_grid(&grid), "⬛\n");
    }

    #[test]
    fn it_should_print_a_3x3_grid() {
        let grid = Grid::new(vec![
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
            CellRow::new(vec![Cell::dead(), Cell::dead(), Cell::dead()]),
            CellRow::new(vec![Cell::live(), Cell::live(), Cell::live()]),
        ]);

        assert_eq!(render_grid(&grid), "⬜⬜⬜\n⬛⬛⬛\n⬜⬜⬜\n");
    }
}
