# Game Flags

Change flag values in GameData (GDT), such as number of upgrade slots,
if a tab is discovered, and quest flags.

- <skyb>:slots</skyb> can be used to change the flag value
  for how many slots are available for Weapons, Bows and Shields
- <skyb>:discovered</skyb> can be used to change which tabs
  are discovered

Moreover, <skyb>!set-gdt</skyb> can be used to set *any* GDT flag.

## Syntax
> `:slots [CATEGORY=NUM]` <br>
> `:discovered [CATEGORY=true|false]` <br>
> `!set-gdt <FLAG>[GDT_META]` <br>

## Number of Slots (i.e. Hestu Upgrade)

<skyb>:slots</skyb> or <skyb>:slot</skyb> sets these GDT flags
`WeaponPorchStockNum`, `BowPorchStockNum` and `ShieldPorchStockNum`,
according to the category that is specified.

Examples:
```skybook
# sets the number of weapon slots to 20 (singular/plural forms are both accepted)
:slots [weapon=20]
# sets the number of weapon slots to 10, and number of bow slots to 8
:slots [weapons=10, bows=8]
# you will get an error if the number is out of range
:slots [weapons=21, bows=4, shields=3]
# Ranges: (inclusive)
# Weapon: 8-20
# Bow: 5-14
# Shields: 4-20
```

## Discovered Tabs
<skyb>:discovered</skyb> edits the `IsOpenItemCategory` flag array.
The category is parsed in the same way as [item categories](../user/syntax_item.md#name).
With a few minor differences:
- `arrow` and `arrows` are allowed, and they are the same as `bow`/`bows`
- Specific armor subtype (Upper/Lower/Head) is not allowed, use `armor`/`armors` instead

You can turn a tab to discovered with `true` and to undiscovered with `false`,
unspecified tabs are unchanged.

Examples
```skybook
# Discover the weapon, bow, shield tabs
:discovered [weapon, bow, shield]
# Undiscover the armor tab
:discovered [armor=false]
```

```admonish note
When changing a tab from undiscovered to discovered, the inventory will not automatically
update until it is changed (i.e. when `updateInventoryInfo()` is called again)
```

## Any Flag
<skyb>!set-gdt</skyb> can set any flag by name.

The name is specified with angled brackets (e.g. `<PorchItem>`), to indicate the value should
be interpreted literally, just like with angled-bracketed item names.

The meta is where you specify the value. First, you need to specify one of the property keys:
- `bool`
- `s32` (alias: `i32`)
- `f32`
- `str32` (alias: `string32`)
- `str64` (alias: `string64`)
- `str256` (alias: `string256`)
- `vec2f` (alias: `vector2f`)
- `vec3f` (alias: `vector3f`)

For non-vector types, the meta value is the value to set. Examples:
```skybook
# Make Master Sword stay in True Form
!set-gdt <Open_MasterSword_FullPower>[bool=true]
# Sets Master Sword cool down timer
!set-gdt <MasterSwordRecoverTime>[f32=10.0]
```

For array values, use the `index` property to specify the array index (alias: `i` or `idx`)
```skybook
# Set the first item in GDT to Travel Medallion
# String values work without quotes if it only contains alphabetical characters and _ or -
!set-gdt <PorchItem>[str64="Obj_WrapDLC", i=0]
# Set the value of the 20-th item in GDT (0-indexed) to 1000
!set-gdt <PorchItem_Value1>[s32=1000, idx=20]
# Set the modifier value of the 10-th shield in GDT (0-indexed) to 1000
!set-gdt <PorchShield_ValueSp>[s32=1000, idx=20]
```

For vector values, instead of specifying the value with the `vec2f` or `vec3f` key,
use `x`, `y`, `z` keys to specify each component. Only the components that
are specified will be changed

```skybook
# Set the effect level of the first food to 3, leaving effect id unchanged
!set-gdt <CookEffect0>[vec2f, i=0, y=3]
# Set the sell price of the second food to 101, and the unused y value to 500
!set-gdt <CookEffect1>[vec2f, i=1, x=101, y=500.0]
```

```admonish note
Integer values for vector components are automatically converted to 32-bit IEEE-754 floats
```
