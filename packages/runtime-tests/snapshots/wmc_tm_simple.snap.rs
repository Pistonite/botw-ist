// This has RS extension since that usually gives a minimal syntax highlighting.
//This is not an actual RS file

x!{ SKYBOOK RUNTIME SNAPSHOT V1

=====

----- Step[0]: !break 3 slots

game: (Running)
  screen: (Overworld)
  pouch: (count=-3, are_tabs_valid=true, num_tabs=0, holding_in_inventory=false, trial=false, )
    items: (len=0, )
    tabs: (len=0, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,__,__,__
    items: (len=0, )

----- Step[1]: get travel-medallion

game: (Running)
  screen: (Overworld)
  pouch: (count=-2, are_tabs_valid=true, num_tabs=1, holding_in_inventory=false, trial=false, )
    items: (len=1, )
      [000]: (actor_name=Obj_WarpDLC, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,__,__,Ki
    items: (len=0, )

----- Step[2]: get 999 roasted-endura

game: (Running)
  screen: (Overworld)
  pouch: (count=-1, are_tabs_valid=true, num_tabs=3, holding_in_inventory=false, trial=false, )
    items: (len=2, )
      [000]: (actor_name=Obj_WarpDLC, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Roast_50, value=999, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222200068, )
    tabs: (len=3, )
      [00]: (item_idx=-1, tab_type=Food, )
      [01]: (item_idx=0, tab_type=KeyItem, )
      [02]: (item_idx=1, tab_type=Food, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,__,Fo,Ki
    items: (len=0, )

----- Step[3]: get wild-green[hp=120,price=113]

game: (Running)
  screen: (Overworld)
  pouch: (count=0, are_tabs_valid=true, num_tabs=3, holding_in_inventory=false, trial=false, )
    items: (len=3, )
      [000]: (actor_name=Obj_WarpDLC, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        accessible: false
        dpad_accessible: false
      [001]: (actor_name=Item_Roast_50, value=999, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
        accessible: false
        dpad_accessible: false
      [002]: (actor_name=Item_Cook_B_01, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=2, tab_slot=1, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x0000002222200068, )
        data: (value=120, duration=0, price=113, id=-1, level=0, )
        accessible: false
        dpad_accessible: false
    tabs: (len=3, )
      [00]: (item_idx=-1, tab_type=Food, )
      [01]: (item_idx=0, tab_type=KeyItem, )
      [02]: (item_idx=1, tab_type=Food, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,__,Fo,Ki
    items: (len=0, )

----- Step[4]: get 999 seared-steak

game: (Running)
  screen: (Overworld)
  pouch: (count=1, are_tabs_valid=true, num_tabs=3, holding_in_inventory=false, trial=false, )
    items: (len=4, )
      [000]: (actor_name=Obj_WarpDLC, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Roast_50, value=999, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Item_Cook_B_01, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=2, tab_slot=1, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        data: (value=120, duration=0, price=113, id=-1, level=0, )
      [003]: (actor_name=Item_Roast_01, value=999, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=2, tab_slot=2, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222200068, )
    tabs: (len=3, )
      [00]: (item_idx=-1, tab_type=Food, )
      [01]: (item_idx=0, tab_type=KeyItem, )
      [02]: (item_idx=1, tab_type=Food, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,__,Fo,Ki
    items: (len=4, )
      [000]: (idx=0, actor=Obj_WarpDLC, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Roast_50, value=999, is_equipped=false, )
        food: (idx=0, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [002]: (idx=2, actor=Item_Cook_B_01, value=1, is_equipped=false, )
        food: (idx=1, life_recover=120, duration=0, price=113, effect_id=-1, effect_level=0, )
      [003]: (idx=3, actor=Item_Roast_01, value=999, is_equipped=false, )
        food: (idx=2, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )

----- Step[5]: save

game: (Running)
  screen: (Inventory)
  pouch: (count=1, are_tabs_valid=true, num_tabs=3, holding_in_inventory=false, trial=false, )
    items: (len=4, )
      [000]: (actor_name=Obj_WarpDLC, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Roast_50, value=999, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Item_Cook_B_01, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=2, tab_slot=1, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        data: (value=120, duration=0, price=113, id=-1, level=0, )
      [003]: (actor_name=Item_Roast_01, value=999, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=2, tab_slot=2, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222200068, )
    tabs: (len=3, )
      [00]: (item_idx=-1, tab_type=Food, )
      [01]: (item_idx=0, tab_type=KeyItem, )
      [02]: (item_idx=1, tab_type=Food, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,__,Fo,Ki
    items: (len=4, )
      [000]: (idx=0, actor=Obj_WarpDLC, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Roast_50, value=999, is_equipped=false, )
        food: (idx=0, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [002]: (idx=2, actor=Item_Cook_B_01, value=1, is_equipped=false, )
        food: (idx=1, life_recover=120, duration=0, price=113, effect_id=-1, effect_level=0, )
      [003]: (idx=3, actor=Item_Roast_01, value=999, is_equipped=false, )
        food: (idx=2, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )

----- Step[6]: !system [dlc=none]

<same>
----- Step[7]: close-game

game: (Closed)

----- Step[8]: reload

game: (Running)
  screen: (Overworld)
  pouch: (count=3, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=3, )
      [000]: (actor_name=Item_Roast_50, value=999, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Cook_B_01, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
        data: (value=120, duration=0, price=113, id=-1, level=0, )
      [002]: (actor_name=Item_Roast_01, value=999, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Food, )
      [01]: (item_idx=-1, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,__,Fo,Ki
    items: (len=4, )
      [000]: (idx=0, actor=Obj_WarpDLC, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Roast_50, value=999, is_equipped=false, )
        food: (idx=0, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [002]: (idx=2, actor=Item_Cook_B_01, value=1, is_equipped=false, )
        food: (idx=1, life_recover=120, duration=0, price=113, effect_id=-1, effect_level=0, )
      [003]: (idx=3, actor=Item_Roast_01, value=999, is_equipped=false, )
        food: (idx=2, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )

----- Step[9]: get slate

game: (Running)
  screen: (Overworld)
  pouch: (count=4, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=4, )
      [000]: (actor_name=Item_Roast_50, value=999, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Cook_B_01, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
        data: (value=120, duration=0, price=113, id=-1, level=0, )
      [002]: (actor_name=Item_Roast_01, value=999, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Food, )
      [01]: (item_idx=3, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,__,Fo,Ki
    items: (len=4, )
      [000]: (idx=0, actor=Item_Roast_50, value=999, is_equipped=false, )
        food: (idx=0, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [001]: (idx=1, actor=Item_Cook_B_01, value=1, is_equipped=false, )
        food: (idx=1, life_recover=120, duration=0, price=113, effect_id=-1, effect_level=0, )
      [002]: (idx=2, actor=Item_Roast_01, value=999, is_equipped=false, )
        food: (idx=2, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [003]: (idx=3, actor=Obj_DRStone_Get, value=1, is_equipped=false, )

----- Step[10]: !break 3 slots

game: (Running)
  screen: (Overworld)
  pouch: (count=1, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=4, )
      [000]: (actor_name=Item_Roast_50, value=999, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Cook_B_01, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
        data: (value=120, duration=0, price=113, id=-1, level=0, )
      [002]: (actor_name=Item_Roast_01, value=999, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Food, )
      [01]: (item_idx=3, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,__,Fo,Ki
    items: (len=4, )
      [000]: (idx=0, actor=Item_Roast_50, value=999, is_equipped=false, )
        food: (idx=0, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [001]: (idx=1, actor=Item_Cook_B_01, value=1, is_equipped=false, )
        food: (idx=1, life_recover=120, duration=0, price=113, effect_id=-1, effect_level=0, )
      [002]: (idx=2, actor=Item_Roast_01, value=999, is_equipped=false, )
        food: (idx=2, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [003]: (idx=3, actor=Obj_DRStone_Get, value=1, is_equipped=false, )

----- Step[11]: eat wild-green

game: (Running)
  screen: (Inventory)
  pouch: (count=1, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=4, )
      [000]: (actor_name=Item_Roast_50, value=999, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Cook_B_01, value=0, is_equipped=false, item_type=Food, item_use=Item, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
        data: (value=120, duration=0, price=113, id=-1, level=0, )
        in_inventory: false
      [002]: (actor_name=Item_Roast_01, value=999, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Food, )
      [01]: (item_idx=3, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,__,Fo,Ki
    items: (len=3, )
      [000]: (idx=0, actor=Item_Roast_50, value=999, is_equipped=false, )
        food: (idx=0, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [001]: (idx=1, actor=Item_Roast_01, value=999, is_equipped=false, )
        food: (idx=1, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [002]: (idx=2, actor=Obj_DRStone_Get, value=1, is_equipped=false, )

----- Step[12]: sell all seared-steak

game: (Running)
  screen: (Shop)
  pouch: (count=0, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=3, )
      [000]: (actor_name=Item_Roast_50, value=999, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243a38, )
        accessible: false
        dpad_accessible: false
      [001]: (actor_name=Item_Roast_01, value=0, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243f68, next=0x00000022222437a0, )
        in_inventory: false
        accessible: false
        dpad_accessible: false
      [002]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222200068, )
        accessible: false
        dpad_accessible: false
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Food, )
      [01]: (item_idx=2, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,__,Fo,Ki
    items: (len=0, )

----- Step[13]: reload

game: (Running)
  screen: (Overworld)
  pouch: (count=2, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=5, )
      [000]: (actor_name=Item_Roast_50, value=999, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243a38, )
      [001]: (actor_name=Item_Roast_01, value=0, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243f68, next=0x0000002222243cd0, )
        in_inventory: false
      [002]: (actor_name=Item_Cook_B_01, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [003]: (actor_name=Item_Roast_01, value=999, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=0, tab_slot=3, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        data: (value=120, duration=0, price=113, id=-1, level=0, )
      [004]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243508, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Food, )
      [01]: (item_idx=4, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,__,Fo,Ki
    items: (len=4, )
      [000]: (idx=0, actor=Obj_WarpDLC, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Roast_50, value=999, is_equipped=false, )
        food: (idx=0, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [002]: (idx=2, actor=Item_Cook_B_01, value=1, is_equipped=false, )
        food: (idx=1, life_recover=120, duration=0, price=113, effect_id=-1, effect_level=0, )
      [003]: (idx=3, actor=Item_Roast_01, value=999, is_equipped=false, )
        food: (idx=2, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )

}
