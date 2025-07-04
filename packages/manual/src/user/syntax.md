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

The simulator parses the commands by span, not by line. You can view the state
for each command even if multiple of them are on the same line.
```
