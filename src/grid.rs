use crate::cell::Cell;

pub struct Grid {
    pub rows: u32,
    pub columns: u32,
    pub cells: Vec<Cell>,
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Grid {
    pub fn new(rows: u32, columns: u32) -> Self {
        Self {
            rows,
            columns,
            cells: vec![Cell::live(); (rows * columns) as usize],
        }
    }

    pub fn rows(&self) -> u32 {
        self.rows
    }

    pub fn columns(&self) -> u32 {
        self.columns
    }

    pub fn get_cell(&self, row: u32, column: u32) -> &Cell {
        let cell_pos = row * self.columns + column;
        &self.cells[cell_pos as usize]
    }
}

#[cfg(test)]
mod tests {
    use crate::{cell::Cell, grid::Grid};

    #[test]
    fn a_grid_has_two_dimensions() {
        let grid = Grid::default();

        assert_eq!(grid.rows(), 0);
        assert_eq!(grid.columns(), 0);
    }

    #[test]
    fn a_grid_contains_a_cell_in_each_position() {
        let grid = Grid {
            rows: 1,
            columns: 1,
            cells: vec![Cell::live()],
        };

        assert!(grid.get_cell(0, 0).is_live());
    }

    // todo: 
    // a_grid_should_panic_when_accessing_an_invalid_position
    // the_default_grid_does_not_contain_any_cell

    // notas:
    // si permito crearlo dinámicamente el tamaño no tengo que permitir manipular
    // rows y columns o el vector directamente una vez creado.
    // Podría extraer eso a una matriz peo por ahora lo dejo en el grid.
    // EL constructor acepta el vector de cualquier tamaño y lanzo exception
    // si no coincidedn los datos.
}
