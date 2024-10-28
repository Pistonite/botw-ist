# COMMANDS

A WIP list of commands...


core commands:

  # PMDM

  # Stage 1
  !core-add ITEM:
    Add ITEM to PMDM
  !core-add-one ITEM:
    Add one ITEM to PMDM
  !core-remove-held:
    Remove held items
  !core-hold SLOT:
    Perform HOLD/DROP action on SLOT
  !core-unhold:
    Perform UNHOLD
  !core-equip SLOT:
    Perform EQUIP action on SLOT
  !core-unequip SLOT:
    Perform UNEQUIP action on SLOT

  # Stage 2
  !core-eat-once SLOT:
    Perform EAT action on SLOT once
  !core-sell SLOT:
    Perform SELL action on SLOT
  !core-use-weapon:
    Use the weapon
  !core-use-shield:
    Use the shield
  !core-shoot-arrow:
    Use the bow to shoow an arrow
  !core-sync-gdt:
    Sync PMDM to GDT

  # Stage 3
  !core-check-can-cook:
    Check if the player can cook (inventory space)
  !core-check-can-add ITEM:
    Check if ITEM can be added to inventory
  !core-check-has ITEM:
    Check if ITEM currently exists in inventory 
  !core-check-can-remove ITEM:
    Check if ITEM can be removed from inventory
  !core-check-can-access-inventory:
    Check if the player can access the inventory (mCount != 0)
    
  !core-sort TAB_INDEX:
    Perform SORT while on TAB_INDEX
  !core-set-gdt FLAG VALUE:
    Set GDT FLAG to VALUE

  

  extra:
    set_state() (many setter methods...)
    get_state() (many getter methods...)


runtime:
  inventory (core)
  overworld:
    equipped
    dropped
  saves
  features:
    strict-mode:
      Disallow deprecated actions implemented by runtime instead of the core
    core-check-can-cook:
      checks if the player has enough inventory space to cook
      default: true
    core-check-can-add
    core-check-can-remove
    core-check-can-access-inventory
    area-check:
      checks if the current area has the items
      default: false, unless (area)
    area-check-shop:
      checks if the current area has a shop
      default: false, unless (area)
    area-check-shop-buy:
      checks if the current area has a shop that sells the items
      default: false, unless (area)
    area-check-cook
      checks if the current area has a cooking pot
      default: false, unless (area)
    ovwd-check-dropped:
      checks if the overworld has the items
      default: true
    ovwd-auto-despawn-drop-limit:
      automatically despawn dropped items over 10
      default: true

    sellable-check:
      check if item is sellable for SELL actions
    edible-check:
      check if item is edible for EAT actions
    holdable-check:
      check if item is holdable for HOLD actions
    holdable-check-limit:
      check if there's room to hold item

annotations:
  global:
    // I guess these are technically features?
    (FEATURE)
      Enable Feature
    (no-FEATURE)
      Disable Feature
    (menu-overload)
      Start menu overload
    (no-menu-overload)
      Stop menu overload
    (drop-after-dialog)
      Make holding items drop when dialog scope deactivates, instead of when inventory scope deactivates
      (Effect triggers only once)
    (drop-bomb-arrow)
      Make bomb arrows drop on ground after shooting them (i.e. rain)
    (no-drop-bomb-arrow)
      Stop bomb arrows from dropping on ground after shooting them

    // other
    (obtained PROGRESSION_FLAG)
      Set PROGRESSION_FLAG (calls !core-set-gdt interally)
    (group) { 
      (note) NOTES;
      COMMANDS
    }
      Group commands to be one action in view mode
    (note) NOTES
      Set notes for the next command/action
    // comment
    # comment
    (area) AREA {
      AREA_ANOTATIONS
    }
      Declare an area

    (autoist) {
      AUTOIST_ANOTATIONS
    }
      Declare Auto IST

  area:
    (has) ITEMS:
      args:
        ITEMS: ItemListInfinityForAdd
    (sells) ITEMS:
    (has-statue)
    (has-pot)
    (has-fire)
       Allow roasting without your own fire
    (freezing)
       Items on the ground can freeze by themselves, allowing freeze without your own ice
    (has-hot-spring)
       Eggs can be boiled
    (time-from) AREA X;
    

  
       

   

