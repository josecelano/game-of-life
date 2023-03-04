use game_of_life::{application::app, infrastructure::console};

fn main() {
    let _final_state = app::run(&console::arguments());
}
