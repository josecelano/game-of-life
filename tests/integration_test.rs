use std::fs;

use game_of_life::application::app;

#[test]
fn golden_master_test() {
    // Snapshot test for output after one generation

    const PATTERN: &str = "./patterns/glider.txt";
    const ROWS: &str = "30";
    const COLUMNS: &str = "60";
    const GENERATIONS: &str = "1";
    const GENERATION_LIFETIME: &str = "0";

    let command =
        format!("cargo run {PATTERN} {ROWS} {COLUMNS} {GENERATIONS} {GENERATION_LIFETIME}");

    let final_state = app::run(&extract_args(&command));

    let expected_final_state = fs::read_to_string("./tests/fixtures/expected_output.txt")
        .expect("test should have a fixture with the final game output");

    assert_eq!(final_state, expected_final_state);
}

/// Extract command arguments into a string vector
fn extract_args(command: &str) -> Vec<String> {
    command
        .split(' ')
        .skip(2)
        .map(std::borrow::ToOwned::to_owned)
        .collect()
}
