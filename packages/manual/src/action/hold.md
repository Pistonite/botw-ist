# Hold Items

Hold a list of items from the inventory screen.

Yellow border and circles on an inventory slot indicate items are being held from that slot,
The number of circles corresponds to how many items are being held.

When Link is also holding items in the overworld, the held items will be also displayed
in the overworld UI with yellow border and a circle.

## Syntax
> `hold` [`CONSTRAINED_ITEM_LIST`](../user/syntax.md)<br>
> `hold-attach` [`CONSTRAINED_ITEM_LIST`](../user/syntax.md) - see [below](#smuggle-state-for-arrowless-offset) <br>
> `unhold`

Examples
```skybook
hold apple
hold 2 apple
hold 1 shroom 1 pepper
hold-attach 1 shroom 1 pepper
unhold
```

## Smuggle State for Arrowless Offset
The <skyb>hold-attach</skyb> action can be used to activate the item smuggle
state required for `Arrowless Offset`, which is when the held materials are attached
to Link's hand instead of being held in front of him.

To do this in the game, you need:
- A Shield
- A one-handed Weapon

To perform this:
1. Enable Weapon Smuggle and make sure a shield is equipped
2. Hold the `ZL` button
3. Hold items from up to 5 slots
4. Switch to a one-handed weapon
   - Switch to another or to something else and back if you are already equipping a one-handed weapon
5. Jump and let go of `ZL` button, after landing, when the shield is to Link's side,
   unequip the shield

While in this state, you can perform actions which are not normally possible, such as getting
items or talking to NPC. While doing so, the simulator will *delay-drop* the items. This is essential to 
generate offsets. In game, you can do this by either:
- Whistle and perform the action (`Dpad Down > A`) quickly before the items drop
- Pull out Bow and perform the action (`ZR > A`) quickly before the items drop

## Stop holding
<skyb>unhold</skyb> puts the items currently held back to the inventory

## Detail
- <skyb>hold</skyb> and <skyb>hold-attach</skyb> require [`Inventory` screen](../user/screen_system.md)
- <skyb>unhold</skyb> requires either `Overworld` or `Inventory` screen,
  and will automatically switch to `Overworld` if not satisfied
- <skyb>hold</skyb> will not work if you are already holding 5 items
- When holding, the held items will only be spawned in the overworld when the inventory is closed.

