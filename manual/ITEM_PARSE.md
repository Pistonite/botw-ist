Item specifier:

```
ItemListForAdd    -> NumberedItem+
ItemListForRemove -> AllableItem+
ItemListForArea   -> InfableItem+
NumberedItem  -> NUMBER Item
InfableItem   -> NumOrInf Item
AllableItem   -> NumOrAll Item
NumOrInf      -> NUMBER | "infinite"
NumOrAll      -> NUMBER | "all"
Item          -> ItemName ItemMeta?
ItemMeta      -> [ ItemMetaEntry (OpComma ItemMetaEntry)* ]
ItemMetaEntry -> ItemMetaKey (OpAssign ItemMetaValue)?
ItemMetaKey   -> WORD
ItemMetaValue -> WORD | NUMBER | BOOL
ItemName      -> WORD+ | < WORD >

ItemSlot      -> ( OpSlotSpec "slot" NUMBER )

OpSlotSpec    -> "from" | "in" | "to"

OpAssign      -> = | :
OpComma       -> ,
```
