# 排序

The <skyb>sort</skyb> command can be used to sort items in a category.

## 语法
> `sort CATEGORY` <br>
> `sort CATEGORY X times` <br>

Annotations: 
- [`:same-dialog`](#在出售界面排序)
- [`:accurately-simulate`](#性能优化)

Examples:
```skybook
sort weapons
sort materials 2 times
talk-to shop-keeper
sell all apples
:same-dialog sort material
untalk
```

## 装备类物品
For `Weapon`, `Bow`, `Shield`, and `Armor` categories, the game has 2 modes to sort them
that it toggles every time you sort.

For `Weapon`, the modes are:
- By weapon type first, then weapon power, then weapon modifier, and finally item name.
- By item name first, then weapon type, then weapon power, and finally weapon modifier.

For `Bow` and `Shield`, the modes are:
- By power first, then modifier, then item name.
- By item name first, then weapon power or guard power, then modifier.

For `Armor`, the modes are:
- By armor type first, then item name.
- By item name first, then armor type.

By default, the first time you sort one of these categories, it will be the first sorting
mode (not by item name first). You can toggle this with the [<skyb>!set-gdt</skyb>](./flags.md#any-flag)
supercommand.

```skybook
sort armors # will sort by armor type first (SortTypeArmorPouch changes from false to true)
sort armors # will sort by item name first (SortTypeArmorPouch changes from true to false)
!set-gdt <SortTypeArmorPouch>[bool=true] # manually set the flag to true
sort armors # will sort by item name first
sort armors # will sort by armor type first

# Flag name for other categories:
# - SortTypeWeaponPouch
# - SortTypeBowPouch
# - SortTypeShieldPouch
```

## 在出售界面排序
You can sort `Armor`, `Material` and `Food` while selling items. By default, <skyb>sort</skyb>
assumes you want to sort in inventory screen, and it will closes the shop screen
and open inventory screen even if the category specified would allow you to sort in shop screen.
This would cause less confusion with automatic screen switch.

To actually sort in selling screen, you need to use the <skyb>:same-dialog</skyb> annotation,
similar to buying from the same NPC without exiting dialog:
```skybook
talk-to beedle
sort materials # error: cannot auto switch to inventory
:same-dialog sort materials # ok: opens selling screen
```

## 性能优化
For performance, the number of times is capped at an internal max, since sorting an already-sorted
list should have no effect. Use <skyb>:accurately-simulate</skyb> to override this behavior.

```skybook
# actually press Y 300 times for some reason
:accurately-simulate sort materials 300 times
```

## 细节

- <skyb>sort</skyb> requires [`Inventory`](../user/screen_system.md) screen
  - unless <skyb>:same-dialog</skyb> is used in either `Shop Buying` or `Shop Selling` screen,
    in which case it will automatically switch to `Shop Selling`.
- Sorting will attempt to remove translucent items. If `mCount` is `0` afterward in inventory screen,
  the inventory will become inaccessible.
- Only `Armor`, `Material` and `Food` categories are allowed to be sorted while selling.
- Sorting any armor subtype has the same effect as the armor type.
- Sorting category with broken slots may not fully sort it the first time. This is related
  to how the merge sort is implemented by the game, and is not a bug in the simulator.
