# Entangle

Activate [Prompt Entanglement](../ist/pe.md) and perform actions on a slot using prompts
from another.

## Syntax
> `entangle` [`ITEM`](../user/syntax_item.md)<br>
> `:targeting` [`ITEM`](../user/syntax_item.md)<br>

## Activate PE

Using the Cursor Glitch, you can "activate" slots for Prompt Entanglement (PE).
While the Cursor Glitch is active, you can switch tabs in groups of 3 to keep
the glitch active. Therefore, conceptually, when you activate a slot,
all slots that are 3 tabs apart can be considered activated as well.

This action is simulated by the <skyb>entangle</skyb> command.

```skybook
# Targets the Pot Lid, and activates that slot, as well as all slots that
# are 3 tabs apart
entangle pot-lid
```

While a slot is activated, you will see a "Link" icon next to it.

```admonish tip
If the correspond slot does't exist on a tab (i.e the tab doesn't have enough items),
the Link icon will not display on the slot. However, you can still target
the slot, and usually it will target the first item in that tab.

When in Tabbed view, the Tab Minimap will also display which tabs contain an activated slot,
```

The effect of the activation will last until the inventory is closed. You can also
use another <skyb>entangle</skyb> command to change which slot is activated.

## Targeting an Item
The second step to using PE is to select a target item that will receive the prompt.
The <skyb>:targeting</skyb> annotation is used to do that.

```skybook
# If the item is in an activated slot, you can use the name to select it
:targeting apple
# You can also select the slot directly
# This is useful if you are targeting an empty slot (which can't be selected
# by item name, since there's no item there)
# Note that specifying the first item directly will not work, if the activated
# slot is not in row 1 and col 1.
:targeting apple[category=material, tab=1, row=1, col=3]
```

Finally, in the next command after <skyb>:targeting</skyb>, you can perform
an action on a PE-enabled slot. If the target item can be reached by the item
in the action, the action will be performed on the target item instead.

```skybook
entangle pot-lid
:targeting roasted-endura-carrot
drop pot-lid # will hold the roasted endura carrot
```

The effect of <skyb>:targeting</skyb> will only last until the next command,
but you can use multiple <skyb>:targeting</skyb> within the same <skyb>entangle</skyb>

