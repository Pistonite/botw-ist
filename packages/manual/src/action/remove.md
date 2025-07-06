# Remove Items

Remove items from the inventory.

- See [Shop](./shop.md) for specially removing items from selling the item to a shop owner
- See [Equipments](./equip.md) for removing equipments from the overworld.

## Syntax

> `drop` <br>
> `drop` [`CONSTRAINED_ITEM_LIST`](../user/syntax_item.md) <br>
> `dnp` [`CONSTRAINED_ITEM_LIST`](../user/syntax_item.md) <br>
> `eat` [`CONSTRAINED_ITEM_LIST`](../user/syntax_item.md) <br>
> `use` [`ITEM_NAME`](../user/syntax_item.md) `[X times]` <br>
> `!remove` [`CONSTRAINED_ITEM_LIST`](../user/syntax_item.md) <br>

Annotations: [`:overworld`](#drop-equipments)

Examples
```skybook
hold 5 apples
drop
drop 15 apples
!remove all cores
```

## Hold and Drop
The <skyb>drop</skyb> action is used to drop the materials being held in the overworld
to the ground. 

If a list of materials is specified, it's equivalent to:
- <skyb>hold</skyb> the item,
- <skyb>drop</skyb> it,
- Repeat for every item, one at a time.

See [Hold Items](./hold.md) as well.

The <skyb>dnp</skyb> command stands for `drop and pick up`, which can be used
to re-order the items in the inventory.


Example
```skybook
dnp 10 apples 20 bananas
# which is equivalent to:
drop 10 apples 20 bananas
pick-up 10 apples 20 bananas
```

For simplicity, items will not despawn in the middle of <skyb>dnp</skyb>


## Drop Equipments
The <skyb>drop</skyb> action is also used to drop the equipments directly from inventory or
from the overworld player (e.g. getting shocked).

Examples
```skybook
# This does the following:
# - open inventory
# - select first royal-claymore, drop it
# - select another royal-claymore, drop it as well
drop 2 royal-claymore

# Drop the weapon directly from the overworld player
# For example when getting shocked
:overworld drop weapon
```

```admonish
You can also use <skyb>throw</skyb> and <skyb>display</skyb> to remove equipments
from the overworld player
```

## Using fairy
The <skyb>use</skyb> command removes items from the inventory while in the overworld. In the game,
this is only possible to remove fairies by having them save Link from death.
However, the simulator lets you remove any item.

```skybook
use fairy 2 times
use wood # No known way to do this in game
```

```admonish tip
The <skyb>use</skyb> command is also used for [using equipments in the overworld](./equip.md)
```

```admonish warning
Since the game can only remove items in this way by the item name, it's not possible
to specify extra properties, including food effects.
```

## Forcefully remove items
The <skyb>!remove</skyb> supercommand lets you forcefully delete items from the inventory:
- For Arrows, Materials, Foods, and Key Items, the value of the slot will decrease by the amount
- For the rest, the amount in the command corresponds to how many slots of this item you want to remove.

```admonish warning
The implementation of this command is custom and do not correspond to any of the game's code. Using
this command can lead to inaccurate simulation that's not reproducable in game.

For example, overworld equipments are not synced when removed this way!
```

## Detail
- <skyb>drop</skyb> and <skyb>dnp</skyb>requires [`Overworld` screen](../user/screen_system.md)
  if dropping materials, and `Inventory` for dropping equipments.
  - The simulator may switch screen back and forth during the same <skyb>drop</skyb>
    action to facilitate this
- <skyb>use</skyb> requires `Overworld` screen.
