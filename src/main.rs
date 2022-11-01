use game_of_life::game::play;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    for iter in 1..100000 {
        clear_screen();
        print!("{}", play());
        println!("Iter:{}", iter);
    }
}
