// This has RS extension since that usually gives a minimal syntax highlighting.
//This is not an actual RS file

x!{ SKYBOOK RUNTIME SNAPSHOT V1

=====

----- Step[0]: get 2 apple 3 pepper

game: (Running)
  screen: (Overworld)
  pouch: (count=2, are_tabs_valid=true, num_tabs=1, )
    items: (len=2, )
      [000]: (actor_name=Item_Fruit_A, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Fruit_I, value=3, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )

----- Step[1]: hold pepper

game: (Running)
  screen: (Inventory)
  pouch: (count=2, are_tabs_valid=true, num_tabs=1, )
    items: (len=2, )
      [000]: (actor_name=Item_Fruit_A, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Fruit_I, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222200068, )
        holding: 1
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )

----- Step[2]: hold 2 apple

game: (Running)
  screen: (Inventory)
  pouch: (count=2, are_tabs_valid=true, num_tabs=1, )
    items: (len=2, )
      [000]: (actor_name=Item_Fruit_A, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        holding: 2
      [001]: (actor_name=Item_Fruit_I, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222200068, )
        holding: 1
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )

}
