// This has RS extension since that usually gives a minimal syntax highlighting.
//This is not an actual RS file

x!{ SKYBOOK RUNTIME SNAPSHOT V1

=====

----- Step[0]: get 2 shroom

game: (Running)
  screen: (Overworld)
  pouch: (count=1, are_tabs_valid=true, num_tabs=1, holding_in_inventory=false, )
    items: (len=1, )
      [000]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )

----- Step[1]: :smug

<same>
----- Step[2]: hold 2 shroom

game: (Running)
  screen: (Overworld)
  pouch: (count=1, are_tabs_valid=true, num_tabs=1, holding_in_inventory=true, )
    items: (len=1, )
      [000]: (actor_name=Item_Mushroom_E, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222200068, )
        holding: 2
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  overworld: (len=2, )
      [000]: (typ=Held, actor=Item_Mushroom_E, )
      [001]: (typ=Held, actor=Item_Mushroom_E, )

----- Step[3]: :item-box-pause

<same>
----- Step[4]: get apple

game: (Running)
  screen: (Inventory)
  pouch: (count=2, are_tabs_valid=true, num_tabs=1, holding_in_inventory=true, )
    items: (len=2, )
      [000]: (actor_name=Item_Mushroom_E, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        holding: 2
      [001]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  overworld: (len=2, )
      [000]: (typ=Held, actor=Item_Mushroom_E, )
      [001]: (typ=Held, actor=Item_Mushroom_E, )

----- Step[5]: unhold

game: (Running)
  screen: (Inventory)
  pouch: (count=2, are_tabs_valid=true, num_tabs=1, holding_in_inventory=false, )
    items: (len=2, )
      [000]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )

----- Step[6]: unpause

game: (Running)
  screen: (Overworld)
  pouch: (count=2, are_tabs_valid=true, num_tabs=1, holding_in_inventory=false, )
    items: (len=2, )
      [000]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )

----- Step[7]: :smug

<same>
----- Step[8]: hold 2 shroom

game: (Running)
  screen: (Overworld)
  pouch: (count=2, are_tabs_valid=true, num_tabs=1, holding_in_inventory=true, )
    items: (len=2, )
      [000]: (actor_name=Item_Mushroom_E, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        holding: 2
      [001]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  overworld: (len=2, )
      [000]: (typ=Held, actor=Item_Mushroom_E, )
      [001]: (typ=Held, actor=Item_Mushroom_E, )

----- Step[9]: :item-box-pause

<same>
----- Step[10]: get pepper

game: (Running)
  screen: (Inventory)
  pouch: (count=3, are_tabs_valid=true, num_tabs=1, holding_in_inventory=true, )
    items: (len=3, )
      [000]: (actor_name=Item_Mushroom_E, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        holding: 2
      [001]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Item_Fruit_I, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  overworld: (len=2, )
      [000]: (typ=Held, actor=Item_Mushroom_E, )
      [001]: (typ=Held, actor=Item_Mushroom_E, )

----- Step[11]: unpause

game: (Running)
  screen: (Overworld)
  pouch: (count=2, are_tabs_valid=true, num_tabs=1, holding_in_inventory=false, )
    items: (len=2, )
      [000]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222200068, next=0x0000002222243a38, )
      [001]: (actor_name=Item_Fruit_I, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  overworld: (len=2, )
      [000]: (typ=GroundItem, actor=Item_Mushroom_E, despawning=false, )
      [001]: (typ=GroundItem, actor=Item_Mushroom_E, despawning=false, )

}
