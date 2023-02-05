use game_of_life::domain::timer::Timer;
use std::{fs, time::Duration};

use game_of_life::{
    domain::game::play,
    domain::grid::{size::Size, Grid},
    domain::output::logger::Logger,
    domain::settings::Settings,
};

pub struct NoWait {}

impl Timer for NoWait {
    fn wait(&self, _duration: Duration) {
        // No wait fo testing
    }
}

#[test]
fn golden_test_for_one_generation() {
    let settings = Settings {
        generations: 1,
        generation_duration: Duration::from_secs(0),
        back_grid_size: Size::new(30, 60),
    };

    let text_pattern = fs::read_to_string("./tests/fixtures/glider.txt")
        .expect("should have been able to read the file containing the pattern");
    let grid_pattern: Grid = text_pattern.parse().expect("invalid text pattern");

    let output_logger = Logger::new();
    let timer = NoWait {};

    let final_state = play(&settings, &grid_pattern, &output_logger, &timer);

    let expected_final_state = fs::read_to_string("./tests/fixtures/expected_output.txt")
        .expect("test should have a fixture with the final game output");

    assert_eq!(final_state, expected_final_state);
}
