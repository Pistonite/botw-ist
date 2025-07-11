// This has RS extension since that usually gives a minimal syntax highlighting.
//This is not an actual RS file

x!{ SKYBOOK RUNTIME SNAPSHOT V1

runtime error: : this item cannot be held
  span: 93..103
-----
fire-arrow
-----
=====

----- Step[0]: get 2 apple 3 pepper

game: (Running)
  screen: (Overworld)
  pouch: (count=2, are_tabs_valid=true, num_tabs=1, holding_in_inventory=false, )
    items: (len=2, )
      [000]: (actor_name=Item_Fruit_A, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Fruit_I, value=3, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,__
    items: (len=2, )
      [000]: (idx=0, actor=Item_Fruit_A, value=2, is_equipped=false, )
      [001]: (idx=1, actor=Item_Fruit_I, value=3, is_equipped=false, )

----- Step[1]: hold pepper

game: (Running)
  screen: (Inventory)
  pouch: (count=2, are_tabs_valid=true, num_tabs=1, holding_in_inventory=true, )
    items: (len=2, )
      [000]: (actor_name=Item_Fruit_A, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Fruit_I, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222200068, )
        holding: 1
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,__
    items: (len=2, )
      [000]: (idx=0, actor=Item_Fruit_A, value=2, is_equipped=false, )
      [001]: (idx=1, actor=Item_Fruit_I, value=3, is_equipped=false, )

----- Step[2]: hold 2 apple

game: (Running)
  screen: (Inventory)
  pouch: (count=2, are_tabs_valid=true, num_tabs=1, holding_in_inventory=true, )
    items: (len=2, )
      [000]: (actor_name=Item_Fruit_A, value=0, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        holding: 2
      [001]: (actor_name=Item_Fruit_I, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222200068, )
        holding: 1
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,__
    items: (len=2, )
      [000]: (idx=0, actor=Item_Fruit_A, value=2, is_equipped=false, )
      [001]: (idx=1, actor=Item_Fruit_I, value=3, is_equipped=false, )

----- Step[3]: unhold

game: (Running)
  screen: (Inventory)
  pouch: (count=2, are_tabs_valid=true, num_tabs=1, holding_in_inventory=false, )
    items: (len=2, )
      [000]: (actor_name=Item_Fruit_A, value=2, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Fruit_I, value=3, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,__
    items: (len=2, )
      [000]: (idx=0, actor=Item_Fruit_A, value=2, is_equipped=false, )
      [001]: (idx=1, actor=Item_Fruit_I, value=3, is_equipped=false, )

----- Step[4]: hold 1 apple

game: (Running)
  screen: (Inventory)
  pouch: (count=2, are_tabs_valid=true, num_tabs=1, holding_in_inventory=true, )
    items: (len=2, )
      [000]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
        holding: 1
      [001]: (actor_name=Item_Fruit_I, value=3, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,__
    items: (len=2, )
      [000]: (idx=0, actor=Item_Fruit_A, value=2, is_equipped=false, )
      [001]: (idx=1, actor=Item_Fruit_I, value=3, is_equipped=false, )

----- Step[5]: drop

game: (Running)
  screen: (Overworld)
  pouch: (count=2, are_tabs_valid=true, num_tabs=1, holding_in_inventory=false, )
    items: (len=2, )
      [000]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222200068, next=0x0000002222243cd0, )
      [001]: (actor_name=Item_Fruit_I, value=3, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=0, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222200068, )
    tabs: (len=1, )
      [00]: (item_idx=0, tab_type=Material, )
  overworld: (len=1, )
    [000]: (typ=GroundItem, actor=Item_Fruit_A, despawning=false, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,__,__,__,Ma,__,__
    items: (len=2, )
      [000]: (idx=0, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [001]: (idx=1, actor=Item_Fruit_I, value=3, is_equipped=false, )

----- Step[6]: get 2 fire-arrow

game: (Running)
  screen: (Overworld)
  pouch: (count=3, are_tabs_valid=true, num_tabs=2, holding_in_inventory=false, )
    items: (len=3, )
      [000]: (actor_name=FireArrow, value=2, is_equipped=true, item_type=Arrow, item_use=Item, tab_idx=0, tab_slot=5, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222200068, next=0x0000002222243f68, )
      [001]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222243a38, next=0x0000002222243cd0, )
      [002]: (actor_name=Item_Fruit_I, value=3, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Bow, )
      [01]: (item_idx=1, tab_type=Material, )
  overworld: (len=1, )
    [000]: (typ=GroundItem, actor=Item_Fruit_A, despawning=false, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,Bo,__,__,Ma,__,__
    items: (len=3, )
      [000]: (idx=0, actor=FireArrow, value=2, is_equipped=true, )
      [001]: (idx=1, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [002]: (idx=2, actor=Item_Fruit_I, value=3, is_equipped=false, )

----- Step[7]: hold fire-arrow

game: (Running)
  screen: (Inventory)
  pouch: (count=3, are_tabs_valid=true, num_tabs=2, holding_in_inventory=true, )
    items: (len=3, )
      [000]: (actor_name=FireArrow, value=2, is_equipped=true, item_type=Arrow, item_use=Item, tab_idx=0, tab_slot=5, )
        node: (valid=true, pos=417, addr=0x0000002222243a38, prev=0x0000002222200068, next=0x0000002222243f68, )
      [001]: (actor_name=Item_Fruit_A, value=1, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=1, tab_slot=0, )
        node: (valid=true, pos=419, addr=0x0000002222243f68, prev=0x0000002222243a38, next=0x0000002222243cd0, )
      [002]: (actor_name=Item_Fruit_I, value=3, is_equipped=false, item_type=Material, item_use=CureItem, tab_idx=1, tab_slot=1, )
        node: (valid=true, pos=418, addr=0x0000002222243cd0, prev=0x0000002222243f68, next=0x0000002222200068, )
    tabs: (len=2, )
      [00]: (item_idx=0, tab_type=Bow, )
      [01]: (item_idx=1, tab_type=Material, )
  overworld: (len=1, )
    [000]: (typ=GroundItem, actor=Item_Fruit_A, despawning=false, )
  gdt: (weapons=8, bows=5, shields=4, )
    discovered_tabs: __,Bo,__,__,Ma,__,__
    items: (len=3, )
      [000]: (idx=0, actor=FireArrow, value=2, is_equipped=true, )
      [001]: (idx=1, actor=Item_Fruit_A, value=1, is_equipped=false, )
      [002]: (idx=2, actor=Item_Fruit_I, value=3, is_equipped=false, )

}
