//! 模拟循环及其运行配置。

mod config;
mod result;
mod runner;

pub use config::SimulationConfig;
pub use result::{SimulationResult, StopReason};
pub use runner::Simulation;
