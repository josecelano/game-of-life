use game_of_life::game::play;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    clear_screen();
    print!("{}", play());
}
