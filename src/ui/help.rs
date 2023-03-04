use text_colorizer::Colorize;

pub fn print_usage() {
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
