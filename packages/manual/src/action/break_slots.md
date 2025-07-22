# Break Slots

**Breaking Slots** refers to the action of generating offsets
to enable IST. See [Inventory Slot Transfer](../ist/index.md) for more info.

You can either break slots by simulating actions, like what you do in the game,
or use the <skyb>!break</skyb> supercommand to directly edit the memory.

## Arrowless Offset

```admonish info
References for commands used for Arrowless Offset:
- [`:smug hold`](./material.md#smuggle-state-for-arrowless-offset)
- [`sell`](./sell.md)
```

The most commonly used method of breaking slots is known as `Arrowless Offset`,
which requires a shield, a one-handed weapon and a shop keeper and can break up to 5 slots at once:

- Enter the [`Smuggle State`](./material.md#smuggle-state-for-arrowless-offset)
- Talk to a shop keeper (by pressing `Dpad Down > A` or `ZR > A` quickly)
- Sell all the items from slots that are being held
- Close the dialog

Example script for Arrowless Offset in the simulator:

```skybook
get 2 shroom 2 pepper 1 banana
:smug hold 1 shroom 1 pepper
sell all shroom all pepper
close-dialog
```

## Hold Smuggle Offset

```admonish todo
Menu Overload functionality is WIP.
```

## Fairy Offset
You can use fairies to break slots by using the last fairy while holding one.

```skybook
hold all but 1 fairy;
use fairy; # by bombing yourself, or stand on fire, etc...
drop; # drop the held fairy
```

## By Magic
See [Low Level Operations](./low_level.md#generate-broken-slots).
