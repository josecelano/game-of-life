use std::time::Duration;

pub trait Timer {
    fn wait(&self, duration: Duration);
}
