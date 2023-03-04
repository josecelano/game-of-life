use std::env;

use crate::domain::output::{printer::Printer, CLEAR_SCREEN};


/// Capture input arguments from the environment
#[must_use]
pub fn arguments() -> Vec<String> {
    let args: Vec<String> = env::args().skip(1).collect();
    args
}

pub struct Console {}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

impl Console {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Printer for Console {
    fn clear(&self) {
        self.print(CLEAR_SCREEN);
    }

    fn print(&self, output: &str) {
        print!("{}", &output);
    }
}
