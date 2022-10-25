use crate::grid::Grid;
use std::fmt::Write;

pub fn print_grid(grid: &Grid) {
    println!("{}", format_grid(grid));
}

pub fn format_grid(grid: &Grid) -> String {
    let mut output = String::new();
    for cell in &grid.cells {
        let cell_char = match cell.is_live() {
            true => "⬜".to_string(),
            false => "⬛".to_string(),
        };
        write!(&mut output, "{}", cell_char).unwrap();
    }
    output.to_string()
}

#[cfg(test)]
mod tests {
    use crate::{cell::Cell, grid::Grid, grid_printer::format_grid};

    #[test]
    fn it_should_print_an_empty_grid() {
        let grid = Grid::default();

        assert_eq!(format_grid(&grid), "");
    }

    #[test]
    fn it_should_print_a_grid_with_only_one_live_cell() {
        let grid = Grid::new(1, 1);

        assert_eq!(format_grid(&grid), "⬜");
    }

    #[test]
    fn it_should_print_a_grid_with_only_one_dead_cell() {
        let grid = Grid {
            rows: 1,
            columns: 1,
            cells: vec![Cell::dead()],
        };

        assert_eq!(format_grid(&grid), "⬛");
    }
}
