// This has RS extension since that usually gives a minimal syntax highlighting.
//This is not an actual RS file

x!{ SKYBOOK RUNTIME SNAPSHOT V1

=====

----- Step[0]: !add-slot

game: (Running)
  screen: (Overworld)
  pouch: (count=0, are_tabs_valid=true, num_tabs=0, holding_in_inventory=false, )
    items: (len=0, )
    tabs: (len=0, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,__,__,__
    items: (len=0, )

----- Step[1]: !init

<same>
----- Step[2]: !init 1 slate 1 glider 5 apples

game: (Running)
  screen: (Overworld)
  pouch: (count=3, are_tabs_valid=true, num_tabs=3, holding_in_inventory=false, )
    items: (len=3, )
      [000]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Item_Fruit_A, value=5, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x0000002222200068, )
    tabs: (len=3, )
      [00]: (item_idx=-1, tab_type=Material, )
      [01]: (item_idx=0, tab_type=KeyItem, )
      [02]: (item_idx=2, tab_type=Material, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,Ki
    items: (len=3, )
      [000]: (idx=0, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [001]: (idx=1, actor=PlayerStole2, value=1, is_equipped=false, )
      [002]: (idx=2, actor=Item_Fruit_A, value=5, is_equipped=false, )

----- Step[3]: !add-slot master-sword[equip]

game: (Running)
  screen: (Overworld)
  pouch: (count=4, are_tabs_valid=true, num_tabs=7, holding_in_inventory=false, )
    items: (len=4, )
      [000]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=2, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Item_Fruit_A, value=5, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=3, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Weapon_Sword_070, value=4000, is_equipped=true, item_type=Sword, item_use=WeaponSmallSword, tab_idx=4, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222200068, )
        dpad_accessible: false
    tabs: (len=7, )
      [00]: (item_idx=-1, tab_type=Sword, )
      [01]: (item_idx=-1, tab_type=Material, )
      [02]: (item_idx=0, tab_type=KeyItem, )
      [03]: (item_idx=2, tab_type=Material, )
      [04]: (item_idx=3, tab_type=Sword, )
      [05]: (item_idx=-1, tab_type=Material, )
      [06]: (item_idx=-1, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,__,__,__,Ma,__,Ki
    items: (len=4, )
      [000]: (idx=0, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [001]: (idx=1, actor=PlayerStole2, value=1, is_equipped=false, )
      [002]: (idx=2, actor=Item_Fruit_A, value=5, is_equipped=false, )
      [003]: (idx=3, actor=Weapon_Sword_070, value=4000, is_equipped=true, )
        weapon: (idx=0, modifier=none, )

}
