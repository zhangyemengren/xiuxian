# 步骤 03：修炼系统

## 目标

让角色在每回合自动选择一种修炼方式，消耗灵力值并获得经验。

## 依赖

- 步骤 02：角色状态。

## 修炼类型

- 炼气；
- 筑基；
- 金丹。

不同修炼类型具有独立的灵力消耗和经验收益配置。

## 范围

- 每回合随机选择一种当前可执行的修炼；
- 修炼成功后扣除灵力值并增加经验；
- 输出修炼类型、消耗和收益；
- 灵力值不足时不允许出现负数，并记录失败原因。

## 非目标

- 修炼触发随机事件；
- 自动恢复灵力值；
- 升级和属性成长；
- 玩家选择修炼类型。

## 验收标准

- 每回合最多执行一次修炼；
- 修炼类型只可能来自定义列表；
- 灵力值和经验变化与对应配置一致；
- 灵力值不足时状态保持合法；
- 固定随机种子时修炼序列可以复现。

## 实现方向

```rust
enum CultivationType {
    QiRefining,
    FoundationEstablishment,
    GoldenCore,
}

struct CultivationRule {
    mp_cost: u32,
    experience_gain: u32,
}

struct CultivationResult {
    cultivation_type: CultivationType,
    mp_spent: u32,
    experience_gained: u32,
    succeeded: bool,
}
```

执行顺序：选择修炼类型、检查灵力、应用消耗和收益、返回结构化结果、记录时间线。修炼规则集中配置，规则层不直接打印日志。
