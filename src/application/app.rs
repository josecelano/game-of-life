use core::time::Duration;
use std::{fs, path::Path, process};
use text_colorizer::Colorize;

use crate::{
    domain::{
        game::play,
        grid::{size::Size, Grid},
        settings::Settings,
    },
    infrastructure::{console::Console, thread::Sleeper},
};

const NUMBER_OF_ARGUMENTS: usize = 5;

#[derive(Debug)]
pub struct Arguments {
    pub pattern_file_path: String, // The path of the file containing the pattern
    pub rows: u32,                 // Number of rows for the background grid
    pub columns: u32,              // Number of columns for the background grid
    pub generations: u32,          // Number of generations to run the game
    pub generation_lifetime: u32,  // Lifetime for a generation in seconds
}

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

fn print_usage() {
    eprintln!(
        "{} is an imaginary robot game (cellular automaton) made by the British mathematician John Horton Conway in 1970. 
        
cargo run PATTERN_FILE_PATH ROWS COLUMNS GENERATIONS GENERATION_LIFETIME

PATTERN_FILE_PATH = The pattern file is a text file containing the pattern you want to use.
ROWS = Number of rows for the background grid
COLUMNS = Number of columns for the background grid
GENERATIONS = Number of generations to run the game
GENERATION_LIFETIME = Lifetime for a generation in seconds
        
For example, for the Glider pattern:

 ⬛⬜⬛
 ⬛⬛⬜
 ⬜⬜⬜

with a 30x60 background grid, you should run this command:

cargo run ./patterns/glider.txt 30 60 1000 1

It will runt for 1000 generations and each generation lives for 1 second.

",
        "The Game of Life".green()
    );
}

fn parse_args(args: &Vec<String>) -> Arguments {
    check_number_of_mandatory_params(args.len(), NUMBER_OF_ARGUMENTS);

    Arguments {
        pattern_file_path: parse_file_path("PATTERN_FILE_PATH", &args[0]),
        rows: parse_positive_integer("ROWS", &args[1]),
        columns: parse_positive_integer("COLUMNS", &args[2]),
        generations: parse_positive_integer("GENERATIONS", &args[3]),
        generation_lifetime: parse_positive_integer("GENERATION_LIFETIME", &args[4]),
    }
}

// todo:
// - Move parse arguments logic to new independent module.
// - Add tests for these functions.

fn check_number_of_mandatory_params(number_of_arguments: usize, expected_number: usize) {
    if number_of_arguments != expected_number {
        eprintln!(
            "{} wrong number of arguments: expected {}, got {}",
            "Invalid arguments:".red().bold(),
            NUMBER_OF_ARGUMENTS,
            number_of_arguments
        );
        print_usage();
        process::exit(1);
    }
}

fn parse_file_path(arg_name: &str, arg_value: &str) -> String {
    if !Path::new(arg_value).exists() {
        print_invalid_file_path_error(arg_name, arg_value);
        process::exit(1);
    }
    arg_value.to_owned()
}
fn parse_positive_integer(arg_name: &str, arg_value: &str) -> u32 {
    let result = arg_value.parse::<u32>();
    if result.is_err() {
        print_invalid_positive_integer_error(arg_name, arg_value);
        process::exit(1);
    }
    result.unwrap()
}

fn print_invalid_file_path_error(arg_name: &str, arg_value: &str) {
    eprintln!(
        "{}: argument {} should be a valid file path, got {}.",
        "File not found".red(),
        arg_name.green(),
        arg_value.green(),
    );
}
fn print_invalid_positive_integer_error(arg_name: &str, arg_value: &str) {
    eprintln!(
        "{}: argument {} should be a positive integer, got {}.",
        "Invalid argument".red(),
        arg_name.green(),
        arg_value.green(),
    );
}
