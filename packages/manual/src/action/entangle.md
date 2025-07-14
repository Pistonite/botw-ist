# Entangle

Activate [Prompt Entanglement](../ist/pe.md) and perform actions on a slot using prompts
from another.

- <skyb>entangle</skyb> activates PE and sets the target item
- <skyb>:targeting</skyb> changes the target item, or allow you to target empty slots

```admonish warning
PE is not 100% accurate
```

## Syntax
> `entangle` [`ITEM`](../user/syntax_item.md)<br>
> `:targeting` [`ITEM`](../user/syntax_item.md)<br>

## Activate PE
While the [Cursor Glitch](../ist/pe.md#cursor-glitch) is active, you can switch tabs in groups of 3 to keep
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
If a slot that's supposed to be activated does't exist in a tab
(i.e the tab doesn't have enough items), there will be a phantom
slot displayed in that location when in Tabbed View.
This is only a visual effect of the simulator.
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
:targeting <empty>[category=material, tab=1, row=1, col=3]
#          ^ the name is ignored while targeting a slot directly, so it
#            doesn't matter what you put here
```

```admonish warning
<skyb>:targeting</skyb> currently also searches slots that are not activated.
If there are multiple matches, you might need to use a [Position Property](../user/syntax_item.md#selecting-from-multiple-matches)
to specify the activated slot.
```

Finally, in the next command after <skyb>:targeting</skyb>, you can perform
an action on a PE-enabled slot. If the target item can be reached by the item
in the action, the action will be performed on the target item instead.

```skybook
entangle roasted-endura-carrot :targeting roasted-endura-carrot
drop pot-lid # will hold the roasted endura carrot
```

Since it can be redundant to activate, then target the same item,
<skyb>entangle</skyb> will also target the item by default.
The command above can be shortened as
```skybook
entangle roasted-endura-carrot
drop pot-lid # will hold the roasted endura carrot
```

However, sometimes it might be cleared to write it as <skyb>entangle</skyb> then <skyb>:targeting</skyb>.
For example, during speedrun, it's usually faster to <skyb>entangle</skyb>
the source item, to skip resetting the prompt, which means the item to setup
the <skyb>entangle</skyb> is different from the item to target.
However, it's up to your preference how to write the command.

The effect of <skyb>:targeting</skyb> will only last until the next command,
but you can use multiple <skyb>:targeting</skyb> within the same <skyb>entangle</skyb>

