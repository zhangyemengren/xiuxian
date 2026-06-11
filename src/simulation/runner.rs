//! 模拟运行器。

use super::{
    config::SimulationConfig,
    result::{SimulationResult, StopReason},
};
use crate::timeline::{Time, TimelineEvent};
use std::{thread, time::Duration};

/// 持有模拟配置、世界时间和回合进度。
pub struct Simulation {
    config: SimulationConfig,
    time: Time,
    completed_rounds: u64,
}

impl Simulation {
    /// 使用给定配置创建一个尚未执行回合的模拟。
    pub fn new(config: SimulationConfig) -> Self {
        Self {
            config,
            time: Time::new(),
            completed_rounds: 0,
        }
    }

    /// 返回已经完整推进的回合数。
    pub fn completed_rounds(&self) -> u64 {
        self.completed_rounds
    }

    /// 返回当前世界年份。
    pub fn current_year(&self) -> u64 {
        self.time.year()
    }

    /// 推进一个回合并返回按发生顺序排列的时间线事件。
    ///
    /// 此方法只负责推进状态，不检查配置中的停止条件，也不输出事件。
    ///
    /// # Panics
    ///
    /// 当回合计数或世界年份超过 [`u64::MAX`] 时 panic。
    pub fn step(&mut self) -> [TimelineEvent; 3] {
        let round = self
            .completed_rounds
            .checked_add(1)
            .expect("round counter overflowed");
        let year = self.time.advance();
        self.completed_rounds = round;

        [
            TimelineEvent::RoundStarted { round },
            TimelineEvent::TimeAdvanced { year },
            TimelineEvent::RoundFinished { round },
        ]
    }

    /// 使用当前线程休眠执行模拟。
    ///
    /// 产生的时间线事件会直接打印到标准输出。有限模拟达到最大回合数后
    /// 返回运行结果。无限模拟不会正常返回，只能由外部终止进程。
    ///
    /// # Panics
    ///
    /// 当回合计数或世界年份超过 [`u64::MAX`] 时 panic。
    pub fn run(&mut self) -> SimulationResult {
        self.run_with(thread::sleep)
    }

    /// 使用指定等待函数执行模拟。
    ///
    /// 每个时间线事件直接打印到标准输出。`sleep` 在两个连续回合之间
    /// 调用，最后一个有限回合结束后不会调用。
    ///
    /// 有限模拟达到最大回合数后返回运行结果。无限模拟不会正常返回。
    ///
    /// # Panics
    ///
    /// 当回合计数或世界年份超过 [`u64::MAX`] 时 panic。等待函数自身
    /// 产生的 panic 也会向调用方传播。
    pub fn run_with<S>(&mut self, mut sleep: S) -> SimulationResult
    where
        S: FnMut(Duration),
    {
        loop {
            if self.config.should_stop(self.completed_rounds) {
                TimelineEvent::SimulationFinished {
                    rounds: self.completed_rounds,
                }
                .display();

                return SimulationResult {
                    completed_rounds: self.completed_rounds,
                    stop_reason: StopReason::MaxRoundsReached,
                };
            }

            for event in self.step() {
                event.display();
            }

            if !self.config.should_stop(self.completed_rounds) {
                sleep(self.config.loop_interval());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limited_simulation_runs_expected_rounds_and_reports_result() {
        let interval = Duration::from_millis(25);
        let config = SimulationConfig::limited(3, interval);
        let mut simulation = Simulation::new(config);
        let mut sleeps = Vec::new();

        let result = simulation.run_with(|duration| sleeps.push(duration));

        assert_eq!(
            result,
            SimulationResult {
                completed_rounds: 3,
                stop_reason: StopReason::MaxRoundsReached,
            }
        );
        assert_eq!(simulation.completed_rounds(), 3);
        assert_eq!(simulation.current_year(), 3);
        assert_eq!(sleeps, vec![interval, interval]);
    }

    #[test]
    fn zero_round_limit_finishes_without_running_or_sleeping() {
        let config = SimulationConfig::limited(0, Duration::from_secs(1));
        let mut simulation = Simulation::new(config);
        let mut sleep_count = 0;

        let result = simulation.run_with(|_| sleep_count += 1);

        assert_eq!(result.completed_rounds, 0);
        assert_eq!(simulation.current_year(), 0);
        assert_eq!(sleep_count, 0);
    }

    #[test]
    fn step_advances_state_and_returns_ordered_events() {
        let config = SimulationConfig::infinite(Duration::ZERO);
        let mut simulation = Simulation::new(config);

        assert_eq!(
            simulation.step(),
            [
                TimelineEvent::RoundStarted { round: 1 },
                TimelineEvent::TimeAdvanced { year: 1 },
                TimelineEvent::RoundFinished { round: 1 },
            ]
        );
        assert_eq!(simulation.completed_rounds(), 1);
        assert_eq!(simulation.current_year(), 1);
    }
}
