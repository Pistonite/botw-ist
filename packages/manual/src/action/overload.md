# Menu Overload

In the game, when spawning more actors then the limit within the actor system,
new actors will fail to be created. This is known as `Menu Overload`.

Since the simulator does not have an actor system like the game,
there are only commands for simulating entering and leaving the overloaded state.

- <skyb>overload</skyb> for triggering overload.
- <skyb>unoverload</skyb> for canceling overload.

## Syntax
> `overload`<br>
> `unoverload`<br>

## Hole Smuggle
If menu is overloaded while closing inventory, the held items won't be spawned
in the overworld. This is known as `Hold Smuggle`.

```skybook
# Get some random items
get 2 apple 2 banana 2 core
# Activate menu overload
overload
# Hold some items and close
hold 1 apple 1 banana 1 core
unpause
# Since we have hold smuggle, we can get items normally
get diamond
```

## Item Transmutation
Hold smuggling can be used to perform Item Transmutation.

```skybook
# Get sacrificial items, overload, and hold 5 of them
get 6 apple; overload hold 5 apple
# Sell the last item in the slot
sell apple
# Get the item you want to transmutate into
get giant-ancient-core

unhold # Now have 6 giant ancient cores!
```

## Breaking Slots
Hold Smuggle can be used to make `Broken Slots`, see [Breaking Slots](./break_slots.md#hold-smuggle-offset).

## Durability Transfer
Menu Overload allows you to switch inventory equipment without switching it in
the overworld. This desynced state allows for transfering durability within
the same type of equipments.

```skybook
# Transfer the durability of Axe to Royal Guard Claymore
get axe royal-guard-claymore
overload
equip royal-guard-claymore
unoverload
use weapon
```

## Details
- <skyb>overload</skyb> can be used in any screen.
- <skyb>unoverload</skyb> requires [`Overworld`](../user/screen_system.md) screen.
  - This is because it is not possible to cancel menu overload while the inventory is open.

