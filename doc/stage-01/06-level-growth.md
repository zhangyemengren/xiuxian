# 步骤 06：等级成长

## 目标

角色经验达到要求后自动升级，并按照成长规则提升基础属性。

## 依赖

- 步骤 05：战斗结算。

## 范围

- 按等级配置升级所需经验；
- 每回合结算后检查升级；
- 支持一次结算连续提升多个等级；
- 升级后随机提升体质、灵力属性或悟性；
- 输出升级前后等级和属性变化。

## 非目标

- 境界突破选择；
- 技能解锁；
- 等级上限后的玩法；
- 玩家分配属性点。

## 验收标准

- 经验不足时不升级；
- 经验达到阈值时准确升级；
- 超额经验得到保留；
- 一次获得大量经验时可连续升级；
- 属性提升范围符合配置；
- 固定随机种子时成长结果可复现。

## 实现方向

```rust
struct GrowthRule {
    required_experience: u32,
    attribute_increase_min: u32,
    attribute_increase_max: u32,
}

struct LevelUpResult {
    from_level: u32,
    to_level: u32,
    remaining_experience: u32,
    attribute_changes: Attributes,
}
```

每回合结算后循环检查升级条件，因此一次可以连续升级。升级阈值集中管理，超额经验保留，属性成长使用统一随机源。
