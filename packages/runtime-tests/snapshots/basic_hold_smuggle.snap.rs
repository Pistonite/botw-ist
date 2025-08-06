// This has RS extension since that usually gives a minimal syntax highlighting.
//This is not an actual RS file

x!{ SKYBOOK RUNTIME SNAPSHOT V1

=====

----- Step[0]: !init apple banana core durian slate glider

game: (Running)
  screen: (Overworld)
  pouch: (count=6, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=6, )
      [000]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Fruit_H, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Item_Enemy_30, value=1, is_equipped=false, item_type=Material, item_use=Item, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Item_Fruit_D, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=3, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=4, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=6, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Fruit_H, value=1, is_equipped=false, )
      [002]: (idx=2, actor=Item_Enemy_30, value=1, is_equipped=false, )
      [003]: (idx=3, actor=Item_Fruit_D, value=1, is_equipped=false, )
      [004]: (idx=4, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [005]: (idx=5, actor=PlayerStole2, value=1, is_equipped=false, )

----- Step[1]: hold apple banana core

game: (Running)
  screen: (Inventory)
  pouch: (count=6, are_tabs_valid=true, num_tabs=2, holding_in_inventory=true, trial=false, )
    items: (len=6, )
      [000]: (actor_name=Item_Fruit_A, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        holding: 1
      [001]: (actor_name=Item_Fruit_H, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
        holding: 1
      [002]: (actor_name=Item_Enemy_30, value=0, is_equipped=false, item_type=Material, item_use=Item, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        holding: 1
      [003]: (actor_name=Item_Fruit_D, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=3, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=4, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=6, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Fruit_H, value=1, is_equipped=false, )
      [002]: (idx=2, actor=Item_Enemy_30, value=1, is_equipped=false, )
      [003]: (idx=3, actor=Item_Fruit_D, value=1, is_equipped=false, )
      [004]: (idx=4, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [005]: (idx=5, actor=PlayerStole2, value=1, is_equipped=false, )

----- Step[2]: overload

game: (Running)
  screen: (Inventory)
  pouch: (count=6, are_tabs_valid=true, num_tabs=2, holding_in_inventory=true, trial=false, )
    items: (len=6, )
      [000]: (actor_name=Item_Fruit_A, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        holding: 1
      [001]: (actor_name=Item_Fruit_H, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
        holding: 1
      [002]: (actor_name=Item_Enemy_30, value=0, is_equipped=false, item_type=Material, item_use=Item, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        holding: 1
      [003]: (actor_name=Item_Fruit_D, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=3, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=4, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=6, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Fruit_H, value=1, is_equipped=false, )
      [002]: (idx=2, actor=Item_Enemy_30, value=1, is_equipped=false, )
      [003]: (idx=3, actor=Item_Fruit_D, value=1, is_equipped=false, )
      [004]: (idx=4, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [005]: (idx=5, actor=PlayerStole2, value=1, is_equipped=false, )

----- Step[3]: unpause

game: (Running)
  screen: (Overworld)
  pouch: (count=6, are_tabs_valid=true, num_tabs=2, holding_in_inventory=true, trial=false, )
    items: (len=6, )
      [000]: (actor_name=Item_Fruit_A, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        holding: 1
      [001]: (actor_name=Item_Fruit_H, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
        holding: 1
      [002]: (actor_name=Item_Enemy_30, value=0, is_equipped=false, item_type=Material, item_use=Item, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        holding: 1
      [003]: (actor_name=Item_Fruit_D, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=3, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=4, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=6, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Fruit_H, value=1, is_equipped=false, )
      [002]: (idx=2, actor=Item_Enemy_30, value=1, is_equipped=false, )
      [003]: (idx=3, actor=Item_Fruit_D, value=1, is_equipped=false, )
      [004]: (idx=4, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [005]: (idx=5, actor=PlayerStole2, value=1, is_equipped=false, )

----- Step[4]: unoverload

game: (Running)
  screen: (Overworld)
  pouch: (count=6, are_tabs_valid=true, num_tabs=2, holding_in_inventory=true, trial=false, )
    items: (len=6, )
      [000]: (actor_name=Item_Fruit_A, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        holding: 1
      [001]: (actor_name=Item_Fruit_H, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
        holding: 1
      [002]: (actor_name=Item_Enemy_30, value=0, is_equipped=false, item_type=Material, item_use=Item, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        holding: 1
      [003]: (actor_name=Item_Fruit_D, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=3, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=4, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=6, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Fruit_H, value=1, is_equipped=false, )
      [002]: (idx=2, actor=Item_Enemy_30, value=1, is_equipped=false, )
      [003]: (idx=3, actor=Item_Fruit_D, value=1, is_equipped=false, )
      [004]: (idx=4, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [005]: (idx=5, actor=PlayerStole2, value=1, is_equipped=false, )

----- Step[5]: pause

game: (Running)
  screen: (Inventory)
  pouch: (count=6, are_tabs_valid=true, num_tabs=2, holding_in_inventory=true, trial=false, )
    items: (len=6, )
      [000]: (actor_name=Item_Fruit_A, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        holding: 1
      [001]: (actor_name=Item_Fruit_H, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
        holding: 1
      [002]: (actor_name=Item_Enemy_30, value=0, is_equipped=false, item_type=Material, item_use=Item, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        holding: 1
      [003]: (actor_name=Item_Fruit_D, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=3, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=4, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=6, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Fruit_H, value=1, is_equipped=false, )
      [002]: (idx=2, actor=Item_Enemy_30, value=1, is_equipped=false, )
      [003]: (idx=3, actor=Item_Fruit_D, value=1, is_equipped=false, )
      [004]: (idx=4, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [005]: (idx=5, actor=PlayerStole2, value=1, is_equipped=false, )

----- Step[6]: unpause

game: (Running)
  screen: (Overworld)
  pouch: (count=6, are_tabs_valid=true, num_tabs=2, holding_in_inventory=true, trial=false, )
    items: (len=6, )
      [000]: (actor_name=Item_Fruit_A, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        holding: 1
      [001]: (actor_name=Item_Fruit_H, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
        holding: 1
      [002]: (actor_name=Item_Enemy_30, value=0, is_equipped=false, item_type=Material, item_use=Item, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        holding: 1
      [003]: (actor_name=Item_Fruit_D, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=3, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=4, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=6, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Fruit_H, value=1, is_equipped=false, )
      [002]: (idx=2, actor=Item_Enemy_30, value=1, is_equipped=false, )
      [003]: (idx=3, actor=Item_Fruit_D, value=1, is_equipped=false, )
      [004]: (idx=4, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [005]: (idx=5, actor=PlayerStole2, value=1, is_equipped=false, )

----- Step[7]: hold durian

game: (Running)
  screen: (Inventory)
  pouch: (count=6, are_tabs_valid=true, num_tabs=2, holding_in_inventory=true, trial=false, )
    items: (len=6, )
      [000]: (actor_name=Item_Fruit_A, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        holding: 1
      [001]: (actor_name=Item_Fruit_H, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
        holding: 1
      [002]: (actor_name=Item_Enemy_30, value=0, is_equipped=false, item_type=Material, item_use=Item, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        holding: 1
      [003]: (actor_name=Item_Fruit_D, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=3, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
        holding: 1
      [004]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=4, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=6, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Fruit_H, value=1, is_equipped=false, )
      [002]: (idx=2, actor=Item_Enemy_30, value=1, is_equipped=false, )
      [003]: (idx=3, actor=Item_Fruit_D, value=1, is_equipped=false, )
      [004]: (idx=4, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [005]: (idx=5, actor=PlayerStole2, value=1, is_equipped=false, )

----- Step[8]: unpause

game: (Running)
  screen: (Overworld)
  pouch: (count=6, are_tabs_valid=true, num_tabs=2, holding_in_inventory=true, trial=false, )
    items: (len=6, )
      [000]: (actor_name=Item_Fruit_A, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        holding: 1
      [001]: (actor_name=Item_Fruit_H, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
        holding: 1
      [002]: (actor_name=Item_Enemy_30, value=0, is_equipped=false, item_type=Material, item_use=Item, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        holding: 1
      [003]: (actor_name=Item_Fruit_D, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=3, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
        holding: 1
      [004]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=4, tab_type=KeyItem, )
  overworld: (len=1, )
    [000]: (typ=Held, actor=Item_Fruit_D, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=6, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Fruit_H, value=1, is_equipped=false, )
      [002]: (idx=2, actor=Item_Enemy_30, value=1, is_equipped=false, )
      [003]: (idx=3, actor=Item_Fruit_D, value=1, is_equipped=false, )
      [004]: (idx=4, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [005]: (idx=5, actor=PlayerStole2, value=1, is_equipped=false, )

----- Step[9]: drop

game: (Running)
  screen: (Overworld)
  pouch: (count=2, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=2, )
      [000]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x0000002222200068, next=0x0000002222243270, )
      [001]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=-1, tab_type=Material, )
      [01]: (item_idx=0, tab_type=KeyItem, )
  overworld: (len=1, )
    [000]: (typ=GroundItem, actor=Item_Fruit_D, despawning=false, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=2, )
      [000]: (idx=0, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [001]: (idx=1, actor=PlayerStole2, value=1, is_equipped=false, )

----- Step[10]: !init 6 apple;

game: (Running)
  screen: (Overworld)
  pouch: (count=1, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=1, )
      [000]: (actor_name=Item_Fruit_A, value=6, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=-1, tab_type=KeyItem, )
  overworld: (len=1, )
    [000]: (typ=GroundItem, actor=Item_Fruit_D, despawning=false, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=1, )
      [000]: (idx=0, actor=Item_Fruit_A, value=6, is_equipped=false, )

----- Step[11]: overload

game: (Running)
  screen: (Overworld)
  pouch: (count=1, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=1, )
      [000]: (actor_name=Item_Fruit_A, value=6, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=-1, tab_type=KeyItem, )
  overworld: (len=1, )
    [000]: (typ=GroundItem, actor=Item_Fruit_D, despawning=false, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=1, )
      [000]: (idx=0, actor=Item_Fruit_A, value=6, is_equipped=false, )

----- Step[12]: hold 5 apple

game: (Running)
  screen: (Inventory)
  pouch: (count=1, are_tabs_valid=true, num_tabs=2, holding_in_inventory=true, trial=false, )
    items: (len=1, )
      [000]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222200068, )
        holding: 5
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=-1, tab_type=KeyItem, )
  overworld: (len=1, )
    [000]: (typ=GroundItem, actor=Item_Fruit_D, despawning=false, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=1, )
      [000]: (idx=0, actor=Item_Fruit_A, value=6, is_equipped=false, )

----- Step[13]: sell apple

game: (Running)
  screen: (Shop)
  pouch: (count=1, are_tabs_valid=true, num_tabs=2, holding_in_inventory=true, trial=false, )
    items: (len=1, )
      [000]: (actor_name=Item_Fruit_A, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222200068, )
        in_inventory: false
        holding: 5
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=-1, tab_type=KeyItem, )
  overworld: (len=1, )
    [000]: (typ=GroundItem, actor=Item_Fruit_D, despawning=false, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=1, )
      [000]: (idx=0, actor=Item_Fruit_A, value=6, is_equipped=false, )

----- Step[14]: get giant-ancient-core

game: (Running)
  screen: (Overworld)
  pouch: (count=1, are_tabs_valid=true, num_tabs=2, holding_in_inventory=true, trial=false, )
    items: (len=1, )
      [000]: (actor_name=Item_Enemy_31, value=1, is_equipped=false, item_type=Material, item_use=Item, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222200068, )
        holding: 5
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=-1, tab_type=KeyItem, )
  overworld: (len=1, )
    [000]: (typ=GroundItem, actor=Item_Fruit_D, despawning=false, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=1, )
      [000]: (idx=0, actor=Item_Enemy_31, value=6, is_equipped=false, )

----- Step[15]: unhold

game: (Running)
  screen: (Overworld)
  pouch: (count=1, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=1, )
      [000]: (actor_name=Item_Enemy_31, value=6, is_equipped=false, item_type=Material, item_use=Item, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=-1, tab_type=KeyItem, )
  overworld: (len=1, )
    [000]: (typ=GroundItem, actor=Item_Fruit_D, despawning=false, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=1, )
      [000]: (idx=0, actor=Item_Enemy_31, value=6, is_equipped=false, )

}
