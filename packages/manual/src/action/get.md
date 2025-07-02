# Get Items

Adding new items to the inventory.

See [Shop](./shop.md) for specially buying items from a shop.

## Syntax
> `get` [`FINITE_ITEM_LIST`](../user/syntax.md#finite-vs-constrained-item-specifier)<br>
> `pick-up` [`CONSTRAINED_ITEM_LIST`](../user/syntax.md#finite-vs-constrained-item-specifier)<br>

Annotations: [`:item-box-pause`](#pause-on-item-text-boxes)

Examples
```skybook
get diamond             # 1 Diamond
get 2 apple 2 banana    # 1 Diamond, 2 Apples, 2 Bananas
drop all apples         # 1 Diamond, 2 Bananas. 2 Apples on the ground
pick-up all apples      # 1 Diamond, 2 Bananas, 2 Apples
```

## Picking up previously dropped Items

The only difference between <skyb>get</skyb> and <skyb>pick-up</skyb>
is that <skyb>pick-up</skyb> is used to target items previously [dropped](./remove.md)
on the [ground](../user/overworld_system.md).

You cannot <skyb>pick-up</skyb> items that aren't on the ground. Use <skyb>get</skyb>
instead.

## Pause on Item Text Boxes
When picking up a new item, opening a chest, or getting item from some event (such as a Korok),
you will get an item text box that allows you to open the pause menu.
The <skyb>:item-box-pause</skyb> annotation can be used to simulate this action.

```admonish warning
The simulator does NOT check if you are allowed to open the pause menu when you get
an item, nor does it check if normal pause menu operations can be performed.

For example, usually you can eat something immediately in the text box that you got it,
but you cannot hold another item. Currently, this situation is too complex to simulate correctly.
```

One use case is to force hold items during an item text box by performing [Item Smuggle for Arrowless Offset](./hold.md#smuggle-state-for-arrowless-offset),
then get an item text box (similar to performing [Arrowless Offset](./break_slots.md#arrowless-offset)).

```skybook
get 2 shrooms
:smug hold 2 shrooms
# Open a chest, for example
:item-box-pause get lynel-shield 
# Here, you are in pause screen while holding 2 shrooms
unpause
# Now the 2 shrooms will drop to the ground because of how the smuggle works
```

## Detail
- Both <skyb>get</skyb> and <skyb>pick-up</skyb> require [`Overworld`](../user/screen_system.md) screen.
- You cannot get new items while holding items in the overworld
  - with <skyb>:smug</skyb>, the held items will be dropped after getting the item
