use std::fmt::Display;

#[derive(Default)]
pub struct Time{
    pub time: String,
}

impl Time {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "云历{}年", self.time)
    }
}