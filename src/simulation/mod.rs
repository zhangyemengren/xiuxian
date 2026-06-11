//! 模拟循环及其运行配置。

use crate::timeline::{TimelineEvent, time::Time};
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

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

    fn should_stop(&self, completed_rounds: u64) -> bool {
        self.max_rounds
            .is_some_and(|max_rounds| completed_rounds >= max_rounds)
    }
}

/// 模拟正常结束的原因。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StopReason {
    /// 已完成配置指定的最大回合数。
    MaxRoundsReached,
}

/// 一次模拟正常结束后的汇总结果。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SimulationResult {
    /// 模拟实际完成的回合数。
    pub completed_rounds: u64,
    /// 模拟正常结束的原因。
    pub stop_reason: StopReason,
}

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

    /// 使用标准输出和当前线程休眠执行模拟。
    ///
    /// 有限模拟达到最大回合数后返回运行结果。无限模拟不会正常返回，
    /// 只能由外部终止进程，或因输出失败而退出。
    ///
    /// # Errors
    ///
    /// 当时间线事件无法写入标准输出时，返回对应的 I/O 错误。
    ///
    /// # Panics
    ///
    /// 当回合计数或世界年份超过 [`u64::MAX`] 时 panic。
    pub fn run(&mut self) -> io::Result<SimulationResult> {
        let stdout = io::stdout();
        let mut output = stdout.lock();
        self.run_with(&mut output, thread::sleep)
    }

    /// 使用指定输出目标和等待函数执行模拟。
    ///
    /// `output` 接收每个时间线事件，每个事件占一行。`sleep` 在两个
    /// 连续回合之间调用，最后一个有限回合结束后不会调用。
    ///
    /// 有限模拟达到最大回合数后返回运行结果。无限模拟不会正常返回，
    /// 除非写入输出目标失败。
    ///
    /// # Errors
    ///
    /// 当时间线事件无法写入 `output` 时，返回对应的 I/O 错误。
    ///
    /// # Panics
    ///
    /// 当回合计数或世界年份超过 [`u64::MAX`] 时 panic。等待函数自身
    /// 产生的 panic 也会向调用方传播。
    pub fn run_with<W, S>(&mut self, output: &mut W, mut sleep: S) -> io::Result<SimulationResult>
    where
        W: Write,
        S: FnMut(Duration),
    {
        loop {
            if self.config.should_stop(self.completed_rounds) {
                TimelineEvent::SimulationFinished {
                    rounds: self.completed_rounds,
                }
                .write_to(output)?;

                return Ok(SimulationResult {
                    completed_rounds: self.completed_rounds,
                    stop_reason: StopReason::MaxRoundsReached,
                });
            }

            for event in self.step() {
                event.write_to(output)?;
            }

            if !self.config.should_stop(self.completed_rounds) {
                sleep(self.config.loop_interval);
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
        let mut output = Vec::new();
        let mut sleeps = Vec::new();

        let result = simulation
            .run_with(&mut output, |duration| sleeps.push(duration))
            .expect("simulation should run");

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
        assert_eq!(
            String::from_utf8(output).expect("output should be UTF-8"),
            concat!(
                "第1回合开始\n",
                "云历1年\n",
                "第1回合结束\n",
                "第2回合开始\n",
                "云历2年\n",
                "第2回合结束\n",
                "第3回合开始\n",
                "云历3年\n",
                "第3回合结束\n",
                "模拟结束，共执行3回合\n",
            )
        );
    }

    #[test]
    fn zero_round_limit_finishes_without_running_or_sleeping() {
        let config = SimulationConfig::limited(0, Duration::from_secs(1));
        let mut simulation = Simulation::new(config);
        let mut output = Vec::new();
        let mut sleep_count = 0;

        let result = simulation
            .run_with(&mut output, |_| sleep_count += 1)
            .expect("simulation should run");

        assert_eq!(result.completed_rounds, 0);
        assert_eq!(simulation.current_year(), 0);
        assert_eq!(sleep_count, 0);
        assert_eq!(
            String::from_utf8(output).expect("output should be UTF-8"),
            "模拟结束，共执行0回合\n"
        );
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
