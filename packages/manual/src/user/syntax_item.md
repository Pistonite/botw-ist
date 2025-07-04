# Item Syntax

The Item Syntax has 3 components: [`amount`](#amount), [`name`](#name), and [`metadata`](#metadata)

```skybook
get    3        pot-lid   [durability=3]
#      ^ amount ^ name    ^ metadata
```

```admonish tip
To specify multiple items in the same command, simply write them one after another,
e.g. <skyb>2 apples 3 bananas</skyb>

When there is only one item in the list, and the amount is `1`, you can omit the amount. For example <skyb>get 1 apple</skyb> can
be shortened to just <skyb>get apple</skyb>. However, amount is required 
when the list contains more than one item name/category
```

The syntax could be used in **3** scenarios, depending on the command:

1. `FINITE_ITEM_LIST`
   - The amount must be a number, not keywords like <skyb>all</skyb>.
   - The name must be an item, not category.
   - The metadata is used to *describe* extra properties of the item.
   - Generally used by commands for *adding* items, such as <skyb>get</skyb>

2. `INFINITE_ITEM_LIST`
   - The amount could be a number or the keyword <skyb>infinite</skyb>.
   - The name could be an item or a category.
   - The metadata is used to *describe* extra properties of the item.
   - Currently, this form is not used by any command.

3. `CONSTRAINED_ITEM_LIST`
   - The amount could be a number, or:
     - The keyword <skyb>all</skyb>
     - In the form <skyb>all but X</skyb>, where `X` is a number.
   - The name could be an item or a category.
   - The metadata is used to *match* from items in some existing list (such as your inventory)
   - Generally used by commands that *targets* some item, such as <skyb>hold</skyb> and <skyb>eat</skyb>.
   - [Position properties](#selecting-from-multiple-matches) can be used


## Amount

The amount of item may have different meaning in different commands. For example,
when using the <skyb>eat</skyb> command, the amount is always the internal value (i.e. the stack size),
since you can eat from corrupted food or decrease armor value/durability by eating. When using <skyb>sell</skyb>,
however, the amount means how many *slots* for unstackable items.

In `CONSTRAINED_ITEM_LIST`, you can use 2 special amount forms: <skyb>all</skyb> and <skyb>all but</skyb>:
- <skyb>all</skyb> will repeatly find the item and perform the action on the item, until the item cannot be found.
- <skyb>all but X</skyb> will first count the total number of times the action can be performed,
  then perform the action `count - X` times. How the total number is counted depends on the command,
  similar to the eat vs sell situation mentioned earlier.

```admonish note
The implementation may vary slightly based on the command, but the concepts are the same.
One notable example is that <skyb>all</skyb> in <skyb>dnp</skyb> is implemented as <skyb>all but 0</skyb>, since
otherwise it will be stuck in an infinite loop.
```

```admonish warning
In rare cases, <skyb>all but</skyb> could be inaccurate, if the total number of items changes unexpectedly due to the action.
Please report if you encounter this issue.
```

## Name

You can specify the name of the item in 4 ways:

1. By `Identifier` - 
   An Item Identifier is multiple english words (`A` to `Z`, case-insensitive), combined with `-` or `_`.
   For example, <skyb>royal-claymore</skyb> and <skyb>trav-bow</skyb> are both valid identifiers.
   There is a fixed algorithm for resolving the identifier to an item. 
   - The result is an item that contains all the individual words, for example <skyb>trav-bow</skyb> results in **trav**eller's **bow**.
   - You can add an effect before a food item to specify the cook effect. For example <skyb>hasty-elixir</skyb>, <skyb>sneaky-wild-greens</skyb>
   - Plural forms with `-s`, `-es`, `-ies` postfixes are supported. They don't affect the amount of the item, only makes
     the command sounds more natural in English.
   - Some shorthands are supported in this form. For example, <skyb>geb</skyb> for <skyb>great-eagle-bow</skyb>, <skyb>aa</skyb> for <skyb>ancient-arrow</skyb>.
2. By `Actor` - 
   You can use angle brackets (`<>`) to specify the internal actor name directly,
   for example <skyb>get <Weapon_Sword_070></skyb>. You cannot specify cook effect in this way.
3. By `Localization` - 
   If English is not your preferred language, you can specify items by their localized name using a quoted-string.
   For example, <skyb>"espadon royal"</skyb> or <skyb>"王族双手剑"</skyb>.
   - The string is fuzzy-searched in all languages
   - To lock the language, prepend the query with the language and a colon, for example, <skyb>"fr:espadon royal"</skyb>
     this could be useful if the query is short, and is matching in another language that you didn't expect.
   - Localized search only applies to items, not commands (like <skyb>get</skyb>, <skyb>hold</skyb>, etc)
4. By `Category` - 
   When selecting items from inventory or some other list of items, you can
   also use a category in the place of the item name to match the first item of that category.
   This can be useful in situations like <skyb>unequip shield</skyb> where you don't need to care
   what shield is currently equipped, or <skyb>pick-up 3 weapons</skyb>,
   where it doesn't matter which weapons are picked up.

```admonish info
  See [token](https://github.com/Pistonite/botw-ist/blob/d5812037f4909eeb48cb2ba666dccdb672563cc4/packages/parser/src/syn/token.rs#L119) for possible category values
```

## Metadata
Metadata specifies extra properties of the item, in the format of 
<skyb>[key1=value1, key2=value2, ...]</skyb>, either `=` or `:` can be used as the key/value delimiter

- In `FINITE_ITEM_LIST`, it is used to specify extra data on the item to be added
  - For example, <skyb>get pot-lid[durability=1]</skyb> gets a new <skyb>pot-lid</skyb> with `1` durability
- In `CONSTRAINED_ITEM_LIST`, it is used to specify extra data used to match the item to operate on.
  - For example, if you are multiple `pot-lids`, <skyb>drop pot-lid[durability=1]</skyb> targets the one with exactly `1` durability.

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
| `modifier` | `modtype` | (`int` or `string`) Set weapon modifier. <br><br>**Cannot be used to set food effect type**. <br><br> Integer values are the same as `price`. String values can be specified multiple times to build up the weapon modifier. <br><br> When used for matching, if only one modifier is specified, it will match any modifier flags that includes the specified one (i.e. other modifiers are allowed), if more than one bit is specified, the modifier flag must match exactly.<br> See [`parse_weapon_modifier_bits`](https://github.com/Pistonite/botw-ist/blob/main/packages/parser/src/cir/item_meta.rs) for possible values |
| `price` | |(`int`) Sets the price of the cooked-food. This can also be used to set multiple weapon modifiers as a bit mask |
| `star` | | (`int`) Armor star (upgrade) number, valid range is `0-4`, inclusive. <br>Note that this is syntactic sugar to change the name of the item, as armor with different star numbers are different items. |
| `time` | | (`int`) Sets the duration of the food effect in seconds |
| `value` | `life` | (`int`) The value of the item, which is the count for stackables or durability multiplied by 100 for equipments. <br>**Note: not to be confused with `life-recover`** |
  
## Selecting from multiple matches
In `CONSTRAINED_ITEM_LIST`, there could be the case where there are multiple items that are exactly the same. There are additional meta properties that you can use
to pick exactly which slot to select.

With `from-slot` property, you can pick the `i`-th matched item. For example,
if there are 3 Pot Lids, you can use <skyb>drop pot-lid[from-slot=2]</skyb> to drop the second Pot Lid. The number is 1-indexed.

You can also target an item by its position in the inventory directly
with one of the following methods:

```skybook
# This is the same as using `from-slot`
# If there are >=2 slots of apple, this will eat from the second slot
eat 2 apple[slot=2]

# Category can be used as the name
# This eats the second slot in the entire inventory that is a material
eat 2 material[slot=2]

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
