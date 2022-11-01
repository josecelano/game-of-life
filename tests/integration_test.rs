use game_of_life::game::play;

#[test]
fn test_add() {
    assert_eq!(play(), "⬜⬜⬜\n⬜⬜⬜\n⬜⬜⬜\n");
}
