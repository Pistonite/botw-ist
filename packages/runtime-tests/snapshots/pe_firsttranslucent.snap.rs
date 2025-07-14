// This has RS extension since that usually gives a minimal syntax highlighting.
//This is not an actual RS file

x!{ SKYBOOK RUNTIME SNAPSHOT V1

=====

----- Step[0]: !init 1 lotus 1 carrot 1 roasted-radish 1 slate 4 orb 1 banana

game: (Running)
  screen: (Overworld)
  pouch: (count=6, are_tabs_valid=true, num_tabs=4, holding_in_inventory=false, )
    items: (len=6, )
      [000]: (actor_name=Item_Fruit_E, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_PlantGet_M, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Item_Roast_18, value=1, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Obj_DungeonClearSeal, value=4, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=2, tab_slot=1, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=Item_Fruit_H, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=3, tab_slot=0, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
    tabs: (len=4, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=2, tab_type=Food, )
      [02]: (item_idx=3, tab_type=KeyItem, )
      [03]: (item_idx=5, tab_type=Material, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,Fo,Ki
    items: (len=6, )
      [000]: (idx=0, actor=Item_Fruit_E, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_PlantGet_M, value=1, is_equipped=false, )
      [002]: (idx=2, actor=Item_Roast_18, value=1, is_equipped=false, )
        food: (idx=0, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [003]: (idx=3, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [004]: (idx=4, actor=Obj_DungeonClearSeal, value=4, is_equipped=false, )
      [005]: (idx=5, actor=Item_Fruit_H, value=1, is_equipped=false, )

----- Step[1]: entangle banana

game: (Running)
  screen: (Inventory)
  pouch: (count=6, are_tabs_valid=true, num_tabs=4, holding_in_inventory=false, )
    items: (len=6, )
      [000]: (actor_name=Item_Fruit_E, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        entangled: true
      [001]: (actor_name=Item_PlantGet_M, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Item_Roast_18, value=1, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Obj_DungeonClearSeal, value=4, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=2, tab_slot=1, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=Item_Fruit_H, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=3, tab_slot=0, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
        entangled: true
    tabs: (len=4, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=2, tab_type=Food, )
      [02]: (item_idx=3, tab_type=KeyItem, )
      [03]: (item_idx=5, tab_type=Material, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,Fo,Ki
    items: (len=6, )
      [000]: (idx=0, actor=Item_Fruit_E, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_PlantGet_M, value=1, is_equipped=false, )
      [002]: (idx=2, actor=Item_Roast_18, value=1, is_equipped=false, )
        food: (idx=0, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [003]: (idx=3, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [004]: (idx=4, actor=Obj_DungeonClearSeal, value=4, is_equipped=false, )
      [005]: (idx=5, actor=Item_Fruit_H, value=1, is_equipped=false, )

----- Step[2]: :targeting lotus

<same>
----- Step[3]: eat banana

game: (Running)
  screen: (Inventory)
  pouch: (count=6, are_tabs_valid=true, num_tabs=4, holding_in_inventory=false, )
    items: (len=6, )
      [000]: (actor_name=Item_Fruit_E, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        in_inventory: false
        entangled: true
      [001]: (actor_name=Item_PlantGet_M, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Item_Roast_18, value=1, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Obj_DungeonClearSeal, value=4, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=2, tab_slot=1, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=Item_Fruit_H, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=3, tab_slot=0, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
        entangled: true
    tabs: (len=4, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=2, tab_type=Food, )
      [02]: (item_idx=3, tab_type=KeyItem, )
      [03]: (item_idx=5, tab_type=Material, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,Fo,Ki
    items: (len=5, )
      [000]: (idx=0, actor=Item_PlantGet_M, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Roast_18, value=1, is_equipped=false, )
        food: (idx=0, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [002]: (idx=2, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [003]: (idx=3, actor=Obj_DungeonClearSeal, value=4, is_equipped=false, )
      [004]: (idx=4, actor=Item_Fruit_H, value=1, is_equipped=false, )

----- Step[4]: entangle banana

<same>
----- Step[5]: :targeting lotus[category=material, tab=1,row=1,col=1]

<same>
----- Step[6]: hold banana

game: (Running)
  screen: (Inventory)
  pouch: (count=6, are_tabs_valid=true, num_tabs=4, holding_in_inventory=true, )
    items: (len=6, )
      [000]: (actor_name=Item_Fruit_E, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        in_inventory: false
        holding: 1
        entangled: true
      [001]: (actor_name=Item_PlantGet_M, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Item_Roast_18, value=1, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Obj_DungeonClearSeal, value=4, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=2, tab_slot=1, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=Item_Fruit_H, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=3, tab_slot=0, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
        entangled: true
    tabs: (len=4, )
      [00]: (item_idx=0, tab_type=Material, )
      [01]: (item_idx=2, tab_type=Food, )
      [02]: (item_idx=3, tab_type=KeyItem, )
      [03]: (item_idx=5, tab_type=Material, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,Fo,Ki
    items: (len=5, )
      [000]: (idx=0, actor=Item_PlantGet_M, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Roast_18, value=1, is_equipped=false, )
        food: (idx=0, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [002]: (idx=2, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [003]: (idx=3, actor=Obj_DungeonClearSeal, value=4, is_equipped=false, )
      [004]: (idx=4, actor=Item_Fruit_H, value=1, is_equipped=false, )

}
