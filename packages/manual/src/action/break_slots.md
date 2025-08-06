# Break Slots

**Breaking Slots** refers to the action of generating offsets
to enable IST. See [Inventory Slot Transfer](../ist/index.md) for more info.

You can either break slots by simulating actions, like what you do in the game,
or use the [<skyb>!break</skyb> supercommand](./low_level.md#generate-broken-slots) to directly edit the memory.

## Arrowless Offset

```admonish info
References for commands used for Arrowless Offset:
- [`:smug hold`](./material.md#smuggle-state-for-arrowless-offset)
- [`sell`](./sell.md)
```

The most commonly used method of breaking slots is known as `Arrowless Offset`,
which requires a shield, a one-handed weapon and a shop keeper and can break up to 5 slots at once:

- Enter the [`Arrowless Smuggle`](./material.md#smuggle-state-for-arrowless-offset) state.
- Talk to a shop keeper (by pressing `Dpad Down > A` or `ZR > A` quickly).
- Sell all the items from slots that are being held.
- Close the dialog.

Example script for Arrowless Offset in the simulator:

```skybook
get 2 shroom 2 pepper 1 banana
:smug hold shroom pepper
sell all materials[held]
close-dialog
```

## Hold Smuggle Offset

Hold Smuggle Offset is similar to Arrowless Offset, by selling smuggled items
and dropping them after.


- Activate Hold Smuggle (By Menu Overloading, for example with Shock Arrows).
- Sell the items that are smuggled.
- Hold another item, to cancel the smuggle.
- Drop held items.

Since it is required to hold another item after selling, this method can only make
up to 4 Broken Slots at a time.

```skybook
get 2 shroom 2 pepper 1 banana
overload
hold shroom pepper
unoverload
sell all materials[held]
close-dialog
hold banana
drop
```

```admonish tip
Other than selling the held slots, you can also do this by entering
a trial, such as Trial of the Sword. This will remove the slot in the inventory.
Now you can get a new item, hold it to cancel hold smuggle, and drop the items.
Note that since getting a new item will take out one held slot out, you can
also only make up to 4 slots at a time with this method.

This is also how IST was initially discovered.
```

## Fairy Offset
You can use fairies to break slots by using the last fairy while holding one.

Example script:

```skybook
hold all but 1 fairy;
use fairy; # by bombing yourself, or stand on fire, etc...
drop; # drop the held fairy
```

## Eat and Hold Offset
With [Prompt Entanglement](../ist/pe.md), you can eat and hold the same slot
to make offsets.

- Eat all of the material in the slot you are using PE with
  - Since targeting a translucent slot with PE will target the first slot,
    you need to either make sure you are eating the first slot,
    or eat the slot you are using AND the first slot.
- Use a "drop" prompt to hold the slot you ate.
- Unpause and drop the items in your hand.

The example script below uses the 3rd slot in a tab (<skyb>shroom</skyb>)
so it also needs to eat all of the first slot (<skyb>apple</skyb>).

```skybook
:discovered [bow, shield] # spacing for PE
get
  1 torch 1 axe 1 hammer
  1 apple 1 banana 1 shroom
eat all apple all shroom
entangle hammer
:targeting <empty>[category=material, row=1, col=3]
drop hammer
drop
```
