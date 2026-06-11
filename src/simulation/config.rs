//! 模拟运行配置。

use std::time::Duration;

/// 控制模拟停止条件和回合间隔的配置。
#[derive(Clone, Copy, Debug)]
pub struct SimulationConfig {
    max_rounds: Option<u64>,
    loop_interval: Duration,
}

impl SimulationConfig {
    /// 创建没有自动停止条件的配置。
    ///
    /// `loop_interval` 指定两个连续回合之间的等待时间。
    pub fn infinite(loop_interval: Duration) -> Self {
        Self {
            max_rounds: None,
            loop_interval,
        }
    }

    /// 创建达到指定回合数后停止的配置。
    ///
    /// `max_rounds` 为 `0` 时，模拟不会执行任何回合。
    /// `loop_interval` 指定两个连续回合之间的等待时间。
    pub fn limited(max_rounds: u64, loop_interval: Duration) -> Self {
        Self {
            max_rounds: Some(max_rounds),
            loop_interval,
        }
    }

    pub(super) fn should_stop(&self, completed_rounds: u64) -> bool {
        self.max_rounds
            .is_some_and(|max_rounds| completed_rounds >= max_rounds)
    }

    pub(super) fn loop_interval(&self) -> Duration {
        self.loop_interval
    }
}
