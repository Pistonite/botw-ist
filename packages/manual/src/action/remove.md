# Remove Items

Remove items from the inventory.

See [Shop](./shop.md) for specially removing items from selling the item to a shop owner
See [Use Items]() for using fairies outside of the inventory.

## Syntax
> `drop` <br>
> `drop` [`CONSTRAINED_ITEM_LIST`](../user/syntax.md)<br>
> `eat` [`CONSTRAINED_ITEM_LIST`](../user/syntax.md) <br>
> `!remove` [`CONSTRAINED_ITEM_LIST`](../user/syntax.md) <br>

Examples
```skybook
hold 5 apples
drop
drop 5 apples
!remove all cores
```

## Drop currently held items
The <skyb>drop</skyb> action is used to drop the items being held in the overworld
to the ground. If a list of items is specified, it's equivalent to <skyb>hold</skyb>ing
those items first, then <skyb>drop</skyb>ping them. See [Hold Items](./hold.md) as well.

Note that <skyb>drop</skyb> cannot work around the hold limit of 5 items:
```skybook
drop 15 apples 
// above will still only drop 5 apples
// because it is equivalent to:
hold 15 apples // <-- here you can only hold 5 max
drop
```

See [Forcefully remove items](#forcefully-remove-items) to see how you might workaround this,
if you are dropping a lot of items

## Forcefully remove items
The <skyb>!remove</skyb> supercommand lets you forcefully delete items from the inventory:
- For Arrows, Materials, Foods, and Key Items, the value of the slot will decrease by the amount
- For the rest, the amount in the command corresponds to how many slots of this item you want to remove.

```admonish warning
The implementation of this command is custom and do not correspond to any of the game's code. Using
this command can lead to inaccurate simulation that's not reproducable in game.
```

## Detail
- <skyb>drop</skyb> requires [`Overworld` screen](../user/screen_system.md)
  
