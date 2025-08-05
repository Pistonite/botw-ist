// This has RS extension since that usually gives a minimal syntax highlighting.
//This is not an actual RS file

x!{ SKYBOOK RUNTIME SNAPSHOT V1

=====

----- Step[0]: get apple, banana, fairy, palm-fruit

game: (Running)
  screen: (Overworld)
  pouch: (count=4, are_tabs_valid=true, num_tabs=1, holding_in_inventory=false, trial=false, )
    items: (len=4, )
      [000]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Fruit_H, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Animal_Insect_F, value=1, is_equipped=false, item_type=Material, item_use=Item, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Item_Fruit_G, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=3, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,__
    items: (len=4, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Fruit_H, value=1, is_equipped=false, )
      [002]: (idx=2, actor=Animal_Insect_F, value=1, is_equipped=false, )
      [003]: (idx=3, actor=Item_Fruit_G, value=1, is_equipped=false, )

----- Step[1]: eat all but banana

game: (Running)
  screen: (Inventory)
  pouch: (count=4, are_tabs_valid=true, num_tabs=1, holding_in_inventory=false, trial=false, )
    items: (len=4, )
      [000]: (actor_name=Item_Fruit_A, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        in_inventory: false
      [001]: (actor_name=Item_Fruit_H, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Animal_Insect_F, value=0, is_equipped=false, item_type=Material, item_use=Item, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        in_inventory: false
      [003]: (actor_name=Item_Fruit_G, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=3, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222200068, )
        in_inventory: false
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,__
    items: (len=1, )
      [000]: (idx=0, actor=Item_Fruit_H, value=1, is_equipped=false, )

}
