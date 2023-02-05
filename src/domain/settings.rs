use std::time::Duration;

use super::grid::size::Size;

pub struct Settings {
    pub generations: i64, // Number of generations to run the game
    pub generation_duration: Duration,
    pub back_grid_size: Size, // Background grid size
}
