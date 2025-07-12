use blueflame::game::{PouchItem, PouchItemType, gdt, singleton_instance};
use blueflame::linker;
use blueflame::memory::{Memory, Ptr, proxy};
use blueflame::processor::{Cpu2, Error};

pub fn get_item_basic(cpu: &mut Cpu2) -> Result<(), Error> {
    let pmdm_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    linker::get_item_with_value(cpu, "Armor_176_Head", 10000, None)?;

    assert_item_helper(
        Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
        cpu.proc.memory(),
        "Armor_176_Head",
        PouchItemType::ArmorHead,
        -1,
    )?;

    linker::load_from_game_data(cpu)?;

    linker::get_item_with_value(cpu, "Obj_KorokNuts", 100, None)?;
    linker::get_item_with_value(cpu, "Obj_KorokNuts", 100, None)?;
    linker::get_item_with_value(cpu, "Obj_KorokNuts", 100, None)?;
    linker::get_item_with_value(cpu, "Obj_KorokNuts", 100, None)?;
    linker::get_item_with_value(cpu, "Obj_KorokNuts", 100, None)?;

    assert_item_helper(
        Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
        cpu.proc.memory(),
        "Obj_KorokNuts",
        PouchItemType::KeyItem,
        500,
    )?;

    let gdt_ptr = gdt::trigger_param_ptr(cpu.proc.memory())?;
    let proc = &cpu.proc;
    proxy! { let params = *gdt_ptr as trigger_param in proc };
    let flag = params.by_name::<gdt::fd!(s32)>("KorokNutsNum").unwrap();
    assert_eq!(*flag.get(), 500);

    Ok(())
}

pub fn get_sword(cpu: &mut Cpu2) -> Result<(), Error> {
    let pmdm_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    linker::get_item(cpu, "Weapon_Sword_009", None, None)?;

    assert_item_helper(
        Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
        cpu.proc.memory(),
        "Weapon_Sword_009",
        PouchItemType::Sword,
        2700,
    )?;

    Ok(())
}

pub fn get_arrow(cpu: &mut Cpu2) -> Result<(), Error> {
    let pmdm_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    linker::get_item(cpu, "AncientArrow", None, None)?;
    assert_item_helper(
        Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
        cpu.proc.memory(),
        "AncientArrow",
        PouchItemType::Arrow,
        1,
    )?;

    linker::get_item(cpu, "FireArrow", None, None)?;
    assert_item_helper(
        Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
        cpu.proc.memory(),
        "FireArrow",
        PouchItemType::Arrow,
        1,
    )?;

    Ok(())
}

pub fn get_bow(cpu: &mut Cpu2) -> Result<(), Error> {
    let pmdm_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    linker::get_item_with_value(cpu, "Weapon_Bow_027", 50, None)?;

    assert_item_helper(
        Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
        cpu.proc.memory(),
        "Weapon_Bow_027",
        PouchItemType::Bow,
        50,
    )?;

    Ok(())
}

pub fn get_shield(cpu: &mut Cpu2) -> Result<(), Error> {
    let pmdm_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    linker::get_item_with_value(cpu, "Weapon_Shield_005", 50, None)?;

    assert_item_helper(
        Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
        cpu.proc.memory(),
        "Weapon_Shield_005",
        PouchItemType::Shield,
        50,
    )?;

    Ok(())
}

pub fn get_material(cpu: &mut Cpu2) -> Result<(), Error> {
    let pmdm_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    linker::get_item(cpu, "Item_Fruit_B", None, None)?;
    assert_item_helper(
        Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
        cpu.proc.memory(),
        "Item_Fruit_B",
        PouchItemType::Material,
        1,
    )?;
    linker::get_item(cpu, "Item_Fruit_B", None, None)?;
    assert_item_helper(
        Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
        cpu.proc.memory(),
        "Item_Fruit_B",
        PouchItemType::Material,
        2,
    )?;
    Ok(())
}

pub fn get_food(cpu: &mut Cpu2) -> Result<(), Error> {
    let pmdm_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    linker::get_item_with_value(cpu, "Item_Roast_07", 100, None)?;
    assert_item_helper(
        Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
        cpu.proc.memory(),
        "Item_Roast_07",
        PouchItemType::Food,
        100,
    )?;
    Ok(())
}

pub fn get_food_with_effect(cpu: &mut Cpu2) -> Result<(), Error> {
    let pmdm_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    linker::get_cook_item(
        cpu,
        "Item_Cook_C_17",
        &["Animal_Insect_A", "Animal_Insect_A"],
        Some(12.0),
        Some(300),
        Some(50),
        Some(13),
        Some(2.0),
    )?;
    assert_item_helper(
        Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
        cpu.proc.memory(),
        "Item_Cook_C_17",
        PouchItemType::Food,
        1,
    )?;
    Ok(())
}

pub fn get_armor(cpu: &mut Cpu2) -> Result<(), Error> {
    let pmdm_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    linker::get_item_with_value(cpu, "Armor_001_Head", 8, None)?;
    assert_item_helper(
        Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
        cpu.proc.memory(),
        "Armor_001_Head",
        PouchItemType::ArmorHead,
        8,
    )?;
    Ok(())
}

pub fn get_orb(cpu: &mut Cpu2) -> Result<(), Error> {
    let pmdm_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    linker::get_item(cpu, "Obj_DungeonClearSeal", Some(8), None)?;
    assert_item_helper(
        Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
        cpu.proc.memory(),
        "Obj_DungeonClearSeal",
        PouchItemType::KeyItem,
        8,
    )?;
    linker::get_item(cpu, "Obj_DungeonClearSeal", Some(8), None)?;
    assert_item_helper(
        Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
        cpu.proc.memory(),
        "Obj_DungeonClearSeal",
        PouchItemType::KeyItem,
        16,
    )?;
    linker::get_item(cpu, "Obj_DungeonClearSeal", Some(140), None)?;
    assert_item_helper(
        Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
        cpu.proc.memory(),
        "Obj_DungeonClearSeal",
        PouchItemType::KeyItem,
        124, // maxed out
    )?;
    Ok(())
}

fn assert_item_helper(
    item: Ptr![PouchItem],
    memory: &Memory,
    expected_name: &str,
    expected_type: PouchItemType,
    expected_value: i32,
) -> Result<(), blueflame::memory::Error> {
    assert!(!item.is_nullptr());

    let name_ptr = Ptr!(&item->mName);
    assert_eq!(name_ptr.utf8_lossy(memory)?, expected_name);

    let item_type = Ptr!(&item->mType).load(memory)?;
    assert_eq!(item_type, expected_type as i32);

    let value = Ptr!(&item->mValue).load(memory)?;
    assert_eq!(value, expected_value);

    Ok(())
}
