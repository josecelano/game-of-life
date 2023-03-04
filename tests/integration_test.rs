use std::fs;

use game_of_life::application::app;

#[test]
fn golden_test_for_one_generation() {
    const PATTERN: &str = "./patterns/glider.txt";
    const ROWS: &str = "30";
    const COLUMNS: &str = "60";
    const GENERATIONS: &str = "1";
    const GENERATION_LIFETIME: &str = "0";

    let args = [
        PATTERN.to_string(),
        ROWS.to_string(),
        COLUMNS.to_string(),
        GENERATIONS.to_string(),
        GENERATION_LIFETIME.to_string(),
    ];

    let final_state = app::run(&args);

    let expected_final_state = fs::read_to_string("./tests/fixtures/expected_output.txt")
        .expect("test should have a fixture with the final game output");

    assert_eq!(final_state, expected_final_state);
}
