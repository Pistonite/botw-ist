// This has RS extension since that usually gives a minimal syntax highlighting.
//This is not an actual RS file

x!{ SKYBOOK RUNTIME SNAPSHOT V1

=====

----- Step[0]: get 5 apples 2 shroom 1 banana 2 pepper

game: (Running)
  screen: (Overworld)
  pouch: (count=4, are_tabs_valid=true, num_tabs=1, )
    items: (len=4, )
      [000]: (actor_name=Item_Fruit_A, value=5, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Item_Fruit_H, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Item_Fruit_I, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=3, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )

----- Step[1]: :smug

<same>
----- Step[2]: hold 2 apples

game: (Running)
  screen: (Overworld)
  pouch: (count=4, are_tabs_valid=true, num_tabs=1, )
    items: (len=4, )
      [000]: (actor_name=Item_Fruit_A, value=3, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        holding: 2
      [001]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Item_Fruit_H, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Item_Fruit_I, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=3, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  overworld: (len=2, )
      [000]: (typ=Held, actor=Item_Fruit_A, )
      [001]: (typ=Held, actor=Item_Fruit_A, )

----- Step[3]: sell all apples

game: (Running)
  screen: (Shop)
  pouch: (count=4, are_tabs_valid=true, num_tabs=1, )
    items: (len=4, )
      [000]: (actor_name=Item_Fruit_A, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        in_inventory: false
        holding: 2
      [001]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Item_Fruit_H, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Item_Fruit_I, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=3, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  overworld: (len=2, )
      [000]: (typ=Held, actor=Item_Fruit_A, )
      [001]: (typ=Held, actor=Item_Fruit_A, )

----- Step[4]: close-dialog

game: (Running)
  screen: (Overworld)
  pouch: (count=2, are_tabs_valid=true, num_tabs=1, )
    items: (len=3, )
      [000]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222200068, next=0x0000002222243a38, )
      [001]: (actor_name=Item_Fruit_H, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [002]: (actor_name=Item_Fruit_I, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  overworld: (len=2, )
      [000]: (typ=GroundItem, actor=Item_Fruit_A, despawning=false, )
      [001]: (typ=GroundItem, actor=Item_Fruit_A, despawning=false, )

----- Step[5]: drop all banana 1 pepper

game: (Running)
  screen: (Overworld)
  pouch: (count=1, are_tabs_valid=true, num_tabs=1, )
    items: (len=2, )
      [000]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222200068, next=0x00000022222437a0, )
      [001]: (actor_name=Item_Fruit_I, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243cd0, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  overworld: (len=4, )
      [000]: (typ=GroundItem, actor=Item_Fruit_A, despawning=false, )
      [001]: (typ=GroundItem, actor=Item_Fruit_A, despawning=false, )
      [002]: (typ=GroundItem, actor=Item_Fruit_H, despawning=false, )
      [003]: (typ=GroundItem, actor=Item_Fruit_I, despawning=false, )

----- Step[6]: !break 2 slots

game: (Running)
  screen: (Overworld)
  pouch: (count=-1, are_tabs_valid=true, num_tabs=1, )
    items: (len=2, )
      [000]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222200068, next=0x00000022222437a0, )
      [001]: (actor_name=Item_Fruit_I, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243cd0, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  overworld: (len=4, )
      [000]: (typ=GroundItem, actor=Item_Fruit_A, despawning=false, )
      [001]: (typ=GroundItem, actor=Item_Fruit_A, despawning=false, )
      [002]: (typ=GroundItem, actor=Item_Fruit_H, despawning=false, )
      [003]: (typ=GroundItem, actor=Item_Fruit_I, despawning=false, )

}
