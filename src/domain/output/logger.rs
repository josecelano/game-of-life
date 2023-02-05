use std::cell::RefCell;

use super::{printer::Printer, CLEAR_SCREEN};

pub struct Logger {
    output: RefCell<String>,
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

impl Logger {
    #[must_use]
    pub fn new() -> Self {
        Self {
            output: RefCell::new(String::new()),
        }
    }

    pub fn log(&self) -> String {
        self.output.borrow().clone()
    }
}

impl Printer for Logger {
    fn clear(&self) {
        self.print(CLEAR_SCREEN);
    }

    fn print(&self, output: &str) {
        *self.output.borrow_mut() = format!("{}{}", self.output.borrow(), &output);
    }
}

#[cfg(test)]
mod tests {

    use crate::domain::output::{logger::Logger, printer::Printer, CLEAR_SCREEN};

    #[test]
    fn should_capture_the_clear_screen_command() {
        let console_logger = Logger::new();

        console_logger.clear();

        assert_eq!(CLEAR_SCREEN, console_logger.log());
    }

    #[test]
    fn should_capture_the_print_command_output() {
        let console_logger = Logger::new();

        console_logger.print("OUTPUT");

        assert_eq!("OUTPUT", console_logger.log());
    }
}
