# Prompt Entanglement (PE)

```admonish warning
This glitch is related to the Pouch Screen, which has not been reversed
engineered. Most of the concepts are based on experiments, and may not reflect
the actual implementation in the game.
```

```admonish todo
If you can help improve this page,
please edit [this file](https://github.com/Pistonite/botw-ist/tree/main/packages/manual/src/ist/pe.md)
and open a Pull Request
```

**Prompt Entanglement**, or PE, is a glitch that allows you to apply
a prompt (like "Equip", "Drop", "Hold", etc) to from one item to another item
that is not supposed to have that prompt. For example:

- Holding a Food (only Materials are normally holdable)
- Equipping a Material
- Eating a Key Item

## Invalid Star Tab
```admonish note
IST refers to Inventory Slot Transfer in contexts pertaining Invalid Star Tab.
```
To activate PE, the first step is to activate a state known as **Invalid Star Tab**.
This is a state that allows the cursor (the box that highlights which item is 
currently selected in the inventory) to go to the "Key Items" icon.

Currently, the only known way to activate Invalid Star Tab is by having
items in a category that you have not discovered. For example,
have a material *without picking up any material*.

At first this seems impossible. However, the catch is "picking up" - 
You can obtain items without picking them up with IST. In an Invalid Star Tab
setup, there are 3 general steps:

- Save with the tab you want to use undiscovered
- Pick up an item in that tab, which discovers the tab and gets you the item
- Use IST to transfer that item back into the save you made

Once the setup is done, you can verify Invalid Star Tab is active if any of the following is true:
- When you scroll to the right very quickly, the cursor ends up on the Star.
- When you go to the "System" screen, the cursor remains on the left screen.
  You can only see the cursor when you press "Right", which moves it on to the "Save" button.

## Cursor Glitch
```admonish warning
The Cursor Glitch is not fully understood, since the inventory screen
system is not reversed engineered. This section may contain inaccurate information.
```

When Invalid Star Tab is active, you can now perform the **Cursor Glitch**
to achieve PE. This glitch uses a sequence of controller inputs
to quickly move the cursor while Invalid Star Tab is active to confuse
the inventory code, and puts the cursor on a tab that's not the current
tab you are viewing.

```
      |- you are looking at this page
      v
    MATERIAL                 FOOD
[ ] [ ] [ ] [ ] [ ]  [ ] [ ] [ ] [ ] [ ]
[ ] [ ] [ ] [ ] [ ]  [ ] [ ] [X] [ ] [ ] <- cursor (X) is on another page
[ ] [ ] [ ] [ ] [ ]  [ ] [ ] [ ] [ ] [ ]
[ ] [ ] [ ] [ ] [ ]  [ ] [ ] [ ] [ ] [ ]

                         ^
                         |- (this is where Link's model usually is on screen)
```
Typically, we refer to the position of the cursor by the Row and Column.
In the example above, we say the Cursor Glitch is active at `Row 2, Column 3`,
or simply `Row 2 Column 3 is activated`.

```admonish tip
Since the Cursor Glitch is not fully understood, the community has put
together [a spreadsheet](https://docs.google.com/spreadsheets/d/1j0UM0kIGs74DKkKUNGsDH5LhQIBbDDw9cIxxkcE82P8/edit?gid=0#gid=0) of different input sequences you can use
to active each slot.
```

When Cursor Glitch is active, you can keep it active by move tabs in groups
of 3 (tap right stick right 3 times, or left 3 times), without pausing too long
between them. Pausing while not on a multiple of 3 tabs from where the slot
is activated will reset the cursor's position, losing the glitched state.

## Capturing the Prompt
What the Cursor Glitch enables is that we can now move the cursor to another item,
with the inventory screen still "thinks" we are on the original item,
so it opens the prompt of the original item when we trigger it.

