use std::cell::RefCell;

pub const CLEAR_SCREEN: &str = "\x1B[2J\x1B[1;1H";

pub trait Printer {
    fn clear(&self);
    fn print(&self, output: &str);
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

pub struct ConsoleLogger {
    output: RefCell<String>,
}

impl Default for ConsoleLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl ConsoleLogger {
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

impl Printer for ConsoleLogger {
    fn clear(&self) {
        self.print(CLEAR_SCREEN);
    }

    fn print(&self, output: &str) {
        *self.output.borrow_mut() = format!("{}{}", self.output.borrow(), &output);
    }
}

#[cfg(test)]
mod tests {

    mod console_logger {
        use crate::output::{ConsoleLogger, Printer, CLEAR_SCREEN};

        #[test]
        fn should_capture_the_clear_screen_command() {
            let console_logger = ConsoleLogger::new();

            console_logger.clear();

            assert_eq!(CLEAR_SCREEN, console_logger.log());
        }

        #[test]
        fn should_capture_the_print_command_output() {
            let console_logger = ConsoleLogger::new();

            console_logger.print("OUTPUT");

            assert_eq!("OUTPUT", console_logger.log());
        }
    }
}
