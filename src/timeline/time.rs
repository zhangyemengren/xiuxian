//! 世界时间状态。

/// 以年份记录的世界时间。
///
/// 新创建的时间从第 `0` 年开始，第一次推进后进入第 `1` 年。
#[derive(Debug, Default)]
pub struct Time {
    year: u64,
}

impl Time {
    /// 创建初始年份为 `0` 的世界时间。
    pub fn new() -> Self {
        Self::default()
    }

    /// 返回当前年份。
    pub fn year(&self) -> u64 {
        self.year
    }

    /// 将世界时间推进一年并返回推进后的年份。
    ///
    /// # Panics
    ///
    /// 当当前年份为 [`u64::MAX`]、无法继续推进时 panic。
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
