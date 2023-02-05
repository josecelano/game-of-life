use std::{thread, time::Duration};

use crate::domain::timer::Timer;

#[derive(Default)]
pub struct Sleeper {}

impl Timer for Sleeper {
    fn wait(&self, duration: Duration) {
        thread::sleep(duration);
    }
}
