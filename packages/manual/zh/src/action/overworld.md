# 主世界操作

主世界中可以执行的操作：

- <skyb>use</skyb>使用装备（消耗耐久）。也可用于在主世界中直接从背包删除物品，例如使用精灵。 <skyb>use fairy</skyb>.
- <skyb>shoot</skyb> 射箭，同<skyb>use bow</skyb>（使用弓）。
- <skyb>:overworld drop</skyb> 直接在主世界中丢弃装备。

同时，<skyb>spawn</skyb>指令可用于直接在主世界中（地上）生成物品。

## 语法
> `use CATEGORY_OR_ITEM` (默认一次) <br>
> `use CATEGORY_OR_ITEM X times` <br>
> `shoot` <br>
> `shoot X times` <br>
> `:overworld drop` [`CONTRAINED_ITEM_LIST`](../user/syntax_item.md) <br>
> `spawn` [`FINITE_ITEM_LIST`](../user/syntax_item.md) <br>

可用注解：
  - [`:per-use X`](#使用装备) - 设定每次使用装备时消耗的耐久。
  - `:overworld` - 使<skyb>drop</skyb>在主世界执行。

## 使用装备

使用<skyb>use</skyb>指令时，可以指定装备类型或装备中的物品名。

```skybook
# 使用装备中的武器砸地
use weapon
# 砸5次地
use weapon 5 times
# 使用王族弓（射箭），必须当前装备王族弓才行
use royal-bow
# 用当前装备的弓射箭
shoot
```

<skyb>:per-use</skyb>注解可用于修改消耗的耐久值。默认为`100`。

```skybook
# 炸盾消耗30耐久
:per-use 3000 use shield
```

特殊情况：
  - 使用带`IsLifeInfinite=true`参数的武器时不会消耗耐久。
  - 使用光之弓或黄昏光弓时不会消耗箭。
  - 使用开光大师剑（GDT旗标`Open_MasterSword_FullPower`）时消耗指定耐久乘`0.2`，当前耐久值小于`300`除外。

## 使用（非装备）物品

如果<skyb>use</skyb>指定的物品不是装备类，则会尝试直接从背包中移除该物品。唯一在游戏中的场景是删除精灵。但模拟器允许此法删除任何物品。

```skybook
# 用精灵法制作转存格
hold fairy; use fairy; drop
```

## 丢弃装备

一些情况下，可以在不开背包的情况下将主世界装备丢弃，比如被电。

```skybook
get axe
:overworld drop weapon
```

## 主世界生成物品
<skyb>spawn</skyb>指令可用于在地上生成物品。此法生成的物品不受物品数量限制，且在过载状态下也能生成。

```skybook
# 模拟射出一根炸箭掉到地上
shoot
spawn bomb-arrow
```
装备类物品可使用元属性附加耐久或附魔。其他元属性会被忽略。

## 细节

- <skyb>use</skyb> 需要[主世界](../user/screen_system.md)界面。
- [无箭强持](./material.md#无箭强持)状态下，使用武器或盾会取消手持，而使用弓会丢弃手持物品。
