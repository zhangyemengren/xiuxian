//! 模拟结束结果。

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
