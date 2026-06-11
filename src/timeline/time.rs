#[derive(Debug, Default)]
pub struct Time {
    year: u64,
}

impl Time {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn year(&self) -> u64 {
        self.year
    }

    pub fn advance(&mut self) -> u64 {
        self.year = self.year.checked_add(1).expect("year counter overflowed");
        self.year
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advance_returns_and_stores_the_new_year() {
        let mut time = Time::new();

        assert_eq!(time.advance(), 1);
        assert_eq!(time.year(), 1);
    }

    #[test]
    #[should_panic(expected = "year counter overflowed")]
    fn advance_rejects_counter_overflow() {
        let mut time = Time { year: u64::MAX };

        time.advance();
    }
}
