// This has RS extension since that usually gives a minimal syntax highlighting.
//This is not an actual RS file

x!{ SKYBOOK RUNTIME SNAPSHOT V1

=====

----- Step[0]: get 1 axe[life=80000] 2 traveller-bow[life=80000] 1 pot-lid 1 pot-lid[life=80000]

game: (Running)
  screen: (Overworld)
  pouch: (count=5, are_tabs_valid=true, num_tabs=3, holding_in_inventory=false, )
    items: (len=5, )
      [000]: (actor_name=Weapon_Lsword_032, value=80000, is_equipped=true, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Weapon_Bow_001, value=80000, is_equipped=true, item_type=Bow, item_use=WeaponBow, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Weapon_Bow_001, value=80000, is_equipped=false, item_type=Bow, item_use=WeaponBow, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Weapon_Shield_040, value=1000, is_equipped=true, item_type=Shield, item_use=WeaponShield, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Weapon_Shield_040, value=80000, is_equipped=false, item_type=Shield, item_use=WeaponShield, tab_idx=2, tab_slot=1, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222200068, )
    tabs: (len=3, )
      [00]: (item_idx=0, tab_type=Sword, )
      [01]: (item_idx=1, tab_type=Bow, )
      [02]: (item_idx=3, tab_type=Shield, )
  overworld: (len=3, )
    [000]: (typ=Equipped, actor=Weapon_Lsword_032, value=80000, modifier=none, )
    [001]: (typ=Equipped, actor=Weapon_Bow_001, value=80000, modifier=none, )
    [002]: (typ=Equipped, actor=Weapon_Shield_040, value=1000, modifier=none, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,Bo,Sh,__,__,__,__
    items: (len=5, )
      [000]: (idx=0, actor=Weapon_Lsword_032, value=80000, is_equipped=true, )
        weapon: (idx=0, modifier=none, )
      [001]: (idx=1, actor=Weapon_Bow_001, value=80000, is_equipped=true, )
        weapon: (idx=0, modifier=none, )
      [002]: (idx=2, actor=Weapon_Bow_001, value=80000, is_equipped=false, )
        weapon: (idx=1, modifier=none, )
      [003]: (idx=3, actor=Weapon_Shield_040, value=1000, is_equipped=true, )
        weapon: (idx=0, modifier=none, )
      [004]: (idx=4, actor=Weapon_Shield_040, value=80000, is_equipped=false, )
        weapon: (idx=1, modifier=none, )

----- Step[1]: drop 1 weapon 2 bow 2 shield

game: (Running)
  screen: (Inventory)
  pouch: (count=5, are_tabs_valid=true, num_tabs=3, holding_in_inventory=false, )
    items: (len=5, )
      [000]: (actor_name=Weapon_Lsword_032, value=0, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        in_inventory: false
      [001]: (actor_name=Weapon_Bow_001, value=0, is_equipped=false, item_type=Bow, item_use=WeaponBow, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
        in_inventory: false
      [002]: (actor_name=Weapon_Bow_001, value=0, is_equipped=false, item_type=Bow, item_use=WeaponBow, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        in_inventory: false
      [003]: (actor_name=Weapon_Shield_040, value=0, is_equipped=false, item_type=Shield, item_use=WeaponShield, tab_idx=2, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
        in_inventory: false
      [004]: (actor_name=Weapon_Shield_040, value=0, is_equipped=false, item_type=Shield, item_use=WeaponShield, tab_idx=2, tab_slot=1, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222200068, )
        in_inventory: false
    tabs: (len=3, )
      [00]: (item_idx=0, tab_type=Sword, )
      [01]: (item_idx=1, tab_type=Bow, )
      [02]: (item_idx=3, tab_type=Shield, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,Bo,Sh,__,__,__,__
    items: (len=0, )

}
