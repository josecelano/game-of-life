use game_of_life::application::app;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let _final_state = app::run(&args);
}
