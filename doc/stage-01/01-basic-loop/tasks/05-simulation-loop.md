# 任务 05：实现模拟循环

## 状态

- 状态：待开始
- 所属步骤：01 基础循环
- 前置任务：任务 02、任务 04

## 目标结果

`Simulation::run` 根据配置推进回合，并返回包含有序开始、结束事件的时间线。

## Rust 学习点

- 学习范围循环 `for round in 1..=rounds` 和返回值所有权。

## 开始前阅读

- `src/simulation/mod.rs`
- `src/time_line/`
- [基础循环说明](../README.md)

## 实现步骤

- [ ] 先写 `rounds = 1` 的失败测试，期望产生开始和结束两条事件。
- [ ] 定义持有 `SimulationConfig` 的 `Simulation`。
- [ ] 实现构造函数。
- [ ] 实现 `run(self) -> Timeline` 或等价的清晰接口。
- [ ] 对每个回合先追加开始事件，再追加结束事件。
- [ ] 添加测试：回合编号从 `1` 开始且连续。
- [ ] 不在模拟器内使用 `println!`。

## 文件范围

- 允许修改：`src/simulation/`，只读取 `src/time_line/` 的公开接口。
- 暂不修改：`App`、角色、修炼、事件、随机数和统计。

## 验证命令

```bash
cargo fmt --check
cargo test simulation
```

## 完成标准

- [ ] `N` 个回合产生 `2N` 条有序事件。
- [ ] 循环层不负责文本输出。
- [ ] 测试可以直接检查结构化事件。

## 实际结果

待完成后填写。
