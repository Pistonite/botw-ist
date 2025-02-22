# IST Primer

## What is IST
IST stands for Inventory Slot Transfer. It is a glitch in Breath of the Wild
that exploits behavior of the inventory when the number of the items in the inventory tracked by the game
is less than the number of items actually in the inventory.

The developers made sure that these two values are kept in sync during normal
gameplay. However, in very specific scenarios, the game removes the item
slot from the inventory while subtracting the number of items twice,
resulting in the game tracking 1 fewer item slots in the inventory.
By repeating the action, we can make the game track fewer and fewer items.

The difference between the number of items in the inventory and number
tracked by the game is called **Offset** or number of **Broken Slots**.
"Offset" is technically more correct, but because it's ambiguous in some contexts,
this manual will refer to this number as "Broken Slots".
The action to create the Broken Slots is referred to as **Breaking Slots**.

```admonish note
**Technical Detail**

The inventory is stored as a doubly-linked list. The linked list implementation
stores the length of the list separately as it's inefficient to calculate
the length of a linked list. This variable is often referred to as `mCount`,
which is its name in the decompilation project. This is the number of items
the game tracks. The actual number of items in the inventory is the number
of elements in the linked list of items.
```

```admonish info
When considering the inventory items as a list, the left-most item
in the inventory (in the left-most tab) is the first item. Then
the list follows row-major order (i.e. the item to the right of the left-most
item is the second in the list; the first item in the second row is after
the last item in the first row). Empty spaces and empty tabs in the inventory
do not count. For example, if the tabs are "Weapon" followed by "Bow",
the first bow is directly after the last weapon in the list. The spaces
(unoccupied and unupgraded slots) in the "Weapon" tab does not affect
how the items are stored.
```

## Why is it called IST
