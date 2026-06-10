use std::fmt::Display;

#[derive(Default)]
pub struct Time {
    year: u64,
}

impl Time {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn advance(&mut self) {
        self.year += 1;
    }

    pub fn print(&self) {
        println!("{self}");
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "云历{}年", self.year)
    }
}
