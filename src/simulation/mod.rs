use crate::timeline::{TimelineEvent, time::Time};
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

#[derive(Clone, Copy, Debug)]
pub struct SimulationConfig {
    max_rounds: Option<u64>,
    loop_interval: Duration,
}

impl SimulationConfig {
    pub fn infinite(loop_interval: Duration) -> Self {
        Self {
            max_rounds: None,
            loop_interval,
        }
    }

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StopReason {
    MaxRoundsReached,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SimulationResult {
    pub completed_rounds: u64,
    pub stop_reason: StopReason,
}

pub struct Simulation {
    config: SimulationConfig,
    time: Time,
    completed_rounds: u64,
}

impl Simulation {
    pub fn new(config: SimulationConfig) -> Self {
        Self {
            config,
            time: Time::new(),
            completed_rounds: 0,
        }
    }

    pub fn completed_rounds(&self) -> u64 {
        self.completed_rounds
    }

    pub fn current_year(&self) -> u64 {
        self.time.year()
    }

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

    pub fn run(&mut self) -> io::Result<SimulationResult> {
        let stdout = io::stdout();
        let mut output = stdout.lock();
        self.run_with(&mut output, thread::sleep)
    }

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
