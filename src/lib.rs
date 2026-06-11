//! 自动修仙模拟器的核心库。
//!
//! 本库负责组织应用启动、模拟循环、时间线事件和世界状态。

#![warn(missing_docs)]

/// 应用启动与默认配置。
pub mod app;
/// 模拟循环、停止条件和运行结果。
pub mod simulation;
/// 世界时间与模拟事件。
pub mod timeline;
/// 模拟世界及其领域对象。
pub mod world;
