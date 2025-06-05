use blueflame::env::DlcVer;
use blueflame::game::{PouchItem, PouchItemType, gdt, singleton_instance};
use blueflame::linker;
use blueflame::memory::{self, Memory, Ptr, proxy};
use blueflame::processor::{Cpu1, Cpu2, CrashReport, Process};
use blueflame::program;

#[derive(Debug, thiserror::Error)]
enum ErrorWrapper {
    #[error("{0:?}")]
    Crash(#[from] CrashReport),
}

#[test]
pub fn test_item_getters() -> anyhow::Result<()> {
    let data = std::fs::read("../runtime/program.bfi")?;
    let mut program_bytes = Vec::new();
    let program = program::unpack_zc(&data, &mut program_bytes)?;
    let pmdm_addr_for_test = 0x2222200000;
    let proc = linker::init_process(
        program,
        DlcVer::V300,
        0x8888800000,
        0x4000,
        pmdm_addr_for_test,
        20000000,
    )?;

    test_get_general(proc.clone())?;
    test_get_sword(proc.clone())?;
    test_get_bow(proc.clone())?;
    test_get_shield(proc.clone())?;
    test_get_material(proc.clone())?;
    test_get_food(proc.clone())?;
    test_get_armor(proc)?;

    Ok(())
}

fn test_get_general(mut proc: Process) -> anyhow::Result<()> {
    let pmdm_ptr = singleton_instance!(pmdm(proc.memory()))?;
    let mut cpu1 = Cpu1::default();
    let mut cpu2 = Cpu2::new(&mut cpu1, &mut proc);

    // value is durability for equipment, amount for other
    cpu2.with_crash_report(|cpu| {
        linker::get_item_with_value(cpu, "Armor_176_Head", 10000, None)?;

        assert_item_helper(
            Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
            cpu.proc.memory(),
            "Armor_176_Head",
            PouchItemType::ArmorHead,
            -1,
        )?;

        linker::call_load_from_game_data(cpu)?;

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
    })
    .map_err(ErrorWrapper::Crash)?;

    Ok(())
}

pub fn test_get_sword(mut proc: Process) -> anyhow::Result<()> {
    let pmdm_ptr = singleton_instance!(pmdm(proc.memory()))?;
    let mut cpu1 = Cpu1::default();
    let mut cpu2 = Cpu2::new(&mut cpu1, &mut proc);

    cpu2.with_crash_report(|cpu| {
        linker::get_item(cpu, "Weapon_Sword_009", None, None)?;

        assert_item_helper(
            Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
            cpu.proc.memory(),
            "Weapon_Sword_009",
            PouchItemType::Sword,
            2700,
        )?;

        Ok(())
    })
    .map_err(ErrorWrapper::Crash)?;

    Ok(())
}

fn test_get_bow(mut proc: Process) -> anyhow::Result<()> {
    let pmdm_ptr = singleton_instance!(pmdm(proc.memory()))?;
    let mut cpu1 = Cpu1::default();
    let mut cpu2 = Cpu2::new(&mut cpu1, &mut proc);

    cpu2.with_crash_report(|cpu| {
        linker::get_item_with_value(cpu, "Weapon_Bow_027", 50, None)?;
        assert_item_helper(
            Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
            cpu.proc.memory(),
            "Weapon_Bow_027",
            PouchItemType::Bow,
            50,
        )?;
        Ok(())
    })
    .map_err(ErrorWrapper::Crash)?;

    Ok(())
}

fn test_get_shield(mut proc: Process) -> anyhow::Result<()> {
    let pmdm_ptr = singleton_instance!(pmdm(proc.memory()))?;
    let mut cpu1 = Cpu1::default();
    let mut cpu2 = Cpu2::new(&mut cpu1, &mut proc);

    cpu2.with_crash_report(|cpu| {
        linker::get_item_with_value(cpu, "Weapon_Shield_005", 50, None)?;
        assert_item_helper(
            Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
            cpu.proc.memory(),
            "Weapon_Shield_005",
            PouchItemType::Shield,
            50,
        )?;
        Ok(())
    })
    .map_err(ErrorWrapper::Crash)?;

    Ok(())
}

fn test_get_material(mut proc: Process) -> anyhow::Result<()> {
    let pmdm_ptr = singleton_instance!(pmdm(proc.memory()))?;
    let mut cpu1 = Cpu1::default();
    let mut cpu2 = Cpu2::new(&mut cpu1, &mut proc);

    cpu2.with_crash_report(|cpu| {
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
    })
    .map_err(ErrorWrapper::Crash)?;

    Ok(())
}

fn test_get_food(mut proc: Process) -> anyhow::Result<()> {
    let pmdm_ptr = singleton_instance!(pmdm(proc.memory()))?;
    let mut cpu1 = Cpu1::default();
    let mut cpu2 = Cpu2::new(&mut cpu1, &mut proc);

    cpu2.with_crash_report(|cpu| {
        linker::get_item_with_value(cpu, "Item_Roast_07", 100, None)?;
        assert_item_helper(
            Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
            cpu.proc.memory(),
            "Item_Roast_07",
            PouchItemType::Food,
            100,
        )?;
        Ok(())
    })
    .map_err(ErrorWrapper::Crash)?;

    Ok(())
}

fn test_get_armor(mut proc: Process) -> anyhow::Result<()> {
    let pmdm_ptr = singleton_instance!(pmdm(proc.memory()))?;
    let mut cpu1 = Cpu1::default();
    let mut cpu2 = Cpu2::new(&mut cpu1, &mut proc);

    cpu2.with_crash_report(|cpu| {
        linker::get_item_with_value(cpu, "Armor_001_Head", 8, None)?;
        assert_item_helper(
            Ptr!(&pmdm_ptr->mLastAddedItem).load(cpu.proc.memory())?,
            cpu.proc.memory(),
            "Armor_001_Head",
            PouchItemType::ArmorHead,
            8,
        )?;
        Ok(())
    })
    .map_err(ErrorWrapper::Crash)?;

    Ok(())
}

fn assert_item_helper(
    item: Ptr![PouchItem],
    memory: &Memory,
    expected_name: &str,
    expected_type: PouchItemType,
    expected_value: i32,
) -> Result<(), memory::Error> {
    assert!(!item.is_nullptr());

    let name_ptr = Ptr!(&item->mName);
    assert_eq!(name_ptr.utf8_lossy(memory)?, expected_name);

    let item_type = Ptr!(&item->mType).load(memory)?;
    assert_eq!(item_type, expected_type as i32);

    let value = Ptr!(&item->mValue).load(memory)?;
    assert_eq!(value, expected_value);

    Ok(())
}
