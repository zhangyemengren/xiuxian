use crate::simulation::{Simulation, SimulationConfig};
use std::{io, time::Duration};

#[derive(Default)]
pub struct App;

impl App {
    pub fn run(&self) -> io::Result<()> {
        let config = SimulationConfig::infinite(Duration::from_secs(1));
        let mut simulation = Simulation::new(config);
        simulation.run()?;
        Ok(())
    }
}
