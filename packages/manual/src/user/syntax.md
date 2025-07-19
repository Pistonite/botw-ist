# Command Syntax

The *simulator script* is used to describe the steps to setup IST. The script is made up of *commands*.
Most commands describe one or more *actions* in game, such as getting an item, dropping some items, or equip something.

The commands can be divided into 3 groups:
- **Actions**: These correspond to actions you do in game, such as <skyb>get</skyb>, <skyb>pick-up</skyb> and <skyb>hold</skyb>
- **Annotations**: These commands start with `:` and are used to change the current configuration, such as <skyb>:slots</skyb>
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

In general, the syntax is case sensitive. Although some features like item search is case-insensitive,
it's recommended to keep everything lower-case, unless upper-case is needed (for example
when specifying actor name or GDT flag name).

```admonish note
In the simulator, the inventory displayed are the state after executing the command
the cursor is on.

The simulator parses the commands by span, not by line. You can view the state
for each command even if multiple of them are on the same line.
```

## Item Syntax
Item syntax is used to specify items for commands like <skyb>get</skyb> or <skyb>drop</skyb>.
See [Item Syntax](./syntax_item.md).

## Meta Syntax
The meta syntax is a versatile syntax used to specify additional contextual metadata,
in the form of ordered key-value pairs, the syntax is:

```skybook
[key1=value1, key2=value2, ...]
# `:` and `=` are interchangeable
[key1:value1, key2:value2, ...]
```

Generally, `key`s are `kebab-case` words, and `value`s can be one of:
- `bool` - either the keyword `true` or `false`.
  - `true` can be omitted, i.e. <skyb>[equip]</skyb> is the same as <skyb>[equip=true]</skyb>
- `integer` - an integer in decimal, or hex prefixed with `0x`, like `10` or `0xa`.
- `float` - a floating point number in decimal, like `1.2` (scientific notation not supported).
- `words` - one or more words consisted of alphabetical characters, `-` and `_`, with spaces allowed in between,
  like `hello my-world`
- `quoted` - a quoted string where any character is allowed `"你好世界"`.
- `angled` - like `words`, but surrounded by `<` and `>` and no spaces are allowed,
  like `<Foo>`.

```admonish tip
Generally, the 3 string formats are all accept and can be interchangeable.
In some cases however, the formats can have different meanings.
```