rtcommands:
  !requires-not-strict:
    Error if strict-mode is enabled
  !area-check ITEMS:
    Check current area has ITEMS
  !area-deduct ITEMS:
    Remove ITEMS from current area
  !area-check-shop-buy ITEMS:
    Check current area has shop that sells ITEMS
  !area-deduct-shop ITEMS:
    Remove ITEMS from shops in current area
  !area-check-shop:
    Check can sell in current area
  !area-check-cook:
    Check current area has a cooking pot
  !area-check-statue:
    Check current area has a goddess statue
  !area-check-hot-spring:
    Check current area has a hot spring
  !area-check-freezing:
    Check current area is freezing
  
  !ovwd-check-dropped ITEMS:
    Check if ITEMS are currently dropped in the overworld
  !edible-check ITEM:
    Check if ITEM is edible
  !sellable-check ITEM:
    Check if ITEM is sellable
  !ovwd-remove ITEM:
    Remove ITEM from the overworld
  !ovwd-hold ITEM:
    Schedule HOLD in overworld
  !ovwd-unhold:
    Schedule Unhold items in the overworld, or unhold immediately
  !ovwd-equip-<type> ITEM:
    Schedule Switch to ITEM in the overworld
  !ovwd-unequip-<type>:
    Schedule Unequip in the overworld
  !ovwd-drop-<type>:
    Schedule Drop equipment in the overworld
  !ovwd-reset:
    Reset the overworld items

  !trade:
    Trade 1 heart or stamina

  !auto-scope SCOPE:
    Attempt to automatically adjust the scope

  !go AREA:
    Go to an area


