use std::fmt::Display;

pub enum TimelineEvent {
    RoundStarted { round: u64 },
    RoundFinished { round: u64 },
    SimulationFinished { rounds: u64 },
}

impl TimelineEvent {
    pub fn print(&self) {
        println!("{self}");
    }
}

impl Display for TimelineEvent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RoundStarted { round } => write!(formatter, "第{round}回合开始"),
            Self::RoundFinished { round } => write!(formatter, "第{round}回合结束"),
            Self::SimulationFinished { rounds } => {
                write!(formatter, "模拟结束，共执行{rounds}回合")
            }
        }
    }
}
