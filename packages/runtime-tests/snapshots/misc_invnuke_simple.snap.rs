// This has RS extension since that usually gives a minimal syntax highlighting.
//This is not an actual RS file

x!{ SKYBOOK RUNTIME SNAPSHOT V1

=====

----- Step[0]: !init 1 apple 2 hylianshroom 3 slate

game: (Running)
  screen: (Overworld)
  pouch: (count=5, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=5, )
      [000]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=2, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=2, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=5, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Mushroom_E, value=2, is_equipped=false, )
      [002]: (idx=2, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [003]: (idx=3, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [004]: (idx=4, actor=Obj_DRStone_Get, value=1, is_equipped=false, )

----- Step[1]: !break 5 slots

game: (Running)
  screen: (Overworld)
  pouch: (count=0, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=5, )
      [000]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        accessible: false
        dpad_accessible: false
      [001]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
        accessible: false
        dpad_accessible: false
      [002]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        accessible: false
        dpad_accessible: false
      [003]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
        accessible: false
        dpad_accessible: false
      [004]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=2, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222200068, )
        accessible: false
        dpad_accessible: false
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=2, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=5, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Mushroom_E, value=2, is_equipped=false, )
      [002]: (idx=2, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [003]: (idx=3, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [004]: (idx=4, actor=Obj_DRStone_Get, value=1, is_equipped=false, )

----- Step[2]: get slate

game: (Running)
  screen: (Overworld)
  pouch: (count=0, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=5, )
      [000]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        accessible: false
        dpad_accessible: false
      [001]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
        accessible: false
        dpad_accessible: false
      [002]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        accessible: false
        dpad_accessible: false
      [003]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
        accessible: false
        dpad_accessible: false
      [004]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=2, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222200068, )
        accessible: false
        dpad_accessible: false
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=2, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=0, )

----- Step[3]: pause;

game: (Running)
  screen: (Inventory)
  pouch: (count=0, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=5, )
      [000]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        accessible: false
        dpad_accessible: false
      [001]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
        accessible: false
        dpad_accessible: false
      [002]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        accessible: false
        dpad_accessible: false
      [003]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
        accessible: false
        dpad_accessible: false
      [004]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=2, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222200068, )
        accessible: false
        dpad_accessible: false
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=2, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=0, )

----- Step[4]: unpause

game: (Running)
  screen: (Overworld)
  pouch: (count=0, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=5, )
      [000]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        accessible: false
        dpad_accessible: false
      [001]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
        accessible: false
        dpad_accessible: false
      [002]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        accessible: false
        dpad_accessible: false
      [003]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
        accessible: false
        dpad_accessible: false
      [004]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=2, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222200068, )
        accessible: false
        dpad_accessible: false
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=2, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=0, )

----- Step[5]: save

game: (Running)
  screen: (Inventory)
  pouch: (count=0, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=5, )
      [000]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        accessible: false
        dpad_accessible: false
      [001]: (actor_name=Item_Mushroom_E, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
        accessible: false
        dpad_accessible: false
      [002]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        accessible: false
        dpad_accessible: false
      [003]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
        accessible: false
        dpad_accessible: false
      [004]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=2, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222200068, )
        accessible: false
        dpad_accessible: false
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=2, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=0, )

}
