//! 模拟过程中产生的时间与事件。

mod event;
/// 世界时间及其推进规则。
pub mod time;

pub use event::TimelineEvent;
