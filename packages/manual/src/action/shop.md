# Shop

Buy or sell items from a shop.

## Syntax
> `talk-to NPC` (can be any word)<br>
> `untalk` OR `close-dialog`<br>
> `sell` [`CONSTRAINED_ITEM_LIST`](../user/syntax.md) <br>
> `buy` [`FINITE_ITEM_LIST`](../user/syntax.md) <br>
> `:same-dialog`<br>

Examples
```skybook
talk-to beedle
sell all shroom all pepper
:same-dialog buy 5 arrows
close-dialog
```

## Buying
In most shops in the game, you buy items by talking to the item
in the overworld, which keeps you in the overworld screen after purchasing
the item. The exceptions are Beedle and travelling merchant, where there
is a dedicated screen for choosing items to buy.

The <skyb>buy</skyb> action defaults to buying in the Overworld. If it matters
in the setup, you can force <skyb>buy</skyb>ing from a dialog by entering
the [`Shop Buying` screen](../user/screen_system.md) manually with 
<skyb>talk-to NPC</skyb>, and <skyb>untalk</skyb> or <skyb>close-dialog</skyb>
afterwards to return to the `Overworld` screen.

To <skyb>buy</skyb> after <skyb>sell</skyb>ing from the same NPC dialog,
you can use the <skyb>:same-dialog</skyb> annotation just before the <skyb>buy</skyb>
command. This will switch from `Shop Selling` screen to `Shop Buying`

## Selling
Unlike buying, selling in this game only has one screen, where you select
items to sell. The screen is very similar to the normal inventory screen, but:

- Only Armor, Material, and Food tabs are displayed
- Items are displayed even when `mCount` is `0`

Items can only be selected to sell from those tabs that are displayed,
and you cannot sell items with the `CannotSell` tag (for example, Zora Armor)



## Detail
- <skyb>buy</skyb> requires [`Overworld` or `Shop Buying` screen](../user/screen_system.md)
  - It will automatically switch to `Overworld` by default. The <skyb>:same-dialog</skyb> annotation
    can be used to switch to `Shop Buying` from `Shop Selling`
- <skyb>sell</skyb> requires `Shop Selling` screen.
- `Buying` and `Selling` screens can be automatically switched between by the simulator even
  if it's manually switched from `Overworld`


