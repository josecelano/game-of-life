use crate::{grid::Grid, output::Printer, settings::Settings};
use core::time::Duration;
use std::{env, fs, path::Path, process};
use text_colorizer::Colorize;

use crate::{game::play, grid::size::Size, output::Console};

const NUMBER_OF_ARGUMENTS: usize = 3;

#[derive(Debug)]
pub struct Arguments {
    pub pattern_file_path: String, // The path of the file containing the pattern
    pub rows: u16,                 // Number of rows for the background grid
    pub columns: u16,              // Number of columns for the background grid
}

pub fn run() {
    let args: Vec<String> = env::args().skip(1).collect();

    let args = parse_args(&args);

    let settings = Settings {
        generations: 1000,
        generation_duration: Duration::from_secs(1),
        back_grid_size: Size::new(args.rows as usize, args.columns as usize),
    };

    // Build pattern
    let text_pattern = fs::read_to_string(args.pattern_file_path)
        .expect("should have been able to read the file containing the pattern");
    let grid_pattern: Grid = text_pattern.parse().expect("invalid text pattern");

    let console = Console::new();

    let final_state = play(&settings, &grid_pattern, &console);

    console.print(&final_state);
}

fn print_usage() {
    eprintln!(
        "{} is an imaginary robot game (cellular automaton) made by the British mathematician John Horton Conway in 1970. 
        
cargo run PATTERN_FILE_PATH ROWS COLUMNS

PATTERN_FILE_PATH = The pattern file is a text file containing the pattern you want to use.
ROWS = Number of rows for the background grid
COLUMNS = Number of columns for the background grid
        
For example, for the Glider pattern:

 ⬛⬜⬛
 ⬛⬛⬜
 ⬜⬜⬜

with a 30x60 background grid, you should run this command:

cargo run ./patterns/glider.txt 30 60

",
        "The Game of Life".green()
    );
}

fn parse_args(args: &Vec<String>) -> Arguments {
    // Check number of mandatory arguments
    if args.len() != NUMBER_OF_ARGUMENTS {
        eprintln!(
            "{} wrong number of arguments: expected {}, got {}",
            "Error".red().bold(),
            NUMBER_OF_ARGUMENTS,
            args.len()
        );
        print_usage();
        process::exit(1);
    }

    // Check that the text file containing the pattern exists
    if !Path::new(&args[0]).exists() {
        eprintln!("{}: {}", "Pattern file not found".red(), &args[0].green(),);
        process::exit(1);
    }

    // Check rows
    let rows = args[1].parse::<u16>();
    if rows.is_err() {
        eprintln!(
            "{}: argument ROWS should be a positive integer, got {}.",
            "Invalid argument".red(),
            &args[1].green(),
        );
        process::exit(1);
    }

    // Check columns
    let columns = args[2].parse::<u16>();
    if columns.is_err() {
        eprintln!(
            "{}: argument COLUMNS should be a positive integer, got {}.",
            "Invalid argument".red(),
            &args[1].green()
        );
        process::exit(1);
    }

    Arguments {
        pattern_file_path: args[0].clone(),
        rows: rows.unwrap(),
        columns: columns.unwrap(),
    }
}
