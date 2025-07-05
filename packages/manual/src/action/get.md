# Get Items

Adding new items to the inventory.

See [Shop](./shop.md) for specially buying items from a shop.

## Syntax
> `get` [`FINITE_ITEM_LIST`](../user/syntax_item.md)<br>
> `pick-up` [`CONSTRAINED_ITEM_LIST`](../user/syntax_item.md#)<br>

Annotations: [`:pause-during`](#pause-on-item-text-boxes), [`:accurately-simulate`](#performance)

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
The <skyb>:pause-during</skyb> annotation can be used to simulate this action.

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

## Performance
The preferred way to simulate getting multiple stackable items, is by invoking the function
for adding the item to inventory repeatedly. However, when the number of items to get
is large, this is a very expensive operation and can slow down script execution significantly.

Therefore, when the amount specified is greater than some internally determined amount,
the implementation switches to a single call of the function with a value. Functionally,
it turns:
```skybook
get 999 apple
```
into:
```skybook
get apple[value=999]
```
Most of the time (if not all), this will not cause inaccuracies. However, if it matters,
you can use the <skyb>:accurately-simulate</skyb> annotation to force the more accurate implementation.

```skybook
# This may take 30 seconds or more to execute, depending on your hardware
:accurately-simulate get 999 apples
```


## Detail
- Both <skyb>get</skyb> and <skyb>pick-up</skyb> require [`Overworld`](../user/screen_system.md) screen.
- You cannot get new items while holding items in the overworld
  - with <skyb>:smug</skyb>, the held items will be dropped after getting the item
