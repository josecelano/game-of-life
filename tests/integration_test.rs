use std::fs;

use game_of_life::game::play;

#[test]
fn test_add() {
    let expected_output = fs::read_to_string("./tests/fixtures/expected_output.txt")
        .expect("Missing snapshot of output in golden test");

    assert_eq!(play(), expected_output);
}
