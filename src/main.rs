pub mod cell;
pub mod grid;
pub mod output;

use core::time::Duration;

use game_of_life::game::play;
use game_of_life::output::Console;

fn main() {
    let generations = 10000;
    let generation_duration = Duration::from_secs(1);
    let console = Console::new();

    print!("{}", play(generations, generation_duration, &console));
}
