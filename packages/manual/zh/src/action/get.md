# 拿物品

Adding new items to the inventory.

- <skyb>get</skyb> command adds new items from an unspecified source (makes
the item from thin air). 
- <skyb>pick-up</skyb> can only be used to get items
previously dropped on the ground. 
- <skyb>buy</skyb> is similar to <skyb>get</skyb>, but has additionally
  functionality to simulate buying from an NPC in the same dialog as selling.

## 语法
> `get` [`FINITE_ITEM_LIST`](../user/syntax_item.md)<br>
> `buy` [`FINITE_ITEM_LIST`](../user/syntax_item.md#)<br>
> `pick-up` [`CONSTRAINED_ITEM_LIST`](../user/syntax_item.md#)<br>

Annotations:
  - [`:same-dialog`](#从NPC处买东西)
  - [`:pause-during`](#新物品提示时开背包)
  - [`:accurately-simulate`](#性能优化)

Examples
```skybook
get diamond             # 1 Diamond
get 2 apple 2 banana    # 1 Diamond, 2 Apples, 2 Bananas
drop all apples         # 1 Diamond, 2 Bananas. 2 Apples on the ground
pick-up all materials   # 1 Diamond, 2 Bananas, 2 Apples
buy 5 eggs
```

## 从地上捡起之前丢的物品
The only difference between <skyb>get</skyb> and <skyb>pick-up</skyb>
is that <skyb>pick-up</skyb> is used to target items previously [dropped](./remove.md)
on the [ground](../user/overworld_system.md).

You cannot <skyb>pick-up</skyb> items that aren't on the ground. Use <skyb>get</skyb>
instead.

## 从NPC处买东西
Normally, you buy items in this game by "talking" to the item directly in the overworld.
Certain NPCs are exceptions, such as Beedle, Travelling Merchants, and Kilton.
For these NPCs, you need to talk to them, and buy from a separate dialog.

By default, <skyb>buy</skyb> will ALWAYS assume you are buying from overworld, unless
you tell it to not do so.

To talk to an NPC and buy, use the <skyb>talk-to</skyb> command.
```skybook
talk-to beedle    # Opens Shop Buying screen
buy 5 arrows
shoot             # Automatically close the screen and shoot arrow
                  # To manually close the screen, use `untalk` or `close-dialog`
```

To sell, then buy within the same dialog sequence, use the <skyb>:same-dialog</skyb>
annotation
```skybook
sell ruby                 # Opens Shop Selling screen
:same-dialog buy 5 arrows # Without exiting dialog, opens Shop Buying screen
close-dialog
```

Also see [Selling](./sell.md).

## 新物品提示时开背包
During <skyb>get</skyb>, <skyb>pick-up</skyb>, or <skyb>buy</skyb>, you may
encounter a "New Item" text box that allows you to open the inventory.

The <skyb>:pause-during</skyb> annotation can be used to simulate this action.

```admonish warning
The simulator does NOT check if you are allowed to open the pause menu when you get
an item, nor does it check if normal pause menu operations can be performed.

For example, usually you can eat something immediately in the text box that you got it,
but you cannot hold another item. Currently, this situation is too complex to simulate correctly.
```

One use case is to force hold items during an item text box by performing [Item Smuggle for Arrowless Offset](./material.md#无箭强持),
then get an item text box (similar to performing [Arrowless Offset](./break_slots.md#无箭法)).

```skybook
get 2 shrooms
:smug hold 2 shrooms
# Open a chest, for example
:pause-during get lynel-shield 
# Here, you are in pause screen while holding 2 shrooms
unpause
# Now the 2 shrooms will drop to the ground because of how the smuggle works
```

You can also use this feature to explicitly annotate optimizations for speedruns.

```skybook
:pause-during get zora-armor; equip zora-armor
```

## 性能优化
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


## 细节
- <skyb>get</skyb>, <skyb>pick-up</skyb> and <skyb>buy</skyb> all require [`Overworld`](../user/screen_system.md) screen.
- You cannot get new items while holding items in the overworld
  - with <skyb>:smug</skyb>, the held items will be dropped after getting the item
