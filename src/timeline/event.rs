//! 可输出的模拟时间线事件。

use std::{
    fmt::Display,
    io::{self, Write},
};

/// 模拟循环中按顺序产生的时间线事件。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimelineEvent {
    /// 一个新回合已经开始。
    RoundStarted {
        /// 从 `1` 开始的回合编号。
        round: u64,
    },
    /// 世界时间已经推进到指定年份。
    TimeAdvanced {
        /// 推进后的世界年份。
        year: u64,
    },
    /// 当前回合已经结束。
    RoundFinished {
        /// 已结束的回合编号。
        round: u64,
    },
    /// 模拟已经正常结束。
    SimulationFinished {
        /// 模拟实际完成的总回合数。
        rounds: u64,
    },
}

impl TimelineEvent {
    /// 将事件的显示文本及换行符写入目标。
    ///
    /// # Errors
    ///
    /// 当目标无法接受完整事件文本时，返回对应的 I/O 错误。
    pub fn write_to(&self, output: &mut impl Write) -> io::Result<()> {
        writeln!(output, "{self}")
    }
}

impl Display for TimelineEvent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RoundStarted { round } => write!(formatter, "第{round}回合开始"),
            Self::TimeAdvanced { year } => write!(formatter, "云历{year}年"),
            Self::RoundFinished { round } => write!(formatter, "第{round}回合结束"),
            Self::SimulationFinished { rounds } => {
                write!(formatter, "模拟结束，共执行{rounds}回合")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn events_have_expected_display_text() {
        let cases = [
            (TimelineEvent::RoundStarted { round: 2 }, "第2回合开始"),
            (TimelineEvent::TimeAdvanced { year: 8 }, "云历8年"),
            (TimelineEvent::RoundFinished { round: 2 }, "第2回合结束"),
            (
                TimelineEvent::SimulationFinished { rounds: 2 },
                "模拟结束，共执行2回合",
            ),
        ];

        for (event, expected) in cases {
            assert_eq!(event.to_string(), expected);
        }
    }

    #[test]
    fn write_to_appends_a_newline() {
        let mut output = Vec::new();

        TimelineEvent::TimeAdvanced { year: 1 }
            .write_to(&mut output)
            .expect("event should be written");

        assert_eq!(output, "云历1年\n".as_bytes());
    }
}
