# 任务 02：建立模拟配置

## 状态

- 状态：待开始
- 所属步骤：01 基础循环
- 前置任务：任务 01

## 目标结果

新增 `SimulationConfig`，可以明确保存本次自动模拟需要执行的回合数。

## Rust 学习点

- 学习结构体定义、构造函数和字段可见性。

## 开始前阅读

- `src/app.rs`
- [基础循环说明](../README.md)

## 实现步骤

- [ ] 创建 `src/simulation/mod.rs`。
- [ ] 在其中定义只包含 `rounds: u32` 的 `SimulationConfig`。
- [ ] 实现 `SimulationConfig::new(rounds: u32) -> Self`。
- [ ] 提供读取回合数的方法，暂不直接暴露可变字段。
- [ ] 在 `src/main.rs` 声明 `simulation` 模块，但暂不修改 `App::run`。
- [ ] 添加测试：传入 `0`、`1`、`100` 时能够读回相同值。

## 文件范围

- 允许修改：`src/main.rs`、`src/simulation/mod.rs`。
- 暂不修改：`App`、时间线、角色、随机数和命令行参数。

## 验证命令

```bash
cargo fmt --check
cargo test simulation
```

## 完成标准

- [ ] `SimulationConfig` 只负责保存配置，不执行循环。
- [ ] 三个边界值测试通过。
- [ ] 未添加当前用不到的配置字段。

## 实际结果

待完成后填写。
