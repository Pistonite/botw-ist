# Inventory Slot Transfer

## What is IST
*Believe it or not, Infinite Stuff Trick is NOT the name of the glitch*

IST stands for **Inventory Slot Transfer**. It is a glitch in Breath of the Wild
that exploits behavior of the inventory when the variable tracking the 
number of the items in the inventory is less than the number of items actually in the inventory.

The developers made sure that these two values are kept in sync during normal
gameplay. However, in very specific scenarios, the game removes the item
slot from the inventory while subtracting the number of items twice,
resulting in the game tracking 1 fewer item slots in the inventory.
By repeating the action, we can make the game track fewer and fewer items.

The difference between the number of items in the inventory and number
tracked by the game is called `Offset` or number of `Broken Slots`.
`Offset` is technically more correct, but because it's ambiguous in some contexts,
this manual will refer to this number as `Broken Slots`.
The action to create the Broken Slots is referred to as `Breaking Slots`.

```admonish info
`Broken Slots` is the name used by the glitch hunting community before
the underlying concepts of the glitch were fully understood. There's nothing
actually broken about the slots.

The variable that tracks the number of items is commonly referred to by
the glitch hunters as `mCount` - a reference to the name of the variable
in the BOTW decompilation project.

This variable is needed because the inventory is stored as a (doubly-)linked list,
which has a O(N) time complexity for calculating its length.

The different counts have this relation:

    mCount + Number of broken slots = Actual number of items

```

## Inventory Representation
The inventory that you see when opening the inventory in-game - the `Visible Inventory` - is 
stored as a (doubly-)linked list. In this list representation, items in different categories
are "concatenated" into the same list. For example, in normal inventory order,
the list may have all the weapons, followed by all the bows, followed by
all the arrows, etc, and at the end are all the key items. In one page
of the item in the in-game UI, the top-left corner is first,
and it follows row-major order (i.e. the item in row 1 column 2 is after the item
in row 1 column 1 in the list), and the bottom-right corner is last,
followed by the upper-left corner item of the next page.

```admonish info
The empty spaces and empty tabs in the inventory do not take space in the list.
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

```admonish tip
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
