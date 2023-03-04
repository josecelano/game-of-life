use std::time::Duration;

use super::grid::size::Size;

pub struct Settings {
    pub back_grid_size: Size,          // Background grid size
    pub generations: u32,              // Number of generations to run the game
    pub generation_lifetime: Duration, // Lifetime for a generation
}
