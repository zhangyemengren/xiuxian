//! 应用启动流程。

use crate::simulation::{Simulation, SimulationConfig};
use std::{io, time::Duration};

/// 使用默认配置启动模拟的应用入口。
#[derive(Default)]
pub struct App;

impl App {
    /// 启动默认的无限模拟。
    ///
    /// 每个回合间隔一秒，模拟会持续运行，直到进程被外部终止或输出失败。
    ///
    /// # Errors
    ///
    /// 当模拟日志无法写入标准输出时，返回对应的 I/O 错误。
    pub fn run(&self) -> io::Result<()> {
        let config = SimulationConfig::infinite(Duration::from_secs(1));
        let mut simulation = Simulation::new(config);
        simulation.run()?;
        Ok(())
    }
}
