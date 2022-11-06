use std::{fs, time::Duration};

use game_of_life::game::play;

#[test]
fn test_add() {
    let generations = 1;
    let generation_duration = Duration::from_secs(0);

    let expected_output = fs::read_to_string("./tests/fixtures/expected_output.txt")
        .expect("Missing snapshot of output in golden test");

    assert_eq!(play(generations, generation_duration), expected_output);
}
