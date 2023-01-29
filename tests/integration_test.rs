use std::{fs, time::Duration};

use game_of_life::{
    game::play,
    grid::{size::Size, Grid},
    output::ConsoleLogger,
};

#[test]
fn golden_test_for_one_generation() {
    let generations = 1;
    let generation_duration = Duration::from_secs(0);
    let back_grid_size = Size::new(30, 60);
    let text_pattern = fs::read_to_string("./tests/fixtures/glider.txt")
        .expect("should have been able to read the file containing the pattern");
    let grid_pattern: Grid = text_pattern.parse().expect("invalid text pattern");

    let console_logger = ConsoleLogger::new();

    let final_state = play(
        generations,
        generation_duration,
        &back_grid_size,
        &grid_pattern,
        &console_logger,
    );

    let expected_final_state = fs::read_to_string("./tests/fixtures/expected_output.txt")
        .expect("test should have a fixture with the final game output");

    assert_eq!(final_state, expected_final_state);
}
