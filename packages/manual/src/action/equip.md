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
to equip and unequip equipments.

Note that <skyb>unequip</skyb> will only target equipped weapons
(effectively adding <skyb>[equipped=true]</skyb> meta property),
and cannot be used to unequip arrows.

Example
```skybook
# Equip the first weapon
equip weapon 
# Unequip the *equipped* weapon
unequip weapon
# Equip multiple items
equip 1 royal-claymore 1 hylian-shield
# Unequip multiple items
unequip all shield all bow
# Unequip a specific item with multiple matches
unequip hylian-shield[slot=2]
```

Changing equipments from pause screen and DPad quick menu
has no difference except for a few edge cases. For example,
when the weapon is only visible in one menu but not the other.
By default, the pause screen is used. You can use the <skyb>:dpad</skyb>
annotation to force the DPad to be used. Using DPad quick menu requires `Overworld` screen.

Example
```skybook
# Switch equipments with DPad quick menu
:dpad equip fire-arrow
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

