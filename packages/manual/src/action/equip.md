# Equipments

Change or use equipments

## Syntax

> `equip` [`CONTRAINED_ITEM_LIST`](../user/syntax_item.md) <br>
> `unequip` [`CONTRAINED_ITEM_LIST`](../user/syntax_item.md) <br>
> `use <weapon|bow|shield> [X times]` <br>
> `shoot [X times]` <br>
> `throw weapon` <br>
> `display` [`CONTRAINED_ITEM_LIST`](../user/syntax_item.md) <br>

Annotations: [`:non-breaking`](#throwing-the-weapon), [`:breaking`](#throwing-the-weapon)
[`:dpad`](#change-equipments)

## Change equipments
The <skyb>equip</skyb> and <skyb>unequip</skyb> actions are used
to change the equipped status on equipments.

Example
```skybook
# Equip the first weapon
equip weapon 
# Unequip the first **equipped** weapon
unequip weapon
# Equip multiple items
equip 1 royal-claymore 1 hylian-shield
# Unequip multiple items
unequip all shields all bows
# Unequip the second equipped Hylian Shield
unequip hylian-shield[slot=2]

# You can also equip armors and champion abilities
equip champion-tunic
unequip gale
# You cannot unequip arrows
unequip fire-arrow # Error! cannot unequip arrow
```

```admonish warning
When using `from-slot` or `slot` for <skyb>unequip</skyb>,
note that <skyb>unequip</skyb> only targets the **equipped** items.
So `slot=2` means the second **equipped** item. <skyb>equip</skyb>
and other commands target all items, so <skyb>equip weapon[slot=3]</skyb>
equips the third weapon in the inventory, regardless of which weapon
is currently equipped. If the third weapon is already equipped, you will
get an error.

This may seem like a weird design choice, but it makes intuitive sense
when you use the command in most cases.
```

```admonish tip
Normally, you would omit the amount for <skyb>equip</skyb> or use `1` for multiple categories, since equipping another item 
of the same category would
just unequip the previous one. However, in some configurations, the items won't be auto-unequipped.
If you actually want to equip more than one item, you have to specify <skyb>[equipped=false]</skyb>.
Otherwise, it will error when it hits an item that's already equipped.
For example, <skyb>equip all weapons[equipped=false]</skyb>.

<skyb>unequip all</skyb> should always work as expected.
```

By default, changing equipments are assumed to be done in the pause menu. This should be
ok in most cases. However, there are edge cases where action must be done through the DPad
Quick Menu, examples include:
- You are holding items in the pause menu.
- The item slot is not visible in the pause menu, only in quick menu.

In these scenarios, you can use the <skyb>:dpad</skyb> annotation to specify the equipment change should be
done via the quick menu.

Example
```skybook
# Switch equipments with DPad quick menu
:dpad equip fire-arrow
:dpad unequip weapon
```

```admonish warning
Note that <skyb>:dpad unequip</skyb> can only be used to unequip the first equipped item in the quick menu,
and cannot be used to unequip arrows.
```

## Using the equipment
You can use the equipment in the overworld with the <skyb>use</skyb> action,
which can be used to simulate actions that consumes durability:
- Hit object with weapon
- Block attack with shield
- Shield surf
- Shooting arrows (The <skyb>shoot</skyb> command is equivalent to <skyb>use bow</skyb>)

Examples
```skybook
# Attack or hit something with weapon
use weapon
# Block a bomb with shield
use shield 30 times
# Shoot 5 arrows
use bow 5 times
shoot 5 times
```

## Throwing the weapon

## Details
- <skyb>equip</skyb> and <skyb>unequip</skyb> require [`Inventory`](../user/screen_ssytem.md) screen
  - If <skyb>:dpad</skyb> is used, `Overworld` screen is required.
- <skyb>use</skyb>, <skyb>throw</skyb> and <skyb>display</skyb> require `Overworld` screen.

