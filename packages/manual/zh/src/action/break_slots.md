# 制作转存格

**制作转存格**指增加物品计数差的操作。见[物品转存简介](../ist/index.md)。

你可以模拟游戏中制作转存格的方式，或直接用[<skyb>!break</skyb> 高级指令](./low_level.md#generate-broken-slots)修改内存制作。

## 无箭法

```admonish info title="信息"
无箭法制作转存格需要的指令参考
- [`:smug hold`](./material.md#无箭强持)
- [`sell`](./sell.md)
```

无箭法是最常用的制作方式。只需要一个盾，一个单手武器，和收购物品的NPC。一次性可以最多做5个转存格：

- 触发[无箭强持](./material.md#无箭强持)。
- 和商店NPC对话（非常快的按`ZR > A`或者吹哨`A`）。
- 把所有手持的物品格子卖掉。
- 退出对话框。

无箭法示例脚本：

```skybook
get 2 shroom 2 pepper 1 banana
:smug hold shroom pepper
sell all materials[held]
close-dialog
```

## 强持法

强持法和无箭法类似，通过卖掉强持的物品和手动丢弃制作转存格。

- 触发强持（电箭过载）。
- 卖掉手持的格子。
- 再手持一个物品解除强持。
- 丢掉手持的物品。
由于强持法需要再手持一个物品解除强持，一次性只能制作4格转存格。

```skybook
get 2 shroom 2 pepper 1 banana
overload
hold shroom pepper
unoverload
sell all materials[held]
close-dialog
hold banana
drop
```

## 精灵法
在手持精灵格到最后一个的情况下用掉该格最后一个精灵可以制作一个转存格。

```skybook
hold all but 1 fairy;
use fairy; # 自炸等
drop; # 丢弃手持的精灵
```

## 选项纠缠法
通过[选项纠缠](../ist/pe.md)，可以通过同时吃和手持同一格来制作转存格。

- 把想要纠缠的格子吃光。
  - 纠缠目标为虚像格时，会选定页面第一格为目标，所以需要吃光第一格并纠缠，
    或者吃光想要纠缠的格子和第一格。
- 使用装备“丢弃”选项，手持吃光的格子。
- 关闭背包，丢弃手中的物品。

以下例子用了第三格纠缠（蘑菇<skyb>shroom</skyb>），所以也需要吃光第一格苹果。

```skybook
:discovered [bow, shield] # spacing for PE
get
  1 torch 1 axe 1 hammer
  1 apple 1 banana 1 shroom
eat all apple all shroom
entangle hammer
:targeting <empty>[category=material, row=1, col=3]
drop hammer
drop
```
