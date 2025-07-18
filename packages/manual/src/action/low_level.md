# Low Level Operations

These supercommands allow directly editing memory for prototyping or testing,
or to workaround limitation of the simulator.

```admonish danger
Because these commands edit memory directly, instead of mimic what the game does,
they could be very inconsistent with the behavior of the game sometimes!

Make sure you read the doc carefully before using them!
```

## Syntax
Generate Broken Slots
> `!break X slots` <br>

Adding item slots directly
> `!init` [`FINITE_ITEM_LIST`](../user/syntax_item.md) <br>
> `!add-slot` [`FINITE_ITEM_LIST`](../user/syntax_item.md) <br>


Forcefully remove item
> `!remove` [`CONSTRAINED_ITEM_LIST`](../user/syntax_item.md) <br>

Examples are available at each section below

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

Note that it will still prevent adding item when `list2.mCount` is `0`.

Furthermore, <skyb>!init</skyb> will reset `list1` and `list2` to the initial state
(where `list2` has `420` items and `list1` has `0`). This means all broken slots
will be cleared as well.

Example
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


## Forcefully remove items
The <skyb>!remove</skyb> supercommand lets you forcefully delete items from the inventory:
- For Arrows, Materials, Foods, and Key Items, the value of the slot will decrease by the amount
- For the rest, the amount in the command corresponds to how many slots of this item you want to remove.

Examples
```skybook
!remove all cores
```
