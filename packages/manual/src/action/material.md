# Material Operations

Performing actions on materials in the inventory. Some actions
may apply to non-materials.

- <skyb>hold</skyb> command performs the "hold" prompt.
- <skyb>unhold</skyb> command stops holding in inventory, or put away the items in overworld.
- <skyb>drop</skyb> command drops currently-held items, or hold and drop new items.
- <skyb>dnp</skyb> command is a shorthand for <skyb>drop</skyb> and [`pick-up`](./get.md).
- <skyb>eat</skyb> command performs the "eat" prompt.

## Syntax
> `hold` [`CONSTRAINED_ITEM_LIST`](../user/syntax.md#finite-vs-constrained-item-specifier)<br>
> `unhold` <br>
> `drop` <br>
> `drop` [`CONSTRAINED_ITEM_LIST`](../user/syntax.md#finite-vs-constrained-item-specifier)<br>
> `dnp` [`CONSTRAINED_ITEM_LIST`](../user/syntax.md#finite-vs-constrained-item-specifier)<br>
> `eat` [`CONSTRAINED_ITEM_LIST`](../user/syntax.md#finite-vs-constrained-item-specifier)<br>

Annotations: 
  - [`:smug`](#smuggle-state-for-arrowless-offset) - Enable Smuggling for Arrowless Offset

Examples
```skybook
hold apple
hold 2 apple
hold 1 shroom 1 pepper
unhold
:smug hold 1 shroom 1 pepper
unhold
eat all materials all food
dnp 5 weapons
```

## Smuggle State for Arrowless Offset
The <skyb>:smug</skyb> annotation can be used to activate the item smuggle
state required for `Arrowless Offset` for the next <skyb>hold</skyb> command, which is when the held materials are attached
to Link's hand instead of being held in front of him.

To do this in the simulator, put <skyb>:smug</skyb> right before the <skyb>hold</skyb> command.

```skybook
:smug
hold 2 shrooms
# Now you are in Overworld, and held items are attached to Link's hand
```

You can also put <skyb>:smug hold</skyb> on the same line (which sounds like *smuggled*, hehe).

To do this in the game, you need:
- A Shield
- A one-handed Weapon

To perform this:
1. Enable Weapon Smuggle and make sure a shield is equipped
2. Hold the `ZL` button
3. Hold items from up to 5 slots
4. Switch to a one-handed weapon
   - Switch to another one-handed weapon, or to something else and back if you are already equipping a one-handed weapon
5. Jump and let go of `ZL` button, after landing, when the shield is to Link's side,
   unequip the shield

While in this state, you can perform actions which are not normally possible, such as getting
items or talking to NPC. While doing so, the simulator will *delay-drop* the items. This is essential to 
generate offsets. In game, you can do this by either:
- Whistle and perform the action (`Dpad Down > A`) quickly before the items drop
- Pull out Bow and perform the action (`ZR > A`) quickly before the items drop

## Dropping Items

```admonish tip
The <skyb>drop</skyb> is also used for dropping equipments, which has
a slightly different semantic. The description here only applies to materials.
```

When using <skyb>drop</skyb> without any items, it means to drop
whatever is currently being held to the ground.

When using <skyb>drop</skyb> with items, it will attempt to hold the items in up to
groups of 5, and drop them. This may not work as expected in rare cases, like
if you hit `mCount=0` in the middle of dropping, you will no longer be able to hold
more items. In this case, you will get an error.

The <skyb>dnp</skyb> command is equivalent to <skyb>drop</skyb>, then <skyb>pick-up</skyb>
the same items. Note that dropped items will not despawn after <skyb>pick-up</skyb>.


## Detail

- <skyb>hold</skyb> requires [`Inventory` screen](../user/screen_system.md),
  and you can only hold a maximum of 5 items.
- <skyb>drop</skyb> requires [`Overworld` screen](../user/screen_system.md)
  when dropping held items. When a list of items is specified, it may switch
  screens multiple times to facilitate the action.
- Certain actions are not possible when you are holding items.
