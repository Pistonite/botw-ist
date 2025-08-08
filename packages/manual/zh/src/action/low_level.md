# 高级操作

这些高级指令可以直接修改背包内存，非常适合用于测试或者绕开模拟器的限制。

```admonish danger title="危险"
这些指令不会模拟游戏中的操作，而是直接修改内存。所以，有些效果可能和游戏中非常不同。

请在使用前仔细阅读文档！
```

## 语法

例子见下方每个指令的段落。

[制作转存格](#制作转存格)
> `!break X slots` <br>

[添加物品格](#添加物品格)
> `!init` [`FINITE_ITEM_LIST`](../user/syntax_item.md) <br>
> `!add-slot` [`FINITE_ITEM_LIST`](../user/syntax_item.md) <br>

[强制删除物品](#强制删除物品)
> `!remove` [`CONSTRAINED_ITEM_LIST`](../user/syntax_item.md) <br>

[修改物品数据](#修改物品数据)
> `!write` [`[META]`](../user/syntax_item.md#属性) `to` [`ITEM`](../user/syntax_item.md)<br>
> `!swap` [`ITEM1`](../user/syntax_item.md) `and` [`ITEM2`](../user/syntax_item.md) <br>

## 制作转存格

```admonish tip title="技巧"
模拟器支持用游戏中的操作制作转存格，见[制作转存格](./break_slots.md)。
```

<skyb>!break</skyb>指令会修改背包表和空闲表的计数额外制作X个转存格。

```skybook
!break 20 slots
```

## 添加物品格
<skyb>!init</skyb>和<skyb>!add-slot</skyb>指令会直接从空闲表中调用新物品到背包表。所有添加物品时检查的机制都会被绕开。注意空闲计数为`0`时，依然不可以添加物品。

除此之外，<skyb>!init</skyb>还会重置背包和空闲表的计数，所以转存格也会被消除。

例子：
```skybook
# 在不排序的情况下设置背包状态
!init 1 slate 1 glider 5 apples
# 添加通常无法添加的物品（如下为神庙电梯）
!add-slot <DgnObj_EntanceElevator_A_01>
# 如果添加可堆叠的物品带有[value=...]属性，则数量变为要添加的格子数
# 下面指令会添加5格箭，每格有300根
!add-slot 5 arrow[value=300]
```

```admonish note title="信息"
执行后，此指令会修复背包状态并和GDT数据同步。同时，物品对应的`IsGet`标记（标记物品是否获得过）也会设为`true`，
物品对应的页面也会设为解锁状态。
```

```admonish danger title="特别注意"
如果在打开背包界面时使用<skyb>!init</skyb>或<skyb>!add-slot</skyb>，背包界面和背包数据可能会不同步，导致添加的物品不能马上使用，需要关闭背包再打开。

虽然模拟器可以实现强制同步，但这样会导致一些内部状态改变，比如主世界装备和背包蓝格的对应关系。
```


## 强制删除物品

<skyb>!remove</skyb>指令可用于强制删除物品：
- 箭，材料，食物和重要道具删除数量以格子值为准。
- 其他以格子数为准。

执行后，此指令会修复背包状态并和GDT数据同步。


例子：
```skybook
!remove all cores
```

```admonish warning title="注意"
此指令可以指定通常情况下背包界面看不见的物品，比如当物品计数为0时，或者当装备类物品超出解锁的格子数时。
```


## 修改物品数据

<skyb>!write</skyb>指令可以通过[物品属性语法](../user/syntax_item.md#属性)修改背包状态。执行后，背包状态会修复，但不会同步到GDT。

目前不支持修改物品的配料表。

例子：

```skybook
# 把大师剑耐久设为0
# 注意，这样不会把大师剑变成损坏状态
# 如果要损坏大师剑（比如MSWMC），需要用`use`指令破坏大师剑
!write [value=0] to master-sword

# 把价格为101的炒菜的价格改为102
!write [price=101] to wild-greens[price=102]

# 如果直接指定物品位置，还可以修改该位置物品名
# 如下，修改材料页第一行第一列的物品为20耐久的王族双手剑
!write [dura=20] to royal-claymore[category=material, row=1, col=1]
```

```admonish warning title="注意"
此指令可以指定通常情况下背包界面看不见的物品，比如当物品计数为0时，或者当装备类物品超出解锁的格子数时。
```

<skyb>!swap</skyb>指令选定2个格子，并交换它们节点在链表中的位置。执行后背包状态会修复，但是不会同步到GDT。

例子：

```skybook
# 交互苹果和香蕉的位置
!swap apple and banana

# 交互装备中的王族双手剑和装备中的弓的位置
!swap royal-claymore[equipped] and bow[equipped]
```

```admonish danger title="特别注意"
如果在打开背包界面时使用<skyb>!write</skyb>或<skyb>!swap</skyb>，背包界面和背包数据可能会不同步，导致需要关闭背包再打开才能看到效果。

虽然模拟器可以实现强制同步，但这样会导致一些内部状态改变，比如主世界装备和背包蓝格的对应关系。
```
