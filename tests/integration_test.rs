use std::{fs, time::Duration};

use game_of_life::{game::play, output::ConsoleLogger};

#[test]
fn golden_test_for_one_generation() {
    let generations = 1;
    let generation_duration = Duration::from_secs(0);
    let console_logger = ConsoleLogger::new();

    let final_state = play(generations, generation_duration, &console_logger);

    let expected_final_state = fs::read_to_string("./tests/fixtures/expected_output.txt")
        .expect("test should have a fixture with the final game output");

    assert_eq!(final_state, expected_final_state);
}
