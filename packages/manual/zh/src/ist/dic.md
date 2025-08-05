# 数值讹转 (DIC)

*Inventory Corruption* is a form of *Durability Transfer*.

In the game, durability is stored as a *fixed-point* integer,
with `1` being `0.015`. For example, a weapon with durability `10` has the internal
value of `1000`. In the case of corruption, the durability is transferred
from an equipment - not to another equipment, but to an item. The value
then becomes the count of items. This is very useful, since
you can get a LOT of items from an equipment with relatively low durability
that's easy to get.

This form of corruption was previously only possible using *Memory Storage* -
another very complex glitch with a lengthy setup. With IST, however,
Inventory Corruption is much easier, hence the name *Direct Inventory Corruption*.

We will go over a few game mechanics first in order to understand
why DIC happens.

## Setting Durability on Equipment
The game sometimes need to set the durability of the equipment in the inventory from the overworld weapon actor
in order to keep the durability in sync. This could happen in a few scenarios, including:
- Using an equipment, such as attacking with weapon, shield surf and shooting an arrow (which uses both bow and arrow)
- Switching equipment
- After reloading from a save

The algorithm to do this is:
1. Find the first item in the inventory list that is both equipped and is the same item that is trying to be set (for example, Master Sword)
2. Set the value of that item

If you remember from [Inventory Representation](./index.md#inventory-representation),
the game also need to sync this change to `GameData`. However, syncing the whole
inventory whenever one value changes seem inefficient. Therefore, this form
of sync is done by directly taking the position of the item in the inventory list,
and setting the value of the item in the same position in `GameData`.

This is correct if the `GameData` is always in-sync with `Visible Inventory`,
which would be the case most of the time. Therefore, all the inventory corruption
aims to do is cause `GameData` to be desynced, then trigger a durability set.

```admonish info
Note that this is only one form of desync, which is the primary one used for inventory corruption.
There is another form of desync used by inventory corruption using Inventory Storage, which is a 
derivative of Map Storage. We will not be going into the details for that.
```

## GameData Corruption with IST
All it takes for corruption now is 2 things:
1. Desyncing GameData
2. Apply durability while GameData is desynced.

IST makes desyncing GameData trivial. Recall the 3 steps for reloading:

1. Delete all items
2. Load GameData
3. Add items in GameData to Inventory

After these steps, the game doesn't actually sync `GameData`. Therefore,
*whenever items are transferred, the `GameData` is automatically desynced
after a reload*.

Remember that reloading a save also causes durability to be applied?
This means inventory corruption automatically happens after a reload.
All the player needs to do to exploit this, is to transfer specific items
to desync the `GameData` in the way so that equipped items are aligned
to the item to corrupt. This is why different IST setups exist to corrupt
different things in different speedruns.

```admonish tip
This is also why it is important to follow the setup to unequip/equip
certain items before reloading, because the equipped item slot is what is
used for corruption. To be exact, the durability of the *last equipped slot*
is transferred into the *first equipped slot* in *both* `Visible Inventory`
and `GameData`.
```

## Aligning the Items

In a DIC setup, we need to align the equipped equipment with the item to corrupt.
This is done by transferring the right number of Swords, Shields, Bows, and Arrows.
Transferring anything after Arrows typically has no effect, because they don't
affect the positions of equipments since those categories are after equipments
in inventory order.

Recall that transferring an item means it is not removed in the inventory.
Since the items from GameData are then added on top of that, the transferred
items will appear before the items that are loaded in. Effectively, transferred
equipments will *push* items from the save to slots after it.

To determine the right type of equipment to transfer, consider the order of
the categories:
- Since Shield is last, transferring any type of equipment will affect the position of shields
- Since Weapon is first, only transferring weapon will affect the position of weapons.
- The rest follow the same concept

```admonish example
For example, transferring 1 Bow, 1 Arrow, 1 Shield, will:
- Not change position of Weapons
- Push Bows by 1 slot
- Push Arrows by 2 slots
- Push Shields by 3 slots
```

## Unsorted Inventory and Leftward Corruption
Note that GameData desynced in this way can only push the items *to the right*, not *left*.
For example, Weapons can corrupt everything, but Bows cannot corrupt Weapons, and Shields cannot corrupt Weapons,
Bows, or Arrows. 

Corrupting items that are not possible to corrupt in normal inventory order is known as `Leftward Corruption`,
or sometimes `Forward Corruption`.
This is done by making the inventory into an order that is not normal. For example, to transfer
durability from a Shield to a Sword, the Sword must be put after the Shield. This state is typically known
as `Unsorted Inventory` and opens up a whole different glitch category.

Achieving `Unsorted Inventory` is relatively easy. All we need to look at is how the game
put (sort) the item into the correct category when you get something:

1. The game always adds the item to the end of the inventory
2. The game then sorts the inventory with the following rules:
   - Two items should not change their relative order (if A is before B, A must also be before B after sorting)
   - Two items that have the same category are considered equal
   - Items that should appear in a category before other categories have lower values (say Sword is 0, Bow is 1, etc)
   - The list is sorted from lowest value to highest

```admonish info
This type of sort is referred to as a *stable sort* using a *predicate* that only compares the category of the items.
```

The sorting itself cannot achieve the unsorted state, but *absence of sort* can.
The list code has one more optimization, that sort operations are skipped if
the list has no more than one element. Normally, having 0 or 1 element in the list
means the list is trivially sorted. However, `mCount` is used for this optimization
and we know `mCount` is not the actual number of items in the inventory.
All it needs for the sort to be skipped is to make your `mCount` less than or equal to
one.

```admonish tip
Unsorted Inventory can also be used to transfer more equipment with fewer Broken Slots by
putting Weapons after Key Items. This is typically used in speedrun setups where it could be
faster to not break as many slots. This is why some setups require dropping some weapons, then
immediately pick them up again. While picking up the weapons, the `mCount` never surpasses
`1`, causing the weapons you pick up remain at the end of the inventory.
```