actions:
  # add
  get ITEMS:
    args:
      ITEMS: ItemListForAdd
    cmds:
    - !auto-scope game not-paused
    - !area-check ITEMS
    - !area-deduct ITEMS
    - loop ITEM in ITEMS:
        !core-add ITEM
  buy ITEMS:
    args:
      ITEMS: ItemListForAdd
    cmds:
    - !auto-scope game dialog
    - !area-check-shop-buy ITEMS
    - !area-deduct-shop ITEMS
    - loop ITEM in ITEMS:
        !core-add ITEM
  pick-up ITEMS:
    args:
      ITEMS: ItemListForRemove
    cmds:
    - !ovwd-check-dropped ITEMS
    - !ovwd-remove ITEMS
    - loop ITEM in ITEMS:
        loop item amount
          !core-add-one ITEM
  cook ITEMS: (deprecated)
    args:
      ITEMS: ItemListForAdd
    cmds:
    - add ITEMS
  cook:
    cmds:
    - (feature check-area-cook) !area-check-cook
    - (feature core-check-can-cook)  !core-check-can-cook
    - !auto-scope game not-paused
    - <cook-simulator HELD_ITEMS>
    - !core-remove-held
    - !core-add-one COOKED_ITEM
  cook-with ITEMS:
    args:
      ITEMS: ItemListForRemove
    cmds:
    - hold ITEMS
    - cook

  # Remove
  drop:
    cmds:
    - !auto-scope game not-paused
    - !ovwd-unhold
    - !ovwd-add HELD_ITEMS
    - !core-remove-held
  drop ITEMS:
  put-aside ITEMS:
    args:
      ITEMS: ItemListForRemove
    cmds:
    - loop ITEM in ITEMS:
        if ITEM is droppable:
            !core-hold SLOT
            !ovwd-drop-<type>
        else:
            hold ITEM
            drop

  eat ITEMS:
    args:
      ITEMS: ItemListForRemove
    cmds:
    - !auto-scope game inventory
    - loop SLOT in ITEMS:
        loop item amount
          !core-eat-once SLOT

  sell ITEMS:
    args:
      ITEMS: ItemListForRemove
    cmds:
    - !auto-scope game dialog
    - loop ITEM in ITEMS:
        !sellable-check ITEM
    - !area-check-shop
    - loop SLOT in ITEMS:
        !core-sell SLOT

  trade X hearts:
  trade X stamina:
    cmds:
    - !auto-scope game dialog
    - !area-check-statue
    - loop X:
        !trade X
      
      

  remove ITEMS: (deprecated)
  with ITEMS: 
    args:
      ITEMS: ItemListForRemove
    - !requires-non-strict
    - !auto-scope game inventory
    - <runtime implementation>

  # holding
  hold ITEMS:
    args:
      ITEMS: ItemListForRemove
    cmds:
    - !auto-scope game inventory
    - loop SLOT in ITEMS:
        !holdable-check SLOT
        !holdable-check-limit SLOT
        !core-hold SLOT
        if not overload: !ovwd-hold ITEM
  unhold:
  put-away:
    cmds:
    - !auto-scope game
    - !ovwd-unhold
    - !core-unhold
  dnp ITEMS:
    cmds:
    - drop ITEMS
    - pick-up ITEMS
  
  # equipment
  equip ITEM:
    args:
      ITEM: ItemForRemove
    cmds:
    - !auto-scope game inventory
    - !core-equip SLOT
    - if not overload: !ovwd-equip-<type> ITEM
  unequip ITEM:
    args:
      ITEM: ItemForRemove
    cmds:
    - !auto-scope game inventory
    - !core-unequip SLOT
    - if not overload: !ovwd-unequip-<type> ITEM

  unequip-the TYPE:
    args:
      TYPE: EquipmentType
    cmds:
    - !auto-scope game inventory
    - !core-unequip SLOT
    - if not overload: !ovwd-unequip-<type>

  shoot-arrow X:
    cmds:
    - !auto-scope game not-paused
    - loop X:
        !core-shoot-arrow
        if normal/ancient or allow drop bomb arrow:
          !ovwd-add ARROW_TYPE

  use-weapon X:
    cmds:
    - !auto-scope game not-paused
    - loop X:
        !core-use-weapon
  use-shield X:
    cmds:
    - !auto-scope game not-paused
    - loop X:
        !core-use-shield

  # sorting
  sort TAB:
    cmds:
    - !auto-scope game inventory
    - !core-sort TAB_INDEX

  # prompt entanglement
  entangle TAB:
    cmds:
    - !entangle TAB

  # savefiles
  save:
    cmds:
    - !auto-scope game inventory
    - !save
  
  save-as NAME:
    cmds:
    - !auto-scope game inventory
    - !save NAME

  reload:
    cmds:
    - !auto-scope game inventory
    - !reload SAVE # note: will change scope to game
    - !ovwd-reset
  reload SAVE:
    cmds:
    - !auto-scope game inventory
    - !reload-auto SAVE
    - !ovwd-reset

  new-game:
    cmds:
    - !new-game

  close-game:
    - !close-game

  # overworld
  roast ITEMS:
  bake ITEMS:
    args:
      ITEMS: ItemListForRemove
    cmds:
    - !auto-scope game not-paused
    - !ovwd-check-dropped ITEMS
    - loop ITEM in ITEMS:
        !ovwd-remove ITEM
        !ovwd-add ROASTED_ITEM
  boil ITEMS:
    args:
      ITEMS: ItemListForRemove
    cmds:
    - !auto-scope game not-paused
    - !ovwd-check-dropped ITEMS
    - loop ITEM in ITEMS:
        !ovwd-remove ITEM
        !ovwd-add ROASTED_ITEM

  freeze ITEMS:
    args:
      ITEMS: ItemListForRemove
    cmds:
    - !auto-scope game not-paused
    - !ovwd-check-dropped ITEMS
    - loop ITEM in ITEMS:
        !ovwd-remove ITEM
        !ovwd-add FROZEN_ITEM

  go-to AREA:
    cmds:
    - !ovwd-reset
    - !go AREA

  # other
  !break-slot X:
    cmds:
    - !requires-non-strict
    - loop X:
        <runtime implementation>


break slot will be like
```
hold 1 apple 1 orange
(drop-after-dialog)
sell 1 apple 1 orange

```
