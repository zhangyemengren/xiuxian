# 任务 01：建立可验证基线

## 状态

- 状态：进行中
- 所属步骤：01 基础循环
- 前置任务：无

## 目标结果

确认当前代码在不实现新功能的情况下可以格式化、编译、测试和运行，为后续每个小任务建立统一起点。

## Rust 学习点

- 认识 Cargo 项目的三个基本命令：`cargo fmt`、`cargo test`、`cargo run`。

## 开始前阅读

- `Cargo.toml`
- `src/main.rs`
- `src/app.rs`
- `src/time_line/mod.rs`
- `src/time_line/time.rs`
- `src/world/mod.rs`

## 实现步骤

- [ ] 执行 `cargo fmt --check`，只修复格式问题，不改变行为。
- [ ] 执行 `cargo test`，记录当前测试数量和结果。
- [ ] 执行 `cargo run`，确认程序能正常退出。
- [ ] 如果出现错误，只做让基线恢复通过的最小修复。
- [ ] 把三条命令的实际结果写到本文末尾。

## 文件范围

- 允许修改：仅修改导致基线失败的现有 Rust 文件。
- 暂不修改：不创建模拟器，不添加回合，不添加角色和玩法规则。

## 验证命令

```bash
cargo fmt --check
cargo test
cargo run
```

## 完成标准

- [ ] 三条验证命令均成功。
- [ ] 没有引入新玩法或新依赖。
- [ ] 实际结果已记录。

## 实际结果

2026-06-10 首次检查：

- `cargo test`：通过，当前共有 `0` 个测试；
- `cargo run`：通过，程序正常退出且暂无输出；
- `cargo fmt --check`：未通过，`src/app.rs`、`src/main.rs`、`src/time_line/time.rs` 和 `src/world/mod.rs` 存在格式差异；
- 当前结论：基线可编译和运行，但完成本任务前需要仅执行格式修复并重新运行三条命令。
