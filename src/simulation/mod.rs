use crate::time_line::{TimelineEvent, time::Time};
use std::{thread, time::Duration};

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

pub struct Simulation {
    config: SimulationConfig,
    time: Time,
}

impl Simulation {
    pub fn new(config: SimulationConfig) -> Self {
        Self {
            config,
            time: Time::new(),
        }
    }

    pub fn run(&mut self) {
        let mut completed_rounds = 0;

        loop {
            if self.config.should_stop(completed_rounds) {
                TimelineEvent::SimulationFinished {
                    rounds: completed_rounds,
                }
                .print();
                break;
            }

            let round = completed_rounds + 1;
            TimelineEvent::RoundStarted { round }.print();

            self.time.advance();
            self.time.print();

            TimelineEvent::RoundFinished { round }.print();
            completed_rounds = round;

            if !self.config.should_stop(completed_rounds) {
                thread::sleep(self.config.loop_interval);
            }
        }
    }
}
