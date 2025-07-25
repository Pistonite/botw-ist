// This has RS extension since that usually gives a minimal syntax highlighting.
//This is not an actual RS file

x!{ SKYBOOK RUNTIME SNAPSHOT V1

=====

----- Step[0]: get 1 torch 1 travelers 1 pot 1 hylian-hood 1 apple 1 baked-apple 1 sheikah-slate

game: (Running)
  screen: (Overworld)
  pouch: (count=7, are_tabs_valid=true, num_tabs=7, holding_in_inventory=false, )
    items: (len=7, )
      [000]: (actor_name=Weapon_Sword_043, value=800, is_equipped=true, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Weapon_Bow_001, value=2200, is_equipped=true, item_type=Bow, item_use=WeaponBow, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Weapon_Shield_040, value=1000, is_equipped=true, item_type=Shield, item_use=WeaponShield, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Armor_001_Head, value=0, is_equipped=false, item_type=ArmorHead, item_use=ArmorHead, tab_idx=3, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=4, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=Item_Roast_03, value=1, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=5, tab_slot=0, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222242fd8, )
      [006]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=6, tab_slot=0, )
        node: (valid=true, pos=413, addr=0x0000002222242fd8, prev=0x0000002222243270, next=0x0000002222200068, )
    tabs: (len=7, )
      [00]: (item_idx=0, tab_type=Sword, )
      [01]: (item_idx=1, tab_type=Bow, )
      [02]: (item_idx=2, tab_type=Shield, )
      [03]: (item_idx=3, tab_type=ArmorHead, )
      [04]: (item_idx=4, tab_type=Material, )
      [05]: (item_idx=5, tab_type=Food, )
      [06]: (item_idx=6, tab_type=KeyItem, )
  overworld: (len=3, )
    [000]: (typ=Equipped, actor=Weapon_Sword_043, value=800, modifier=none, )
    [001]: (typ=Equipped, actor=Weapon_Bow_001, value=2200, modifier=none, )
    [002]: (typ=Equipped, actor=Weapon_Shield_040, value=1000, modifier=none, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,Bo,Sh,Ar,Ma,Fo,Ki
    items: (len=7, )
      [000]: (idx=0, actor=Weapon_Sword_043, value=800, is_equipped=true, )
        weapon: (idx=0, modifier=none, )
      [001]: (idx=1, actor=Weapon_Bow_001, value=2200, is_equipped=true, )
        weapon: (idx=0, modifier=none, )
      [002]: (idx=2, actor=Weapon_Shield_040, value=1000, is_equipped=true, )
        weapon: (idx=0, modifier=none, )
      [003]: (idx=3, actor=Armor_001_Head, value=0, is_equipped=false, )
      [004]: (idx=4, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [005]: (idx=5, actor=Item_Roast_03, value=1, is_equipped=false, )
        food: (idx=0, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [006]: (idx=6, actor=Obj_DRStone_Get, value=1, is_equipped=false, )

----- Step[1]: !break 25 slots

game: (Running)
  screen: (Overworld)
  pouch: (count=-18, are_tabs_valid=true, num_tabs=7, holding_in_inventory=false, )
    items: (len=7, )
      [000]: (actor_name=Weapon_Sword_043, value=800, is_equipped=true, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Weapon_Bow_001, value=2200, is_equipped=true, item_type=Bow, item_use=WeaponBow, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Weapon_Shield_040, value=1000, is_equipped=true, item_type=Shield, item_use=WeaponShield, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Armor_001_Head, value=0, is_equipped=false, item_type=ArmorHead, item_use=ArmorHead, tab_idx=3, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=4, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=Item_Roast_03, value=1, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=5, tab_slot=0, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222242fd8, )
      [006]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=6, tab_slot=0, )
        node: (valid=true, pos=413, addr=0x0000002222242fd8, prev=0x0000002222243270, next=0x0000002222200068, )
    tabs: (len=7, )
      [00]: (item_idx=0, tab_type=Sword, )
      [01]: (item_idx=1, tab_type=Bow, )
      [02]: (item_idx=2, tab_type=Shield, )
      [03]: (item_idx=3, tab_type=ArmorHead, )
      [04]: (item_idx=4, tab_type=Material, )
      [05]: (item_idx=5, tab_type=Food, )
      [06]: (item_idx=6, tab_type=KeyItem, )
  overworld: (len=3, )
    [000]: (typ=Equipped, actor=Weapon_Sword_043, value=800, modifier=none, )
    [001]: (typ=Equipped, actor=Weapon_Bow_001, value=2200, modifier=none, )
    [002]: (typ=Equipped, actor=Weapon_Shield_040, value=1000, modifier=none, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,Bo,Sh,Ar,Ma,Fo,Ki
    items: (len=7, )
      [000]: (idx=0, actor=Weapon_Sword_043, value=800, is_equipped=true, )
        weapon: (idx=0, modifier=none, )
      [001]: (idx=1, actor=Weapon_Bow_001, value=2200, is_equipped=true, )
        weapon: (idx=0, modifier=none, )
      [002]: (idx=2, actor=Weapon_Shield_040, value=1000, is_equipped=true, )
        weapon: (idx=0, modifier=none, )
      [003]: (idx=3, actor=Armor_001_Head, value=0, is_equipped=false, )
      [004]: (idx=4, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [005]: (idx=5, actor=Item_Roast_03, value=1, is_equipped=false, )
        food: (idx=0, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [006]: (idx=6, actor=Obj_DRStone_Get, value=1, is_equipped=false, )

----- Step[2]: pause;

game: (Running)
  screen: (Inventory)
  pouch: (count=-18, are_tabs_valid=true, num_tabs=7, holding_in_inventory=false, )
    items: (len=7, )
      [000]: (actor_name=Weapon_Sword_043, value=800, is_equipped=true, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Weapon_Bow_001, value=2200, is_equipped=true, item_type=Bow, item_use=WeaponBow, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Weapon_Shield_040, value=1000, is_equipped=true, item_type=Shield, item_use=WeaponShield, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Armor_001_Head, value=0, is_equipped=false, item_type=ArmorHead, item_use=ArmorHead, tab_idx=3, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=4, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=Item_Roast_03, value=1, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=5, tab_slot=0, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222242fd8, )
      [006]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=6, tab_slot=0, )
        node: (valid=true, pos=413, addr=0x0000002222242fd8, prev=0x0000002222243270, next=0x0000002222200068, )
    tabs: (len=7, )
      [00]: (item_idx=0, tab_type=Sword, )
      [01]: (item_idx=1, tab_type=Bow, )
      [02]: (item_idx=2, tab_type=Shield, )
      [03]: (item_idx=3, tab_type=ArmorHead, )
      [04]: (item_idx=4, tab_type=Material, )
      [05]: (item_idx=5, tab_type=Food, )
      [06]: (item_idx=6, tab_type=KeyItem, )
  overworld: (len=3, )
    [000]: (typ=Equipped, actor=Weapon_Sword_043, value=800, modifier=none, )
    [001]: (typ=Equipped, actor=Weapon_Bow_001, value=2200, modifier=none, )
    [002]: (typ=Equipped, actor=Weapon_Shield_040, value=1000, modifier=none, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,Bo,Sh,Ar,Ma,Fo,Ki
    items: (len=7, )
      [000]: (idx=0, actor=Weapon_Sword_043, value=800, is_equipped=true, )
        weapon: (idx=0, modifier=none, )
      [001]: (idx=1, actor=Weapon_Bow_001, value=2200, is_equipped=true, )
        weapon: (idx=0, modifier=none, )
      [002]: (idx=2, actor=Weapon_Shield_040, value=1000, is_equipped=true, )
        weapon: (idx=0, modifier=none, )
      [003]: (idx=3, actor=Armor_001_Head, value=0, is_equipped=false, )
      [004]: (idx=4, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [005]: (idx=5, actor=Item_Roast_03, value=1, is_equipped=false, )
        food: (idx=0, life_recover=0, duration=0, price=0, effect_id=-1, effect_level=0, )
      [006]: (idx=6, actor=Obj_DRStone_Get, value=1, is_equipped=false, )

----- Step[3]: unpause;

game: (Running)
  screen: (Overworld)
  pouch: (count=-18, are_tabs_valid=true, num_tabs=7, holding_in_inventory=false, )
    items: (len=7, )
      [000]: (actor_name=Weapon_Sword_043, value=800, is_equipped=true, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Weapon_Bow_001, value=2200, is_equipped=true, item_type=Bow, item_use=WeaponBow, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Weapon_Shield_040, value=1000, is_equipped=true, item_type=Shield, item_use=WeaponShield, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Armor_001_Head, value=0, is_equipped=false, item_type=ArmorHead, item_use=ArmorHead, tab_idx=3, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=4, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=Item_Roast_03, value=1, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=5, tab_slot=0, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222242fd8, )
      [006]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=6, tab_slot=0, )
        node: (valid=true, pos=413, addr=0x0000002222242fd8, prev=0x0000002222243270, next=0x0000002222200068, )
    tabs: (len=7, )
      [00]: (item_idx=0, tab_type=Sword, )
      [01]: (item_idx=1, tab_type=Bow, )
      [02]: (item_idx=2, tab_type=Shield, )
      [03]: (item_idx=3, tab_type=ArmorHead, )
      [04]: (item_idx=4, tab_type=Material, )
      [05]: (item_idx=5, tab_type=Food, )
      [06]: (item_idx=6, tab_type=KeyItem, )
  overworld: (len=3, )
    [000]: (typ=Equipped, actor=Weapon_Sword_043, value=800, modifier=none, )
    [001]: (typ=Equipped, actor=Weapon_Bow_001, value=2200, modifier=none, )
    [002]: (typ=Equipped, actor=Weapon_Shield_040, value=1000, modifier=none, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,Bo,Sh,Ar,Ma,Fo,Ki
    items: (len=0, )

----- Step[4]: get
 1 torch
 1 elixir
 1 torch
 1 elixir
 1 torch
 1 elixir
 1 torch
 1 elixir
 1 torch
 1 elixir
 1 torch
 1 elixir
 1 torch
 1 elixir
 1 torch
 1 elixir

game: (Running)
  screen: (Overworld)
  pouch: (count=-2, are_tabs_valid=false, num_tabs=0, holding_in_inventory=false, )
    items: (len=23, )
      [000]: (actor_name=Weapon_Sword_043, value=800, is_equipped=true, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        accessible: false
      [001]: (actor_name=Weapon_Bow_001, value=2200, is_equipped=true, item_type=Bow, item_use=WeaponBow, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Weapon_Shield_040, value=1000, is_equipped=true, item_type=Shield, item_use=WeaponShield, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Armor_001_Head, value=0, is_equipped=false, item_type=ArmorHead, item_use=ArmorHead, tab_idx=0, tab_slot=3, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=4, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=Item_Roast_03, value=1, is_equipped=false, item_type=Food, item_use=CureItem, tab_idx=0, tab_slot=5, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222242fd8, )
      [006]: (actor_name=Obj_DRStone_Get, value=1, is_equipped=false, item_type=KeyItem, item_use=ImportantItem, tab_idx=0, tab_slot=6, )
        node: (valid=true, pos=413, addr=0x0000002222242fd8, prev=0x0000002222243270, next=0x0000002222242d40, )
      [007]: (actor_name=Weapon_Sword_043, value=800, is_equipped=false, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=7, )
        node: (valid=true, pos=412, addr=0x0000002222242d40, prev=0x0000002222242fd8, next=0x0000002222242aa8, )
        dpad_accessible: false
      [008]: (actor_name=Item_Cook_C_17, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=0, tab_slot=8, )
        node: (valid=true, pos=411, addr=0x0000002222242aa8, prev=0x0000002222242d40, next=0x0000002222242810, )
      [009]: (actor_name=Weapon_Sword_043, value=800, is_equipped=false, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=9, )
        node: (valid=true, pos=410, addr=0x0000002222242810, prev=0x0000002222242aa8, next=0x0000002222242578, )
        dpad_accessible: false
      [010]: (actor_name=Item_Cook_C_17, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=0, tab_slot=10, )
        node: (valid=true, pos=409, addr=0x0000002222242578, prev=0x0000002222242810, next=0x00000022222422e0, )
      [011]: (actor_name=Weapon_Sword_043, value=800, is_equipped=false, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=11, )
        node: (valid=true, pos=408, addr=0x00000022222422e0, prev=0x0000002222242578, next=0x0000002222242048, )
        dpad_accessible: false
      [012]: (actor_name=Item_Cook_C_17, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=0, tab_slot=12, )
        node: (valid=true, pos=407, addr=0x0000002222242048, prev=0x00000022222422e0, next=0x0000002222241db0, )
      [013]: (actor_name=Weapon_Sword_043, value=800, is_equipped=false, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=13, )
        node: (valid=true, pos=406, addr=0x0000002222241db0, prev=0x0000002222242048, next=0x0000002222241b18, )
        dpad_accessible: false
      [014]: (actor_name=Item_Cook_C_17, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=0, tab_slot=14, )
        node: (valid=true, pos=405, addr=0x0000002222241b18, prev=0x0000002222241db0, next=0x0000002222241880, )
      [015]: (actor_name=Weapon_Sword_043, value=800, is_equipped=false, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=15, )
        node: (valid=true, pos=404, addr=0x0000002222241880, prev=0x0000002222241b18, next=0x00000022222415e8, )
        dpad_accessible: false
      [016]: (actor_name=Item_Cook_C_17, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=0, tab_slot=16, )
        node: (valid=true, pos=403, addr=0x00000022222415e8, prev=0x0000002222241880, next=0x0000002222241350, )
      [017]: (actor_name=Weapon_Sword_043, value=800, is_equipped=false, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=17, )
        node: (valid=true, pos=402, addr=0x0000002222241350, prev=0x00000022222415e8, next=0x00000022222410b8, )
        dpad_accessible: false
      [018]: (actor_name=Item_Cook_C_17, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=0, tab_slot=18, )
        node: (valid=true, pos=401, addr=0x00000022222410b8, prev=0x0000002222241350, next=0x0000002222240e20, )
      [019]: (actor_name=Weapon_Sword_043, value=800, is_equipped=false, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=19, )
        node: (valid=true, pos=400, addr=0x0000002222240e20, prev=0x00000022222410b8, next=0x0000002222240b88, )
        dpad_accessible: false
      [020]: (actor_name=Item_Cook_C_17, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=0, tab_slot=20, )
        node: (valid=true, pos=399, addr=0x0000002222240b88, prev=0x0000002222240e20, next=0x00000022222408f0, )
      [021]: (actor_name=Weapon_Sword_043, value=800, is_equipped=false, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=21, )
        node: (valid=true, pos=398, addr=0x00000022222408f0, prev=0x0000002222240b88, next=0x0000002222240658, )
        dpad_accessible: false
      [022]: (actor_name=Item_Cook_C_17, value=1, is_equipped=false, item_type=Food, item_use=Item, tab_idx=0, tab_slot=22, )
        node: (valid=true, pos=397, addr=0x0000002222240658, prev=0x00000022222408f0, next=0x0000002222200068, )
    tabs: (len=0, )
  overworld: (len=3, )
    [000]: (typ=Equipped, actor=Weapon_Sword_043, value=800, modifier=none, )
    [001]: (typ=Equipped, actor=Weapon_Bow_001, value=2200, modifier=none, )
    [002]: (typ=Equipped, actor=Weapon_Shield_040, value=1000, modifier=none, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,Bo,Sh,Ar,Ma,Fo,Ki
    items: (len=0, )

}
