use core::time::Duration;
use std::fs;

use crate::{
    domain::{
        game::play,
        grid::{size::Size, Grid},
        settings::Settings,
    },
    infrastructure::{console::Console, thread::Sleeper},
    ui::console::{parse_args, Arguments},
};

// todo: add unit tests for `app::run`.

#[must_use]
pub fn run(args: &Vec<String>) -> String {
    let args = parse_args(args);

    play(
        &setup_settings(&args),
        &build_pattern(&args.pattern_file_path),
        &Console::new(),
        &Sleeper::default(),
    )
}

fn setup_settings(args: &Arguments) -> Settings {
    Settings {
        back_grid_size: Size::new(args.rows as usize, args.columns as usize),
        generations: args.generations,
        generation_lifetime: Duration::from_secs(args.generation_lifetime.into()),
    }
}

fn build_pattern(pattern_file_path: &str) -> Grid {
    let text_pattern = fs::read_to_string(pattern_file_path)
        .expect("should have been able to read the file containing the pattern");
    text_pattern.parse().expect("invalid text pattern")
}
