//! 应用启动流程。

use crate::simulation::{Simulation, SimulationConfig};
use std::time::Duration;

/// 使用默认配置启动模拟的应用入口。
#[derive(Default)]
pub struct App;

impl App {
    /// 启动默认的无限模拟。
    ///
    /// 每个回合间隔一秒，模拟会持续运行，直到进程被外部终止。
    pub fn run(&self) {
        let config = SimulationConfig::infinite(Duration::from_secs(1));
        let mut simulation = Simulation::new(config);
        simulation.run();
    }
}
