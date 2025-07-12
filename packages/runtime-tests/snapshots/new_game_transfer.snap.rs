// This has RS extension since that usually gives a minimal syntax highlighting.
//This is not an actual RS file

x!{ SKYBOOK RUNTIME SNAPSHOT V1

=====

----- Step[0]: get apple

game: (Running)
  screen: (Overworld)
  pouch: (count=1, are_tabs_valid=true, num_tabs=1, holding_in_inventory=false, )
    items: (len=1, )
      [000]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,__
    items: (len=1, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )

----- Step[1]: close-game

game: (Closed)

----- Step[2]: new-game

game: (Running)
  screen: (Overworld)
  pouch: (count=0, are_tabs_valid=true, num_tabs=0, holding_in_inventory=false, )
    items: (len=0, )
    tabs: (len=0, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,__,__,__
    items: (len=0, )

----- Step[3]: get 1 banana 1 shroom

game: (Running)
  screen: (Overworld)
  pouch: (count=2, are_tabs_valid=true, num_tabs=1, holding_in_inventory=false, )
    items: (len=2, )
      [000]: (actor_name=Item_Fruit_H, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Mushroom_E, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,__
    items: (len=2, )
      [000]: (idx=0, actor=Item_Fruit_H, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Mushroom_E, value=1, is_equipped=false, )

----- Step[4]: !break 1 slot

game: (Running)
  screen: (Overworld)
  pouch: (count=1, are_tabs_valid=true, num_tabs=1, holding_in_inventory=false, )
    items: (len=2, )
      [000]: (actor_name=Item_Fruit_H, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Mushroom_E, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,__
    items: (len=2, )
      [000]: (idx=0, actor=Item_Fruit_H, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Mushroom_E, value=1, is_equipped=false, )

----- Step[5]: new-game

game: (Running)
  screen: (Overworld)
  pouch: (count=0, are_tabs_valid=true, num_tabs=1, holding_in_inventory=false, )
    items: (len=1, )
      [000]: (actor_name=Item_Mushroom_E, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222200068, next=0x0000002222200068, )
        accessible: false
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,__,__,__
    items: (len=0, )

}
