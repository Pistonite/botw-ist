use std::error::Error;

use blueflame::{
    boot::init_memory,
    processor::Processor,
    structs::{GameDataItemIter, PouchItemType, PouchItemTypeLookup},
    Core,
};

#[test]
fn test_one_simple_item() -> Result<(), Box<dyn Error>> {
    let data = std::fs::read("./test_files/program.blfm").unwrap();
    let program = blueflame_program::unpack_blueflame(&data).unwrap();
    let (mut mem, mut prox) =
        init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 20000000).unwrap();
    let mut cpu = Processor::default();
    let mut core = Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut prox,
    };
    core.setup().unwrap();

    // Setup
    core.pmdm_item_get("Item_Ore_A", 5, 0, 0)?;
    core.save_to_game_data()?;

    // Create iterator and read
    let pouch_item_lookup = PouchItemTypeLookup::new()?;
    let mut iter = GameDataItemIter::new(&core, pouch_item_lookup);
    let next = iter.next_item()?;

    assert_eq!("Item_Ore_A", next.name);
    assert_eq!(5, next.value);
    assert_eq!(PouchItemType::Material, next.r#type);

    Ok(())
}

#[test]
fn test_many_simple_items() -> Result<(), Box<dyn Error>> {
    let data = std::fs::read("./test_files/program.blfm").unwrap();
    let program = blueflame_program::unpack_blueflame(&data).unwrap();
    let (mut mem, mut prox) =
        init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 20000000).unwrap();
    let mut cpu = Processor::default();
    let mut core = Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut prox,
    };
    core.setup().unwrap();

    // Setup
    core.pmdm_item_get("Item_Ore_A", 5, 0, 0)?;
    core.pmdm_item_get("Obj_DRStone_Get", 1, 0, 0)?;
    core.pmdm_item_get("PlayerStole2", 1, 0, 0)?;
    core.pmdm_item_get("Obj_DungeonClearSeal", 1, 0, 0)?;
    core.save_to_game_data()?;

    // Create iterator and read
    let pouch_item_lookup = PouchItemTypeLookup::new()?;
    let mut iter = GameDataItemIter::new(&core, pouch_item_lookup);

    let next = iter.next_item()?;
    assert_eq!("Item_Ore_A", next.name);
    assert_eq!(5, next.value);
    assert_eq!(PouchItemType::Material, next.r#type);

    let next = iter.next_item()?;
    assert_eq!("Obj_DRStone_Get", next.name);
    assert_eq!(1, next.value);
    assert_eq!(PouchItemType::KeyItem, next.r#type);

    let next = iter.next_item()?;
    assert_eq!("PlayerStole2", next.name);
    assert_eq!(1, next.value);
    assert_eq!(PouchItemType::KeyItem, next.r#type);

    let next = iter.next_item()?;
    assert_eq!("Obj_DungeonClearSeal", next.name);
    assert_eq!(1, next.value);
    assert_eq!(PouchItemType::KeyItem, next.r#type);

    Ok(())
}

#[test]
fn test_weapon_items() -> Result<(), Box<dyn Error>> {
    let data = std::fs::read("./test_files/program.blfm").unwrap();
    let program = blueflame_program::unpack_blueflame(&data).unwrap();
    let (mut mem, mut prox) =
        init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 20000000).unwrap();
    let mut cpu = Processor::default();
    let mut core = Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut prox,
    };
    core.setup().unwrap();

    // Setup
    core.pmdm_item_get("Weapon_Sword_058", 2700, 0b00100, 10)?;
    core.pmdm_item_get("Weapon_Bow_038", 2000, 0b01000, 15)?;
    core.pmdm_item_get("Weapon_Shield_004", 500, 0, 0)?;
    core.save_to_game_data()?;

    // Create iterator and read
    let pouch_item_lookup = PouchItemTypeLookup::new()?;
    let mut iter = GameDataItemIter::new(&core, pouch_item_lookup);

    let next = iter.next_item()?;
    assert_eq!("Weapon_Sword_058", next.name);
    assert_eq!(2700, next.value);
    assert_eq!(PouchItemType::Sword, next.r#type);
    let weapon_data = next.weapon_data.unwrap();
    assert_eq!(0b00100, weapon_data.modifier);
    assert_eq!(10, weapon_data.modifier_value);

    let next = iter.next_item()?;
    assert_eq!("Weapon_Bow_038", next.name);
    assert_eq!(2000, next.value);
    assert_eq!(PouchItemType::Bow, next.r#type);
    let weapon_data = next.weapon_data.unwrap();
    assert_eq!(0b01000, weapon_data.modifier);
    assert_eq!(15, weapon_data.modifier_value);

    let next = iter.next_item()?;
    assert_eq!("Weapon_Shield_004", next.name);
    assert_eq!(500, next.value);
    assert_eq!(PouchItemType::Shield, next.r#type);
    let weapon_data = next.weapon_data.unwrap();
    assert_eq!(0, weapon_data.modifier);
    assert_eq!(0, weapon_data.modifier_value);

    Ok(())
}

#[test]
fn test_food_items() -> Result<(), Box<dyn Error>> {
    let data = std::fs::read("./test_files/program.blfm").unwrap();
    let program = blueflame_program::unpack_blueflame(&data).unwrap();
    let (mut mem, mut prox) =
        init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 20000000).unwrap();
    let mut cpu = Processor::default();
    let mut core = Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut prox,
    };
    core.setup().unwrap();

    // Setup
    core.cook_item_get(
        "Item_Cook_O_01",
        0.0,
        4,
        0,
        blueflame::structs::CookEffectId::LifeRecover,
        4.0,
        false,
    )?;
    core.save_to_game_data()?;

    // Create iterator and read
    let pouch_item_lookup = PouchItemTypeLookup::new()?;
    let mut iter = GameDataItemIter::new(&core, pouch_item_lookup);

    let next = iter.next_item()?;
    assert_eq!("Item_Cook_O_01", next.name);
    assert_eq!(1, next.value);
    assert_eq!(PouchItemType::Food, next.r#type);
    let cook_data = next.cook_data.unwrap();
    assert_eq!(0, cook_data.sell_price);
    assert_eq!(0, cook_data.health_recover);
    assert_eq!(4, cook_data.effect_duration);
    assert_eq!(4.0, cook_data.effect.1);

    Ok(())
}