When we trigger the prompt (pressing `A`), the prompt used
comes from the item that is currently showing description on the screen:
```
      |- you are looking at this page
      v
    MATERIAL         
[ ] [ ] [ ] [ ] [ ]  Link is displayed here
[ ] [ ] [ ] [ ] [ ]          [X] <- cursor (X) is on another page
[ ] [ ] [ ] [ ] [ ]  ===================
[ ] [ ] [ ] [ ] [ ]  <The item's name and description is displayed here>
```

When moving tabs in groups of 3, the Cursor Glitch causes the prompt
to be *locked*, meaning it will not update the name/description of the item.
You can force update it by going to the System screen and back (pressing `R` then `L`).
This is often refered to as "resetting" or "capturing" the prompt.

With the prompt locked, you can now move the cursor to 3 tabs left or right,
which will change which item the cursor is on, but will not update the prompt.
Now, you can press `A` to trigger and use the prompt.

In general, these are the steps to do any PE setup:
- Use the input sequence (the Cheat Code) to activate a slot
- Ensure the right prompt is captured by moving tabs in groups of 3,
  and optionally go to the System screen and back to reset the prompt
- Move tabs in groups of 3 to the target item, and use the prompt

```admonish tip
Since you can only keep the glitch by moving tabs in groups of 3,
this means PE can only be used between 2 items that are multiple of 3 tabs apart
(i.e. have 2 other tabs between the source and target items)
```

## Applications

### Weapon Modifier Corruption
With PE, you can hold ingredients like stackable food
that aren't holdable normally. When these ingredients are cooked, their
properties gets processed by the complex cooking system and produces
meal values that aren't obtainable by cooking normal ingredients,
which can give better values for [Weapon Modifier Corruption](./wmc.md).

You can hold stackable food by either using "Hold" prompt from material,
pressing `X` while locked to a material, or using the "Drop" prompt
from equipments. Using the "Drop" prompt will not put Link in the holding
state in inventory screen, allowing you to perform other actions while technically
holding.

```admonish note
Note that if you try to hold items that do not have a model (Food, or some Key Items),
the game will crash when trying to render it in the inventory. You can workaround
this with Super Hold Smuggle or some other method, which we will not go into depth
here.

The simulator allows you to hold anything without crashing.
```

### Removing Arrow Slots and Permanent Items
When you obtain a type of arrow, that arrow slot is stuck in the inventory
forever, even if you shoot the arrow to 0. With PE, you can entangle
a material prompt with the arrow prompt. Now you can press `X` and use
hold to remove arrows. When you have `0` arrows, you can use
the `eat` prompt to remove the slot completely.

Similar to removing arrow slots, you can remove permanent items
like Sheikah Slate, Glider, Zora Armor, etc, by eating it.

### Duplicate Materials
When using PE to hold, it subtracts the amount from the item you are holding,
but checks the amount of the original item to see if you can keep holding.
Since the amount of original item never decreases in the process, you can
keep holding the item even if the stack is at 0. You can either unhold,
or drop the items on the ground to realize the gain.

Note that if you use this method to duplicate material, you need at least 4 tabs
of materials.

### Durability Transfer and Desync Equipment
You can use PE to change equipment, while not changing the equipped status of the slot.
This is very similar to Desyncing with Menu Overload.

To transfer durability:
- Equip the item to *receive* the durability
- Activate the slot with the item to *give* the durability
- Use the "Equip" prompt of that item on something else (for example, a material)
- Close Inventory

This will switch the equipment in the overworld while not in the inventory.
The change equip action will cause a durability update, which transfers the durability.

```admonish tip
Note that unlike durability transfer with Menu Overload, you do not need to use the equipment
to update the durability. This is because the desync acheived by PE is the exact opposite of Menu Overload:

- Menu Overload desyncs by switching the equipment in the Inventory, but not in overworld
- PE desyncs by switching the equipment in the overworld, but not in the inventory

Since with Menu Overload, you do not switch equipment in the overworld, which does
not trigger the change equip action. Therefore, using the equipment manually is required
to update the durability.
```

You can also use this to unequip the One-hit Obliterator, which is more
consistent than using Menu Overload. After performing the steps above,
you will be able to unequip the OHO from the DPad Quick Menu.

