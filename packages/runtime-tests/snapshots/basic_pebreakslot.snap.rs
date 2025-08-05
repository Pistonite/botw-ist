// This has RS extension since that usually gives a minimal syntax highlighting.
//This is not an actual RS file

x!{ SKYBOOK RUNTIME SNAPSHOT V1

=====

----- Step[0]: :discovered [bow, shield]

game: (Running)
  screen: (Overworld)
  pouch: (count=0, are_tabs_valid=true, num_tabs=0, holding_in_inventory=false, trial=false, )
    items: (len=0, )
    tabs: (len=0, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,Bo,Sh,__,__,__,__
    items: (len=0, )

----- Step[1]: get
  1 torch 1 axe 1 hammer
  1 apple 1 banana 1 shroom

game: (Running)
  screen: (Overworld)
  pouch: (count=6, are_tabs_valid=true, num_tabs=4, holding_in_inventory=false, trial=false, )
    items: (len=6, )
      [000]: (actor_name=Weapon_Sword_043, value=800, is_equipped=true, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Weapon_Lsword_032, value=4700, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Weapon_Lsword_031, value=4000, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=3, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
      [004]: (actor_name=Item_Fruit_H, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=3, tab_slot=1, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=Item_Mushroom_E, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=3, tab_slot=2, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
    tabs: (len=4, )
      [00]: (item_idx=0, tab_type=Sword, )
      [01]: (item_idx=-1, tab_type=Bow, )
      [02]: (item_idx=-1, tab_type=Shield, )
      [03]: (item_idx=3, tab_type=Material, )
  overworld: (len=1, )
    [000]: (typ=Equipped, actor=Weapon_Sword_043, value=800, modifier=none, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,Bo,Sh,__,Ma,__,__
    items: (len=6, )
      [000]: (idx=0, actor=Weapon_Sword_043, value=800, is_equipped=true, )
        weapon: (idx=0, modifier=none, )
      [001]: (idx=1, actor=Weapon_Lsword_032, value=4700, is_equipped=false, )
        weapon: (idx=1, modifier=none, )
      [002]: (idx=2, actor=Weapon_Lsword_031, value=4000, is_equipped=false, )
        weapon: (idx=2, modifier=none, )
      [003]: (idx=3, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [004]: (idx=4, actor=Item_Fruit_H, value=1, is_equipped=false, )
      [005]: (idx=5, actor=Item_Mushroom_E, value=1, is_equipped=false, )

----- Step[2]: eat all apple all shroom

game: (Running)
  screen: (Inventory)
  pouch: (count=6, are_tabs_valid=true, num_tabs=4, holding_in_inventory=false, trial=false, )
    items: (len=6, )
      [000]: (actor_name=Weapon_Sword_043, value=800, is_equipped=true, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Weapon_Lsword_032, value=4700, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Weapon_Lsword_031, value=4000, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
      [003]: (actor_name=Item_Fruit_A, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=3, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
        in_inventory: false
      [004]: (actor_name=Item_Fruit_H, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=3, tab_slot=1, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=Item_Mushroom_E, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=3, tab_slot=2, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
        in_inventory: false
    tabs: (len=4, )
      [00]: (item_idx=0, tab_type=Sword, )
      [01]: (item_idx=-1, tab_type=Bow, )
      [02]: (item_idx=-1, tab_type=Shield, )
      [03]: (item_idx=3, tab_type=Material, )
  overworld: (len=1, )
    [000]: (typ=Equipped, actor=Weapon_Sword_043, value=800, modifier=none, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,Bo,Sh,__,Ma,__,__
    items: (len=4, )
      [000]: (idx=0, actor=Weapon_Sword_043, value=800, is_equipped=true, )
        weapon: (idx=0, modifier=none, )
      [001]: (idx=1, actor=Weapon_Lsword_032, value=4700, is_equipped=false, )
        weapon: (idx=1, modifier=none, )
      [002]: (idx=2, actor=Weapon_Lsword_031, value=4000, is_equipped=false, )
        weapon: (idx=2, modifier=none, )
      [003]: (idx=3, actor=Item_Fruit_H, value=1, is_equipped=false, )

----- Step[3]: entangle hammer

game: (Running)
  screen: (Inventory)
  pouch: (count=6, are_tabs_valid=true, num_tabs=4, holding_in_inventory=false, trial=false, )
    items: (len=6, )
      [000]: (actor_name=Weapon_Sword_043, value=800, is_equipped=true, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Weapon_Lsword_032, value=4700, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Weapon_Lsword_031, value=4000, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        entangled: true
      [003]: (actor_name=Item_Fruit_A, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=3, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
        in_inventory: false
      [004]: (actor_name=Item_Fruit_H, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=3, tab_slot=1, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=Item_Mushroom_E, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=3, tab_slot=2, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
        in_inventory: false
        entangled: true
    tabs: (len=4, )
      [00]: (item_idx=0, tab_type=Sword, )
      [01]: (item_idx=-1, tab_type=Bow, )
      [02]: (item_idx=-1, tab_type=Shield, )
      [03]: (item_idx=3, tab_type=Material, )
  overworld: (len=1, )
    [000]: (typ=Equipped, actor=Weapon_Sword_043, value=800, modifier=none, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,Bo,Sh,__,Ma,__,__
    items: (len=4, )
      [000]: (idx=0, actor=Weapon_Sword_043, value=800, is_equipped=true, )
        weapon: (idx=0, modifier=none, )
      [001]: (idx=1, actor=Weapon_Lsword_032, value=4700, is_equipped=false, )
        weapon: (idx=1, modifier=none, )
      [002]: (idx=2, actor=Weapon_Lsword_031, value=4000, is_equipped=false, )
        weapon: (idx=2, modifier=none, )
      [003]: (idx=3, actor=Item_Fruit_H, value=1, is_equipped=false, )

----- Step[4]: :targeting <empty>[category=material, row=1, col=3]

<same>
----- Step[5]: drop hammer

game: (Running)
  screen: (Inventory)
  pouch: (count=6, are_tabs_valid=true, num_tabs=4, holding_in_inventory=false, trial=false, )
    items: (len=6, )
      [000]: (actor_name=Weapon_Sword_043, value=800, is_equipped=true, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Weapon_Lsword_032, value=4700, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Weapon_Lsword_031, value=4000, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x00000022222437a0, )
        entangled: true
      [003]: (actor_name=Item_Fruit_A, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=3, tab_slot=0, )
        node: (valid=true, pos=416, addr=0x00000022222437a0, prev=0x0000002222243a38, next=0x0000002222243508, )
        in_inventory: false
        holding: 1
      [004]: (actor_name=Item_Fruit_H, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=3, tab_slot=1, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x00000022222437a0, next=0x0000002222243270, )
      [005]: (actor_name=Item_Mushroom_E, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=3, tab_slot=2, )
        node: (valid=true, pos=414, addr=0x0000002222243270, prev=0x0000002222243508, next=0x0000002222200068, )
        in_inventory: false
        entangled: true
    tabs: (len=4, )
      [00]: (item_idx=0, tab_type=Sword, )
      [01]: (item_idx=-1, tab_type=Bow, )
      [02]: (item_idx=-1, tab_type=Shield, )
      [03]: (item_idx=3, tab_type=Material, )
  overworld: (len=1, )
    [000]: (typ=Equipped, actor=Weapon_Sword_043, value=800, modifier=none, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,Bo,Sh,__,Ma,__,__
    items: (len=4, )
      [000]: (idx=0, actor=Weapon_Sword_043, value=800, is_equipped=true, )
        weapon: (idx=0, modifier=none, )
      [001]: (idx=1, actor=Weapon_Lsword_032, value=4700, is_equipped=false, )
        weapon: (idx=1, modifier=none, )
      [002]: (idx=2, actor=Weapon_Lsword_031, value=4000, is_equipped=false, )
        weapon: (idx=2, modifier=none, )
      [003]: (idx=3, actor=Item_Fruit_H, value=1, is_equipped=false, )

----- Step[6]: drop

game: (Running)
  screen: (Overworld)
  pouch: (count=3, are_tabs_valid=true, num_tabs=4, holding_in_inventory=false, trial=false, )
    items: (len=4, )
      [000]: (actor_name=Weapon_Sword_043, value=800, is_equipped=true, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Weapon_Lsword_032, value=4700, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Weapon_Lsword_031, value=4000, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x0000002222243508, )
      [003]: (actor_name=Item_Fruit_H, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=3, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x0000002222243a38, next=0x0000002222200068, )
    tabs: (len=4, )
      [00]: (item_idx=0, tab_type=Sword, )
      [01]: (item_idx=-1, tab_type=Bow, )
      [02]: (item_idx=-1, tab_type=Shield, )
      [03]: (item_idx=3, tab_type=Material, )
  overworld: (len=2, )
    [000]: (typ=Equipped, actor=Weapon_Sword_043, value=800, modifier=none, )
    [001]: (typ=GroundItem, actor=Item_Fruit_A, despawning=false, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,Bo,Sh,__,Ma,__,__
    items: (len=4, )
      [000]: (idx=0, actor=Weapon_Sword_043, value=800, is_equipped=true, )
        weapon: (idx=0, modifier=none, )
      [001]: (idx=1, actor=Weapon_Lsword_032, value=4700, is_equipped=false, )
        weapon: (idx=1, modifier=none, )
      [002]: (idx=2, actor=Weapon_Lsword_031, value=4000, is_equipped=false, )
        weapon: (idx=2, modifier=none, )
      [003]: (idx=3, actor=Item_Fruit_H, value=1, is_equipped=false, )

----- Step[7]: hold

game: (Running)
  screen: (Inventory)
  pouch: (count=3, are_tabs_valid=true, num_tabs=4, holding_in_inventory=true, trial=false, )
    items: (len=4, )
      [000]: (actor_name=Weapon_Sword_043, value=800, is_equipped=true, item_type=Sword, item_use=WeaponSmallSword, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Weapon_Lsword_032, value=4700, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222243a38, )
      [002]: (actor_name=Weapon_Lsword_031, value=4000, is_equipped=false, item_type=Sword, item_use=WeaponLargeSword, tab_idx=0, tab_slot=2, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222243cd0, next=0x0000002222243508, )
      [003]: (actor_name=Item_Fruit_H, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=3, tab_slot=0, )
        node: (valid=true, pos=415, addr=0x0000002222243508, prev=0x0000002222243a38, next=0x0000002222200068, )
    tabs: (len=4, )
      [00]: (item_idx=0, tab_type=Sword, )
      [01]: (item_idx=-1, tab_type=Bow, )
      [02]: (item_idx=-1, tab_type=Shield, )
      [03]: (item_idx=3, tab_type=Material, )
  overworld: (len=2, )
    [000]: (typ=Equipped, actor=Weapon_Sword_043, value=800, modifier=none, )
    [001]: (typ=GroundItem, actor=Item_Fruit_A, despawning=false, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: Sw,Bo,Sh,__,Ma,__,__
    items: (len=4, )
      [000]: (idx=0, actor=Weapon_Sword_043, value=800, is_equipped=true, )
        weapon: (idx=0, modifier=none, )
      [001]: (idx=1, actor=Weapon_Lsword_032, value=4700, is_equipped=false, )
        weapon: (idx=1, modifier=none, )
      [002]: (idx=2, actor=Weapon_Lsword_031, value=4000, is_equipped=false, )
        weapon: (idx=2, modifier=none, )
      [003]: (idx=3, actor=Item_Fruit_H, value=1, is_equipped=false, )

}
