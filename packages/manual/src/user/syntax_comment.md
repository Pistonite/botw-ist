# Comments and Notes

Comments and Notes are text in the script that don't affect
the output of the command.

## Comments
Comments are lines that start with `#` or `//`. They are completely ignored.

```skybook
# This is a comment
// This is also a comment
```

```admonish tip
In the script editor, you can use the hotkey `Ctrl + /` to quick toggle
selected lines between commented/uncommented.
```

## Block Literals
Block Literal is a multi-line block that starts and ends with `'''` (triple single-quotes).

```skybook
'''
This is a block literal

It can have multiple lines
'''
```

Addtionally, a block literal can have a `tag`, which is a string after the `'''`
that starts the block. For example, the `note` tag can be used to add
notes to blocks of commands, which can be viewed in the `Notes` extension.

```skybook
'''note
Drop these in the same pile
'''
drop all weapons
drop all shields
```

```admonish info
The `Notes` feature is not implemented yet.
```
