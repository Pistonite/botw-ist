# Command Syntax

The *simulator script* is used to describe the steps to setup IST. The script is made up of *commands*.
Most commands describe one or more *actions* in game, such as getting an item, dropping some items, or equip something.

The commands can be divided into 3 groups:
- **Actions**: These correspond to actions you do in game, such as <skyb>get</skyb>, <skyb>pick-up</skyb> and <skyb>hold</skyb>
- **Annotations**: These commands start with `:` and are used to change the current configuration, such as <skyb>:weapon-slots</skyb>
- **Supercommands**: These command start with `!` and are more powerful than the actions.
  They often interact directly with the game's state in a way that's not possible with a regular action.

Whitespaces are insignificant in the syntax, including new lines.
This means one command can be broken into multiple lines and more than one command
can be put on the same line.
Commands can also have an optional trailing `;`.

```skybook
# These 2 commands are equivalent
get 1 apple 1 pot-lid 1 hammer;

get
  1 apple
  1 pot-lid
  1 hammer

# Trailing ; is optional even for multiple commands on the same line
hold 2 apples drop
# but it's clearer if you separate them with a ;
hold 2 apples; drop
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
  - However, the amount is required for each item if there are multiple items in the same command. For example <skyb>get apple banana</skyb> is invalid, and it must be <skyb>get 1 apple 1 banana</skyb>
  - You can use `all` in some commands, see [below](#the-all-amount-specifier)
- `name` is the item to get, which can be one of the following formats:
  - *By Identifier*: Multiple english words separated by `-` and `_`, for example
    <skyb> get 1 royal-claymore 1 trav-bow</skyb>.
    The result is an item that contains all the words (for example `trav-bow` results in **trav**eller's **bow**.
    - Plural forms can be used as well, i.e. `apples` is the same as `apple`
    - There is an internal algorithm that decides what item it is if there are multiple matches.
  - *By Localized Name*: A quoted word like <skyb>get "royal claymore"</skyb>. By default, all languages are searched,
    so you can also do something like <skyb>"espadon royal"</skyb> or <skyb>"王族双手剑"</skyb>. The item is fuzzy-searched.
    - If the matched language is not what you want, you can also lock the language, for example <skyb>"fr:espadon royal"</skyb>
  - *By Actor Name*: An angle-bracketed string like <skyb>get <Weapon_Sword_070></skyb>, to specify the item use its internal actor name directly.
  - *By Category* (Limited Scenarios Only): In cases where a category can uniquely identify an item, you can use the 
    category name to specify the item. For example <skyb>unequip shield</skyb> to unequip the currently equipped shield if there is only one shield equipped.
    Note the behavior might vary based on the command. See [token](https://github.com/Pistonite/botw-ist/blob/d5812037f4909eeb48cb2ba666dccdb672563cc4/packages/parser/src/syn/token.rs#L119) for possible category values
- `metadata` is extra properties of the item, in the format of <skyb>[key1=value1, key2=value2, ...]</skyb>, either `=` or `:` can be used as the key/value delimiter


## Finite vs Constrained Item Specifier
The item metadata has different meaning in 2 scenarios:
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
| `ingr` | | (`string`) Set the ingredient of the cooked-food. The string must be an item identifier (see above). The property can be specified multiple times to add multiple ingredients. |
| `level`| | (`int`) Sets the level of the effect for cooked-food |
| `life-recover`| `hp`, `modpower` | (`int`) Sets the number of quarter-hearts cooked-food recovers, or value of a weapon modifier |
| `modifier` | `modtype` | (`int` or `string`) Set weapon modifier. <br>**Cannot be used to set food effect type**. <br> Integer values are the same as `price`. String values can be specified multiple times to build up the weapon modifier. See [`parse_weapon_modifier_bits`](https://github.com/Pistonite/botw-ist/blob/main/packages/parser/src/cir/item_meta.rs) for possible values |
| `price` | |(`int`) Sets the price of the cooked-food. This can also be used to set multiple weapon modifiers as a bit mask |
| `star` | | (`int`) Armor star (upgrade) number, valid range is `0-4`, inclusive. <br>Note that this is syntactic sugar to change the name of the item, as armor with different star numbers are different items. |
| `time` | | (`int`) Sets the duration of the food effect in seconds |
| `value` | `life` | (`int`) The value of the item, which is the count for stackables or durability multiplied by 100 for equipments. <br>**Note: not to be confused with `life-recover`** |
  
## Selecting from multiple matches
When selecting an item slot (for example to <skyb>hold</skyb>), there could be cases
where there are multiple items that are exactly the same. There are additional meta properties that you can use
to pick exactly which slot to select.

With `from-slot` property, you can pick the `i`-th matched item. For example,
if there are 3 Pot Lids, you can use <skyb>drop pot-lid[from-slot=2]</skyb> to drop the second Pot Lid. The number is 1-indexed.

You can also target an item by its position in the inventory directly
with one of the following methods:

```skybook
# This is the same as using `from-slot`
# If there are >=2 slots of apple, this will eat from the second slot
eat 2 apple[slot=2]

# Eat 2 apples from the material tab, in the first row and second column
# When using `category`, the indices are 1-indexed
eat 2 apple[category=material, row=1, col=2]

# Eat 2 apples from the second material tab, in the first row and second column
eat 2 apple[category=material, tab=2, row=1, col=2]

# Eat 2 apples from the second material tab, in the 0-th slot.
# The tab is 1-indexed.
# The slot is the 0-indexed slot in that tab, arranged like this:
# 00 01 02 03 04
# 05 06 07 08 09
# 10 11 12 13 14
# 15 16 17 18 19
eat 2 apple[category=material, tab=2, slot=0]

# Eat 2 apples from the 0-th tab, in the 3rd slot
# The tab index here is the 0-based index in the entire tab array
# The slot is the 0-indexed slot in that tab, see above
eat 2 apple[tab=0, slot=3]

```

```admonish note
- These properties cannot be used when adding items
- If the slot selected by position has a different item, you will receive an error
- When using `row` and `col`, they must be specified after `category` or `tab`
```

```admonish warning
The positions are calculated right before the simulator
tries to find the item to target. This means if the action performed on
previous items in the same command changes the inventory, the position
you need to specify to target the correct item could be different from
what you see in the inventory in the previous step. For this reason,
it's not recommended to specify position when performing an action on multiple
items. Separate the position-dependent action to its own command instead.
```

## The `all` amount specifier
`all` is a special keyword that can be used in place of an amount to mean "all of the item"

```skybook
eat all apples
drop all shields
```

The exact action may depend on the command, for example, <skyb>sell all apples</skyb>
invokes the function for selling with the amount equal to the value of the apple stack,
whereas
<skyb>eat all apples</skyb> selects the "eat"
option for all slots that match `apples` repeatly, until no more slots match (since you can
only eat one at a time).
