use crate::grid::size::Size;
use std::time::Duration;

pub struct Settings {
    pub generations: i64, // Number of generations to run the game
    pub generation_duration: Duration,
    pub back_grid_size: Size, // Background grid size
}
