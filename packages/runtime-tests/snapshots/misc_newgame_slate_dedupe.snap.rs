// This has RS extension since that usually gives a minimal syntax highlighting.
//This is not an actual RS file

x!{ SKYBOOK RUNTIME SNAPSHOT V1

=====

----- Step[0]: save-as sor

game: (Running)
  screen: (Overworld)
  pouch: (count=0, are_tabs_valid=true, num_tabs=0, holding_in_inventory=false, trial=false, )
    items: (len=0, )
    tabs: (len=0, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,__,__,__
    items: (len=0, )

----- Step[1]: !init 4 axe 4 simm 1 slate

game: (Running)
  screen: (Overworld)
  pouch: (count=9, are_tabs_valid=true, num_tabs=3, holding_in_inventory=false, trial=false, )
    items: (len=9, )
      [000]: (actor_name=Weapon_Lsword_032, value=4700, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Weapon_Lsword_032, value=4700, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Weapon_Lsword_032, value=4700, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Weapon_Lsword_032, value=4700, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=3, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Item_Cook_B_02, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=Item_Cook_B_02, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222242fd8, )
      [006]: (actor_name=Item_Cook_B_02, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=1, tab_slot=2, )
        node: (valid=true, pos=413, addr=0x0000002222242fd8, prev=0x0000002222243270, next=0x0000002222242d40, )
      [007]: (actor_name=Item_Cook_B_02, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=1, tab_slot=3, )
        node: (valid=true, pos=412, addr=0x0000002222242d40, prev=0x0000002222242fd8, next=0x0000002222242aa8, )
      [008]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=411, addr=0x0000002222242aa8, prev=0x0000002222242d40, next=0x0000002222200068, )
    tabs: (len=3, )
      [00]: (item_idx=0, tab_type=Sword, )
      [01]: (item_idx=4, tab_type=Food, )
      [02]: (item_idx=8, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,__,__,__,__,Fo,Ki
    items: (len=9, )
      [000]: (idx=0, actor=Weapon_Lsword_032, value=4700, is_equipped=false, )
        weapon: (idx=0, modifier=none, )
      [001]: (idx=1, actor=Weapon_Lsword_032, value=4700, is_equipped=false, )
        weapon: (idx=1, modifier=none, )
      [002]: (idx=2, actor=Weapon_Lsword_032, value=4700, is_equipped=false, )
        weapon: (idx=2, modifier=none, )
      [003]: (idx=3, actor=Weapon_Lsword_032, value=4700, is_equipped=false, )
        weapon: (idx=3, modifier=none, )
      [004]: (idx=4, actor=Item_Cook_B_02, value=1, is_equipped=false, )
        food: (idx=0, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [005]: (idx=5, actor=Item_Cook_B_02, value=1, is_equipped=false, )
        food: (idx=1, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [006]: (idx=6, actor=Item_Cook_B_02, value=1, is_equipped=false, )
        food: (idx=2, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [007]: (idx=7, actor=Item_Cook_B_02, value=1, is_equipped=false, )
        food: (idx=3, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [008]: (idx=8, actor=Obj_DRStone_Get, value=1, is_equipped=false, )

----- Step[2]: !break 1 slot

game: (Running)
  screen: (Overworld)
  pouch: (count=8, are_tabs_valid=true, num_tabs=3, holding_in_inventory=false, trial=false, )
    items: (len=9, )
      [000]: (actor_name=Weapon_Lsword_032, value=4700, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Weapon_Lsword_032, value=4700, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Weapon_Lsword_032, value=4700, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Weapon_Lsword_032, value=4700, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=3, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Item_Cook_B_02, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=Item_Cook_B_02, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222242fd8, )
      [006]: (actor_name=Item_Cook_B_02, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=1, tab_slot=2, )
        node: (valid=true, pos=413, addr=0x0000002222242fd8, prev=0x0000002222243270, next=0x0000002222242d40, )
      [007]: (actor_name=Item_Cook_B_02, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=1, tab_slot=3, )
        node: (valid=true, pos=412, addr=0x0000002222242d40, prev=0x0000002222242fd8, next=0x0000002222242aa8, )
      [008]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=411, addr=0x0000002222242aa8, prev=0x0000002222242d40, next=0x0000002222200068, )
    tabs: (len=3, )
      [00]: (item_idx=0, tab_type=Sword, )
      [01]: (item_idx=4, tab_type=Food, )
      [02]: (item_idx=8, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,__,__,__,__,Fo,Ki
    items: (len=9, )
      [000]: (idx=0, actor=Weapon_Lsword_032, value=4700, is_equipped=false, )
        weapon: (idx=0, modifier=none, )
      [001]: (idx=1, actor=Weapon_Lsword_032, value=4700, is_equipped=false, )
        weapon: (idx=1, modifier=none, )
      [002]: (idx=2, actor=Weapon_Lsword_032, value=4700, is_equipped=false, )
        weapon: (idx=2, modifier=none, )
      [003]: (idx=3, actor=Weapon_Lsword_032, value=4700, is_equipped=false, )
        weapon: (idx=3, modifier=none, )
      [004]: (idx=4, actor=Item_Cook_B_02, value=1, is_equipped=false, )
        food: (idx=0, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [005]: (idx=5, actor=Item_Cook_B_02, value=1, is_equipped=false, )
        food: (idx=1, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [006]: (idx=6, actor=Item_Cook_B_02, value=1, is_equipped=false, )
        food: (idx=2, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [007]: (idx=7, actor=Item_Cook_B_02, value=1, is_equipped=false, )
        food: (idx=3, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [008]: (idx=8, actor=Obj_DRStone_Get, value=1, is_equipped=false, )

----- Step[3]: reload sor

game: (Running)
  screen: (Overworld)
  pouch: (count=0, are_tabs_valid=true, num_tabs=1, holding_in_inventory=false, trial=false, )
    items: (len=1, )
      [000]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=411, addr=0x0000002222242aa8, prev=0x0000002222200068, next=0x0000002222200068, )
        accessible: false
        dpad_accessible: false
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=KeyItem, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,__,__,__
    items: (len=0, )

----- Step[4]: get 1 axe

game: (Running)
  screen: (Overworld)
  pouch: (count=1, are_tabs_valid=true, num_tabs=3, holding_in_inventory=false, trial=false, )
    items: (len=2, )
      [000]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=411, addr=0x0000002222242aa8, prev=0x0000002222200068, next=0x0000002222242d40, )
        accessible: false
        dpad_accessible: false
      [001]: (actor_name=Weapon_Lsword_032, value=4700, is_equipped=true, item_type=Sword, item_use=WeaponLargeSword, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=412, addr=0x0000002222242d40, prev=0x0000002222242aa8, next=0x0000002222200068, )
        dpad_accessible: false
    tabs: (len=3, )
      [00]: (item_idx=-1, tab_type=Sword, )
      [01]: (item_idx=0, tab_type=KeyItem, )
      [02]: (item_idx=1, tab_type=Sword, )
  overworld: (len=1, )
    [000]: (typ=Equipped, actor=Weapon_Lsword_032, value=4700, modifier=none, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,__,__,__,__,__,__
    items: (len=2, )
      [000]: (idx=0, actor=Obj_DRStone_Get, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Weapon_Lsword_032, value=4700, is_equipped=true, )
        weapon: (idx=0, modifier=none, )

----- Step[5]: drop 1 weapon

game: (Running)
  screen: (Inventory)
  pouch: (count=1, are_tabs_valid=true, num_tabs=3, holding_in_inventory=false, trial=false, )
    items: (len=2, )
      [000]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=411, addr=0x0000002222242aa8, prev=0x0000002222200068, next=0x0000002222242d40, )
        accessible: false
        dpad_accessible: false
      [001]: (actor_name=Weapon_Lsword_032, value=0, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=412, addr=0x0000002222242d40, prev=0x0000002222242aa8, next=0x0000002222200068, )
        in_inventory: false
        dpad_accessible: false
    tabs: (len=3, )
      [00]: (item_idx=-1, tab_type=Sword, )
      [01]: (item_idx=0, tab_type=KeyItem, )
      [02]: (item_idx=1, tab_type=Sword, )
  overworld: (len=2, )
    [000]: (typ=Equipped, actor=Weapon_Lsword_032, value=4700, modifier=none, )
    [001]: (typ=GroundEquipment, actor=Weapon_Lsword_032, value=4700, modifier=none, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,__,__,__,__,__,__
    items: (len=1, )
      [000]: (idx=0, actor=Obj_DRStone_Get, value=1, is_equipped=false, )

----- Step[6]: get 1 slate

game: (Running)
  screen: (Overworld)
  pouch: (count=0, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, trial=false, )
    items: (len=1, )
      [000]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=411, addr=0x0000002222242aa8, prev=0x0000002222200068, next=0x0000002222200068, )
        accessible: false
        dpad_accessible: false
    tabs: (len=2, )
      [00]: (item_idx=-1, tab_type=Sword, )
      [01]: (item_idx=0, tab_type=KeyItem, )
  overworld: (len=1, )
    [000]: (typ=GroundEquipment, actor=Weapon_Lsword_032, value=4700, modifier=none, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,__,__,__,__,__,__
    items: (len=0, )

}
