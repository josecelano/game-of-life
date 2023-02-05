pub trait Printer {
    fn clear(&self);
    fn print(&self, output: &str);
}
