# Break Slots

**Breaking Slots** refers to the action of generating offsets
to enable IST. See [Inventory Slot Transfer](../ist/index.md) for more info.

You can either break slots by simulating actions, like what you do in the game,
or use the <skyb>!break</skyb> supercommand to directly edit the memory

## Syntax
> `!break X slots`

## Arrowless Offset

```admonish info
References for commands used for Arrowless Offset:
- [`hold-attach`](./hold.md)
- [`sell`](./shop.md)
```

The most commonly used method of breaking slots is known as `Arrowless Offset`.
which requires a shield, a one-handed weapon and a shop keeper and can break up to 5 slots at once:

- Enter the [`Smuggle State`](./hold.md#smuggle-state-for-arrowless-offset)
- Talk to a shop keeper (by pressing `Dpad Down > A` or `ZR > A` quickly)
- Sell all the items from slots that are being held
- Close the dialog

Example script for Arrowless Offset in the simulator
```skybook
get 2 shroom 2 pepper 1 banana
hold-attach 1 shroom 1 pepper
sell all shroom all pepper
close-dialog
```

## Hold Smuggle Offset
TODO

(something like:
```skybook
get 2 shroom 2 pepper 1 banana
overload
hold 1 shroom 1 pepper
sell all shroom all pepper
stop-overload
hold banana
drop
```

## By Magic
For backward compatibility, you can still use <skyb>!break</skyb>
to generate offsets by directly editing the memory of the inventory.
This shouldn't cause any inaccuracies in normal circumstances, but
it's recommended to only use this command for prototyping, and use
the actual actions in the final script.

Example:
```skybook
!break 20 slots
```
