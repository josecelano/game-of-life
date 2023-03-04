use std::{path::Path, process};

use text_colorizer::Colorize;

use crate::ui::help::print_usage;

const NUMBER_OF_ARGUMENTS: usize = 5;

#[derive(Debug)]
pub struct Arguments {
    pub pattern_file_path: String, // The path of the file containing the pattern
    pub rows: u32,                 // Number of rows for the background grid
    pub columns: u32,              // Number of columns for the background grid
    pub generations: u32,          // Number of generations to run the game
    pub generation_lifetime: u32,  // Lifetime for a generation in seconds
}

#[must_use]
pub fn parse_args(args: &Vec<String>) -> Arguments {
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
