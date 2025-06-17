# Command Syntax

The *simulator script* is used to describe the steps to setup IST. The script is made up of *commands*.
Most commands describe one or more *actions* in game, such as getting an item, dropping some items, or equip something.

The commands can be divided into 3 groups:
- **Actions**: These correspond to actions you do in game, such as <skyb>get</skyb>, <skyb>pick-up</skyb> and <skyb>hold</skyb>
- **Annotations**: These commands start with `:` and are used to change the current configuration, such as <skyb>:weapon-slots</skyb>
- **Supercommands**: These command start with `!` and are lower-level commands that can directly interact with the game's state, such as <skyb>!set-inventory</skyb>, <skyb>!set-gdt-flag</skyb>

A command can be broken into multiple lines, and can have optional trailing `;`.

```skybook
# These 2 commands are equivalent
get 1 apple 1 pot-lid 1 hammer;

get
  1 apple
  1 pot-lid
  1 hammer
```

```admonish note
In the simulator, the inventory displayed are the state after executing the command
the cursor is on.
```

## Item Syntax

In general, an "item" in the command refers to the following syntax:

```skybook
get    3        pot-lid   [durability=3]
#      ^ amount ^ name    ^ metadata
```

- `amount` specifies the number of items. For example, the command above gets 3 Pot Lids.
  - When only specifying a single item, the amount can be omitted (<skyb>get pot-lid</skyb> is the same as <skyb>get 1 pot-lid</skyb>).
    However, the amount is required for each item if there are multiple items in the same command
- `name` is the item to get, which can be one of the following formats:
  - *By Identifier*: Multiple english words separated by `-` and `_`, for example
    <skyb> get 1 royal-claymore 1 trav-bow</skyb>.
    The result is an item that contains all the words (for example `trav-bow` results in **trav**eller's **bow**.
    - There is an internal algorithm that decides what item it is if there are multiple matches.
  - *By Localized Name*: A quoted word like <skyb>get "royal claymore"</skyb>. By default, all languages are searched,
    so you can also do something like `"espadon royal"` or `"王族双手剑"`. The item is fuzzy-searched.
    - If the matched language is not what you want, you can also lock the language, for example `"fr:espadon royal"`
  - *By Actor Name*: An angle-bracketed string like <skyb>get <Weapon_Sword_070></skyb>, to specify the item use its internal actor name directly.
- `metadata` is extra properties of the item, in the format of `[key1=value1, key2=value2, ...]`, either `=` or `:` can be used as the key/value delimiter

The metadata can be used in 2 scenarios:
- When *adding* item, it specifies extra properties on the item being added, for example, durability of equipment, weapon modifier, etc.
  This is also referred to as a *finite item specifier*
- When *selecting* items, for example, finding which item in the inventory to <skyb>hold</skyb> or <skyb>sell</skyb>.
  In this case, it specifies extra properties to match on the target item.
  For example, <skyb>eat wild-greens[effect=hasty]</skyb> means 
  eating the specific Wild Greens food with the hasty effect (if there are multiple Wild Greens).
  This is also referred to as a *contrained item specifier*

```admonish warning
The item selection algorithm in V4 is stricter. In V3, if no matching item is found,
it falls back to matching just the item name. In V4, it does not fall back, but skips the item
```

The metadata value can be one of the following types:
- `string` - english words separated by `-` or `_` 
- `int` - an integer in decimal or hex (prefixed with `0x`)
- `float` - a floating point number with 2 integers separated by `.` (the whole number part and the decimal part, both needs to be in decimal).
- `bool` - `true` or `false`. The value can be omitted to indicate `true`, for example, <skyb>drop pot-lid[equipped]</skyb> is the same as <skyb>drop pot-lid[equipped=true]</skyb>

Full list of item metadata properties:

```admonish tip
For links below for possible values, since GitHub does not allow linking
to a symbol, you need to click the link, then search for the symbol in your browser (signed-in user
can also use the symbol list on the right side).
```

| Property | Aliases | Description |
|-|-|-|
| `durability` | `dura` |(`int`) Sets `value` to 100 times the specified number |
| `effect` | | (`int` or `string`) Sets the effect ID for cooked-food. See [parse_cook_effect](https://github.com/Pistonite/botw-ist/blob/main/packages/parser/src/cir/item_meta.rs) for possible values |
| `equipped` |`equip` | (`bool`) If the item is equipped |
| `life-recover`| `hp`, `modpower` | (`int`) Sets the number of quarter-hearts cooked-food recovers, or value of a weapon modifier |
| `modifier` | `modtype` | (`int` or `string`) Set weapon modifier. **Cannot be used to set food effect type**. Integer values are the same as `price`. String values set a single modifier. See [`parse_weapon_modifier_bits`](https://github.com/Pistonite/botw-ist/blob/main/packages/parser/src/cir/item_meta.rs) for possible values |
| `price` | |(`int`) Sets the price of the cooked-food. This can also be used to set multiple weapon modifiers |
| `time` | | (`int`) Sets the duration of the food effect in seconds |
| `value` | `life` | (`int`) The value of the item, which is the count for stackables or durability multiplied by 100 for equipments. **Note: not to be confused with `life-recover`** |
  
