// This has RS extension since that usually gives a minimal syntax highlighting.
//This is not an actual RS file

x!{ SKYBOOK RUNTIME SNAPSHOT V1

=====

----- Step[0]: !init 1 apple 2 hylianshroom 1 slate 1 glider

game: (Running)
  screen: (Overworld)
  pouch: (count=4, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, )
    items: (len=4, )
      [000]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=2, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=4, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Mushroom_E, value=2, is_equipped=false, )
      [002]: (idx=2, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [003]: (idx=3, actor=PlayerStole2, value=1, is_equipped=false, )

----- Step[1]: save

game: (Running)
  screen: (Inventory)
  pouch: (count=4, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, )
    items: (len=4, )
      [000]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=2, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=4, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Mushroom_E, value=2, is_equipped=false, )
      [002]: (idx=2, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [003]: (idx=3, actor=PlayerStole2, value=1, is_equipped=false, )

----- Step[2]: !break 7 slots

game: (Running)
  screen: (Inventory)
  pouch: (count=-3, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, )
    items: (len=4, )
      [000]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=2, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=4, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Mushroom_E, value=2, is_equipped=false, )
      [002]: (idx=2, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [003]: (idx=3, actor=PlayerStole2, value=1, is_equipped=false, )

----- Step[3]: reload

game: (Running)
  screen: (Overworld)
  pouch: (count=-1, are_tabs_valid=true, num_tabs=3, holding_in_inventory=false, )
    items: (len=6, )
      [000]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Item_Mushroom_E, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=2, tab_slot=1, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
    tabs: (len=3, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=2, tab_type=KeyItem, )
      [02]: (item_idx=4, tab_type=Material, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=4, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Mushroom_E, value=2, is_equipped=false, )
      [002]: (idx=2, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [003]: (idx=3, actor=PlayerStole2, value=1, is_equipped=false, )

----- Step[4]: save

game: (Running)
  screen: (Inventory)
  pouch: (count=-1, are_tabs_valid=true, num_tabs=3, holding_in_inventory=false, )
    items: (len=6, )
      [000]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Item_Mushroom_E, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=2, tab_slot=1, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
    tabs: (len=3, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=2, tab_type=KeyItem, )
      [02]: (item_idx=4, tab_type=Material, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=4, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Mushroom_E, value=2, is_equipped=false, )
      [002]: (idx=2, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [003]: (idx=3, actor=PlayerStole2, value=1, is_equipped=false, )

----- Step[5]: close-game

game: (Closed)

----- Step[6]: reload

game: (Running)
  screen: (Overworld)
  pouch: (count=4, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, )
    items: (len=4, )
      [000]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=2, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=4, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Mushroom_E, value=2, is_equipped=false, )
      [002]: (idx=2, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [003]: (idx=3, actor=PlayerStole2, value=1, is_equipped=false, )

}
