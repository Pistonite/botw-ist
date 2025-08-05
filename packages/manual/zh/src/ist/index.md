# 物品转存简介

```admonish info title="信息"
本篇对应B站上[物品转存教程第一期：基础理论](https://www.bilibili.com/video/BV1Dn87zoEXs)。
```

## 什么是IST

IST是**Inventory Slot Transfer**的全称。它是旷野之息中利用了物品计数和物品实际数量不同步的Bug。

开发者确保了在一般游戏操作中，这两个变量始终保持同步。但是有些操作会导致一个物品格子被重复删除，导致物品计数比物品实际数量少1，然后重复操作，即可使物品计数越来越少。物品计数与实际物品数的差称为“物品计数差”，或者“转存格”。“转存格”不是实际存在的格子，所以用“物品计数差”更准确。但是“转存格”的叫法使用更广泛，所以在本手册中，我们将用“转存格”一词。生成计数差的操作称为“制作转存格”。

## 背包结构
游戏中，打开背包界面能看到的物品我们称之为“可视背包”。可视背包由一个双链表组成。背包中所有页面的物品都连在同一个表中。比如，在正常背包顺序下，表中有所有的武器，然后所有的弓，然后所有的箭，以此类推，最后是所有的重要道具。在背包界面的一页物品中，最左上的物品是表里排最靠前的，最右下的物品是最靠后的。然后先连一行的物品，再到下一行。

```admonish info title="信息"
没有物品的格子称为空格。空格不在背包表中，只是在界面中的显示效果。
```



The items are also stored at the same time in `GameData`, which is the game's flag system.
The relevant flags are stored with an array type. For example, `PorchItem` is a flag that
stored an array of 420 strings, each string correspond to one item's name.
Other properties of the items are stored each in a different flag, for example
`PorchItem_EquipFlag` is an array of 420 `bool`s (whether the item is equipped),
and `PorchItem_Value1` is an array of 420 `s32`s (the value/durability of the item).

Whenever the `Visible Inventory` changes, the change is synchronized to `GameData`.
We call this process `Sync GameData` or simply `Sync`. The `GameData` is also
what is stored in the save files.

```admonish tip title="技巧"
When `mCount` is 0, you won't be able to see the items in the inventory when you open it.
This is because the game `thinks` the inventory is empty since the number of items is 0.
However, the items are still there. You can throw a weapon or pick up any item - 
as long as mCount is no longer 0, you will be able to access the inventory again.
```

## Why is it called IST - The main mechanism
The huge number of glitches that are derived from IST all rely on the core
mechanism - transferring slots, which is how the glitch got its name.

This all happens when the inventory is `Reload`-ed, either when loading a save,
or restoring the inventory from a quest that takes away your items (i.e.
Trial of the Sword, Stranded on Eventide, or any of the Blight Refights).

When restoring the inventory, the game needs to do 3 things:

1. Delete all of your current items in `Visible Inventory`.
2. Load the data of the inventory to restore into `GameData`.
   - In case of reloading a save, it is loaded from the save file.
   - In case of quests, it is already stored in `GameData`.
3. Add the items in `GameData` one-by-one into the `Visible Inventory`.

The magic happens in step 1. Since `mCount` is less than the actual number
of items, the game does not fully delete all the items in inventory.
These leftover items will still be there after the reload, effectively
being *transferred* from one save to another.

## Derivative Glitches

The following glitches depend on IST. Click on the link for more information
about each of them.
- [Direct Inventory Corruption (DIC)](./dic.md)
- [Weapon Modifier Corruption (WMC)](./wmc.md)
- [Prompt Entanglement (PE)](./pe.md)
- [Item Stack Underflow](./isu.md)
