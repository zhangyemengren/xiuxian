use crate::simulation::{Simulation, SimulationConfig};
use std::time::Duration;

pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) {
        let config = SimulationConfig::infinite(Duration::from_secs(1));
        let mut simulation = Simulation::new(config);
        simulation.run();
    }
}
