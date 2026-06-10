# 步骤 01：基础循环

## 完成后可以看到

程序可以按配置执行指定数量的回合，并按顺序输出每个回合的开始和结束日志。

## 本步骤要做

- 配置模拟回合数；
- 按顺序执行每个回合；
- 保存结构化的回合开始和结束记录；
- 输出时间线；
- 模拟结束后输出总回合数。

暂时不做角色、修炼、事件、战斗、成长、随机数和统计。

## 实现方向

```rust
struct SimulationConfig {
    rounds: u32,
}

struct Simulation {
    config: SimulationConfig,
}

enum TimelineEvent {
    RoundStarted { round: u32 },
    RoundFinished { round: u32 },
}
```

`App` 创建配置并启动 `Simulation`。`Simulation` 使用 `1..=rounds` 推进回合并返回时间线，不直接打印日志；`App` 最后统一输出。

## 任务清单

按表格顺序执行。每个任务文档都写了需要阅读的文件、具体操作和验证命令。

| 顺序 | 任务 | 状态 |
| ---: | --- | --- |
| 1 | [建立可验证基线](./tasks/01-baseline.md) | 进行中 |
| 2 | [建立模拟配置](./tasks/02-simulation-config.md) | 待开始 |
| 3 | [定义回合记录](./tasks/03-round-record.md) | 待开始 |
| 4 | [实现时间线容器](./tasks/04-timeline.md) | 待开始 |
| 5 | [实现模拟循环](./tasks/05-simulation-loop.md) | 待开始 |
| 6 | [接入 App 并输出](./tasks/06-app-output.md) | 待开始 |
| 7 | [补齐边界验收](./tasks/07-acceptance.md) | 待开始 |

## 验收标准

- 输入回合数 `N` 后，程序恰好执行 `N` 个回合；
- 回合编号从 `1` 开始且连续；
- `N = 0` 时不执行回合，并正常结束；
- 日志顺序与实际执行顺序一致；
- `0`、`1`、`100` 回合都有自动化测试；
- `cargo fmt --check`、`cargo test` 和 `cargo run` 全部成功。
