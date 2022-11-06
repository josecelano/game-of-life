use core::time::Duration;
use game_of_life::game::play;

fn main() {
    let generations = 10000;
    let generation_duration = Duration::from_secs(2);

    print!("{}", play(generations, generation_duration));
}
