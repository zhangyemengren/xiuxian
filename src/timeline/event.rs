//! 可输出的模拟时间线事件。

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
    /// 将事件信息打印到标准输出。
    pub fn display(&self) {
        match self {
            Self::RoundStarted { round } => println!("第{round}回合开始"),
            Self::TimeAdvanced { year } => println!("云历{year}年"),
            Self::RoundFinished { round } => println!("第{round}回合结束"),
            Self::SimulationFinished { rounds } => println!("模拟结束，共执行{rounds}回合"),
        }
    }
}
