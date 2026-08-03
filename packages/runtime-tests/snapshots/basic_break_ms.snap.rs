// This has RS extension since that usually gives a minimal syntax highlighting.
//This is not an actual RS file

x!{ SKYBOOK RUNTIME SNAPSHOT V1

=====

----- Step[0]: get master-sword slate glider

game: (Running)
  screen: (Overworld)
  pouch: (count=3, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=3, )
      [000]: (actor_name=Weapon_Sword_070, value=4000, is_equipped=true, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Sword, )
      [01]: (item_idx=1, tab_type=KeyItem, )
  overworld: (len=1, )
    [000]: (typ=Equipped, actor=Weapon_Sword_070, value=4000, modifier=none, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,__,__,__,__,__,Ki
    items: (len=3, )
      [000]: (idx=0, actor=Weapon_Sword_070, value=4000, is_equipped=true, )
        weapon: (idx=0, modifier=none, )
      [001]: (idx=1, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [002]: (idx=2, actor=PlayerStole2, value=1, is_equipped=false, )

----- Step[1]: use weapon 40 times

game: (Running)
  screen: (Overworld)
  pouch: (count=3, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=3, )
      [000]: (actor_name=Weapon_Sword_070, value=0, is_equipped=false, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        dpad_accessible: false
      [001]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Sword, )
      [01]: (item_idx=1, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,__,__,__,__,__,Ki
    items: (len=3, )
      [000]: (idx=0, actor=Weapon_Sword_070, value=0, is_equipped=false, )
        weapon: (idx=0, modifier=none, )
      [001]: (idx=1, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [002]: (idx=2, actor=PlayerStole2, value=1, is_equipped=false, )

----- Step[2]: save

game: (Running)
  screen: (Inventory)
  pouch: (count=3, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=3, )
      [000]: (actor_name=Weapon_Sword_070, value=0, is_equipped=false, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        dpad_accessible: false
      [001]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Sword, )
      [01]: (item_idx=1, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,__,__,__,__,__,Ki
    items: (len=3, )
      [000]: (idx=0, actor=Weapon_Sword_070, value=0, is_equipped=false, )
        weapon: (idx=0, modifier=none, )
      [001]: (idx=1, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [002]: (idx=2, actor=PlayerStole2, value=1, is_equipped=false, )

----- Step[3]: reload

game: (Running)
  screen: (Overworld)
  pouch: (count=3, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=3, )
      [000]: (actor_name=Weapon_Sword_070, value=0, is_equipped=false, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222200068, next=0x0000002222243cd0, )
        dpad_accessible: false
      [001]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243a38, next=0x0000002222243f68, )
      [002]: (actor_name=PlayerStole2, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222243cd0, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Sword, )
      [01]: (item_idx=1, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,__,__,__,__,__,Ki
    items: (len=3, )
      [000]: (idx=0, actor=Weapon_Sword_070, value=0, is_equipped=false, )
        weapon: (idx=0, modifier=none, )
      [001]: (idx=1, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [002]: (idx=2, actor=PlayerStole2, value=1, is_equipped=false, )

}
