# 制作转存格

**制作转存格**指增加物品计数差的操作。见[物品转存简介](../ist/index.md)。

你可以模拟游戏中制作转存格的方式，或直接用[<skyb>!break</skyb> supercommand](./low_level.md#generate-broken-slots)修改内存制作。

## 无箭法

```admonish info title="信息"
无箭法制作转存格需要的指令参考
- [`:smug hold`](./material.md#无箭强持)
- [`sell`](./sell.md)
```

无箭法是最常用的制作方式。只需要一个盾，一个单手武器，和收购物品的NPC。一次性可以最多做5个转存格：

- 触发[无箭强持](./material.md#无箭强持)。
- Talk to a shop keeper (by pressing `Dpad Down > A` or `ZR > A` quickly).
- Sell all the items from slots that are being held.
- Close the dialog.

Example script for Arrowless Offset in the simulator:

```skybook
get 2 shroom 2 pepper 1 banana
:smug hold shroom pepper
sell all materials[held]
close-dialog
```

## 强持法

```admonish todo
Menu Overload functionality is WIP.
```

## 精灵法
You can use fairies to break slots by using the last fairy while holding one.

Example script:

```skybook
hold all but 1 fairy;
use fairy; # by bombing yourself, or stand on fire, etc...
drop; # drop the held fairy
```

## 选项纠缠法
With [Prompt Entanglement](../ist/pe.md), you can eat and hold the same slot
to make offsets.

- Eat all of the material in the slot you are using PE with
  - Since targeting a translucent slot with PE will target the first slot,
    you need to either make sure you are eating the first slot,
    or eat the slot you are using AND the first slot.
- Use a "drop" prompt to hold the slot you ate.
- Unpause and drop the items in your hand.

The example script below uses the 3rd slot in a tab (<skyb>shroom</skyb>)
so it also needs to eat all of the first slot (<skyb>apple</skyb>).

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
