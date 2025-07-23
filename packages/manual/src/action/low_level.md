# Low Level Operations

These supercommands allow directly editing memory for prototyping or testing,
or to workaround limitations of the simulator.

```admonish danger
Because these commands edit memory directly, instead of mimicking what the game does,
they could be very inconsistent with the behavior of the game sometimes!

Make sure you read the doc carefully before using them!
```

## Syntax

Examples are available at each section below.

[Generate Broken Slots](#generate-broken-slots)
> `!break X slots` <br>

[Adding item slots directly](#add-item-slots)
> `!init` [`FINITE_ITEM_LIST`](../user/syntax_item.md) <br>
> `!add-slot` [`FINITE_ITEM_LIST`](../user/syntax_item.md) <br>

[Forcefully remove item](#forcefully-remove-items)
> `!remove` [`CONSTRAINED_ITEM_LIST`](../user/syntax_item.md) <br>

[Change item data](#change-item-data)
> `!write` [`[META]`](../user/syntax_item.md#metadata) `to` [`ITEM`](../user/syntax_item.md)<br>
> `!swap` [`ITEM1`](../user/syntax_item.md) `and` [`ITEM2`](../user/syntax_item.md) <br>

## Generate Broken Slots

```admonish tip
The simulator supports breaking slots using actions you would
do in-game. See [Break Slots](./break_slots.md).
```

The <skyb>!break</skyb> command edits `mCount` of `list1` and `list2` directly
to effectively break slots "by magic".

Example
```skybook
!break 20 slots
```

## Add Item Slots
The <skyb>!init</skyb> and <skyb>!add-slot</skyb> command will directly
push a new item from `list2` to `list1`, and set the memory according
to the item you specified. This will bypass ALL checks for adding item to inventory.

Note that it will still prevent adding items when `list2.mCount` is `0`.

Furthermore, <skyb>!init</skyb> will reset `list1` and `list2` to the initial state
(where `list2` has `420` items and `list1` has `0`). This means all broken slots
will be cleared as well.

Example:

```skybook
# setting items without sorting
!init 1 slate 1 glider 5 apples
# adding items not addable (doesn't have CanGetPouch flag)
!add-slot <DgnObj_EntanceElevator_A_01>
# when adding stackable items with [value=...], the "amount" becomes
# how many slots to add. e.g. the command below will add 5 slots of 300x arrows
!add-slot 5 arrow[value=300]
```

```admonish note
The inventory state and GameData will be synced. This will also
set the corresponding `IsGet` flag for the item, and the `IsOpenItemCategory`
for the corresponding category.
```


## Forcefully Remove Items

The <skyb>!remove</skyb> supercommand lets you forcefully delete items from the inventory:
- For Arrows, Materials, Foods, and Key Items, the value of the slot will decrease by the amount.
- For the rest, the amount in the command corresponds to how many slots of this item you want to remove.

Inventory and GameData state are fixed and synced afterward.

Example:

```skybook
!remove all cores
```

```admonish warning
This command can target items that are normally inaccessible in the pouch screen.
For example, when `mCount` is `0`, or when the item slot is over the maximum available slots
for Weapon/Bow/Arrow/Shield.
```


## Change Item Data

The <skyb>!write</skyb> supercommand lets you edit the data for an item using the
[Item Meta Syntax](../user/syntax_item.md#metadata). Inventory is fixed afterward,
but GameData is NOT synced (for historical reason).

Currently, changing the `ingredients` is not supported.

Examples:

```skybook
# Change the value of the Master Sword to 0
# NOTE THAT THIS DOES NOT BREAK IT 
# to break Master Sword properly (allowing MSWMC), you have to use the `use` command
!write [value=0] to master-sword

# Change the price of wild greens with price 102 to 101
!write [price=101] to wild-greens[price=102]

# When targeting the item by position, you can also change the item name
# Change the item in material tab 1, row 1, column 2, to royal-claymore
# with durability 20 (no matter what the item at that position is)
!write [dura=20] to royal-claymore[category=material, row=1, col=1]
```

```admonish warning
This command can target items that are normally inaccessible in the pouch screen.
For example, when `mCount` is `0`, or when the item slot is over the maximum available slots
for Weapon/Bow/Arrow/Shield.
```

The <skyb>!swap</skyb> supercommands targets 2 items, and swap their nodes in the list.
Inventory is fixed afterward, but GameData is NOT synced (for historical reason).

Examples:

```skybook
# Swap the first apple stack and the first banana stack
!swap apple and banana

# Swap the equipped royal claymore and the equipped bow (whatever the bow is)
!swap royal-claymore[equipped] and bow[equipped]
```
