use std::{
    fmt::Display,
    io::{self, Write},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimelineEvent {
    RoundStarted { round: u64 },
    TimeAdvanced { year: u64 },
    RoundFinished { round: u64 },
    SimulationFinished { rounds: u64 },
}

impl TimelineEvent {
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
