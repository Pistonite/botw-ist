#[cfg(test)]
mod singleton_tests {
    use std::u32;

    use blueflame::boot::init_memory;
    use blueflame::memory::traits::Ptr;
    use blueflame::processor::Processor;
    use blueflame::structs::{CookEffectId, ItemUse, PauseMenuDataMgr, PouchItemType};
    use blueflame::Core;

    use crate::{compute_hash, generate_crc32_table};

    #[test]
    pub fn test_item_get_general() {
        let data = std::fs::read("../test_files/program.blfm").unwrap();
        let program = blueflame_program::unpack_blueflame(&data).unwrap();

        let (mut mem, mut prox) =
            init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 2000000000).unwrap();
        let mut cpu = Processor::default();
        let mut core = Core {
            cpu: &mut cpu,
            mem: &mut mem,
            proxies: &mut prox,
        };
        core.setup().unwrap();

        // value is durability for equipment, amount for other

        core.pmdm_item_get("Armor_176_Head", 10000, 0x0, 0).unwrap();
        let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
        let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
        let new_item = pmdm.mLastAddedItem.deref(&core.mem).unwrap();
        assert_eq!(new_item.get_name(), "Armor_176_Head");
        assert_eq!(new_item.get_type(), PouchItemType::ArmorHead);

        core.load_from_game_data().unwrap();

        core.pmdm_item_get("Item_Roast_07", 100, 0, 0x0).unwrap();
        let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
        let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
        let new_item = pmdm.mLastAddedItem.deref(&core.mem).unwrap();
        assert_eq!(new_item.get_name(), "Item_Roast_07");
        assert_eq!(new_item.get_type(), PouchItemType::Food);

        // core.pmdm_item_get("Obj_KorokNuts", 100, 0, 0x0).unwrap();
        // core.pmdm_item_get("Obj_KorokNuts", 100, 0, 0x0).unwrap();
        // core.pmdm_item_get("Obj_KorokNuts", 100, 0, 0x0).unwrap();
        // core.pmdm_item_get("Obj_KorokNuts", 100, 0, 0x0).unwrap();
        // core.pmdm_item_get("Obj_KorokNuts", 100, 0, 0x0).unwrap();

        // core.pmdm_item_get("Item_Roast_07", 50, 0, 0x0).unwrap();

        // core.pmdm_item_get("Item_Fruit_B", 50, 0, 0x0).unwrap();

        // let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());

        // let mut iter = pmdm.get_active_item_iter();
        // while iter.has_next() {
        //     let pi = iter.next(&core.mem).unwrap().deref(&core.mem).unwrap();
        //     println!("{0}", pi.to_string());
        // }

        // assert!(false);

        core.pmdm_item_get("Obj_KorokNuts", 100, 0, 0x0).unwrap();
        core.pmdm_item_get("Obj_KorokNuts", 100, 0, 0x0).unwrap();
        core.pmdm_item_get("Obj_KorokNuts", 100, 0, 0x0).unwrap();
        core.pmdm_item_get("Obj_KorokNuts", 100, 0, 0x0).unwrap();
        core.pmdm_item_get("Obj_KorokNuts", 100, 0, 0x0).unwrap();
        let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
        let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
        let new_item = pmdm.mLastAddedItem.deref(&core.mem).unwrap();
        assert_eq!(new_item.get_name(), "Obj_KorokNuts");
        assert_eq!(new_item.get_type(), PouchItemType::KeyItem);
        let trigger_param = core
            .proxies
            .get_trigger_param(&core.mem, core.mem.get_trigger_param_addr())
            .unwrap();
        assert_eq!(
            *trigger_param
                .get_s32_flag_by_name(String::from("KorokNutsNum"))
                .unwrap()
                .get(),
            500
        );

        core.load_from_game_data().unwrap();
    }

    #[test]
    pub fn test_item_get_sword() {
        let data = std::fs::read("../test_files/program.blfm").unwrap();
        let program = blueflame_program::unpack_blueflame(&data).unwrap();
        let (mut mem, mut prox) =
            init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 2000000000).unwrap();
        let mut cpu = Processor::default();
        let mut core = Core {
            cpu: &mut cpu,
            mem: &mut mem,
            proxies: &mut prox,
        };
        core.setup().unwrap();

        core.pmdm_item_get("Weapon_Sword_009", 50, 0, 0x0).unwrap();
        let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
        let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
        let new_item = pmdm.mLastAddedItem.deref(&core.mem).unwrap();
        assert_eq!(new_item.get_name(), "Weapon_Sword_009");
        assert_eq!(new_item.get_type(), PouchItemType::Sword);
    }

    #[test]
    pub fn test_item_get_bow() {
        let data = std::fs::read("../test_files/program.blfm").unwrap();
        let program = blueflame_program::unpack_blueflame(&data).unwrap();
        let (mut mem, mut prox) =
            init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 2000000000).unwrap();
        let mut cpu = Processor::default();
        let mut core = Core {
            cpu: &mut cpu,
            mem: &mut mem,
            proxies: &mut prox,
        };
        core.setup().unwrap();

        core.pmdm_item_get("Weapon_Bow_027", 50, 0, 0x0).unwrap();
        let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
        let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
        let new_item = pmdm.mLastAddedItem.deref(&core.mem).unwrap();
        assert_eq!(new_item.get_name(), "Weapon_Bow_027");
        assert_eq!(new_item.get_type(), PouchItemType::Bow);
    }

    #[test]
    pub fn test_item_get_shield() {
        let data = std::fs::read("../test_files/program.blfm").unwrap();
        let program = blueflame_program::unpack_blueflame(&data).unwrap();
        let (mut mem, mut prox) =
            init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 2000000000).unwrap();
        let mut cpu = Processor::default();
        let mut core = Core {
            cpu: &mut cpu,
            mem: &mut mem,
            proxies: &mut prox,
        };
        core.setup().unwrap();

        core.pmdm_item_get("Weapon_Shield_005", 50, 0, 0x0).unwrap();
        let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
        let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
        let new_item = pmdm.mLastAddedItem.deref(&core.mem).unwrap();
        assert_eq!(new_item.get_name(), "Weapon_Shield_005");
        assert_eq!(new_item.get_type(), PouchItemType::Shield);
    }

    #[test]
    pub fn test_item_get_material() {
        let data = std::fs::read("../test_files/program.blfm").unwrap();
        let program = blueflame_program::unpack_blueflame(&data).unwrap();
        let (mut mem, mut prox) =
            init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 2000000000).unwrap();
        let mut cpu = Processor::default();
        let mut core = Core {
            cpu: &mut cpu,
            mem: &mut mem,
            proxies: &mut prox,
        };
        core.setup().unwrap();

        core.pmdm_item_get("Item_Fruit_B", 50, 0, 0x0).unwrap();
        let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
        let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
        let new_item = pmdm.mLastAddedItem.deref(&core.mem).unwrap();
        assert_eq!(new_item.get_name(), "Item_Fruit_B");
        assert_eq!(new_item.get_type(), PouchItemType::Material);
    }

    #[test]
    pub fn test_item_get_food() {
        let data = std::fs::read("../test_files/program.blfm").unwrap();
        let program = blueflame_program::unpack_blueflame(&data).unwrap();
        let (mut mem, mut prox) =
            init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 2000000000).unwrap();
        let mut cpu = Processor::default();
        let mut core = Core {
            cpu: &mut cpu,
            mem: &mut mem,
            proxies: &mut prox,
        };
        core.setup().unwrap();

        core.pmdm_item_get("Item_Roast_07", 100, 0, 0x0).unwrap();
        let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
        let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
        let new_item = pmdm.mLastAddedItem.deref(&core.mem).unwrap();
        assert_eq!(new_item.get_name(), "Item_Roast_07");
        assert_eq!(new_item.get_type(), PouchItemType::Food);
    }

    #[test]
    pub fn test_item_get_armor() {
        let data = std::fs::read("../test_files/program.blfm").unwrap();
        let program = blueflame_program::unpack_blueflame(&data).unwrap();
        let (mut mem, mut prox) =
            init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 2000000000).unwrap();
        let mut cpu = Processor::default();
        let mut core = Core {
            cpu: &mut cpu,
            mem: &mut mem,
            proxies: &mut prox,
        };
        core.setup().unwrap();

        core.pmdm_item_get("Armor_176_Head", 10000, 0x0, 0).unwrap();
        let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
        let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
        let new_item = pmdm.mLastAddedItem.deref(&core.mem).unwrap();
        assert_eq!(new_item.get_name(), "Armor_176_Head");
        assert_eq!(new_item.get_type(), PouchItemType::ArmorHead);
    }

    #[test]
    pub fn test_item_get_koroknuts() {
        let data = std::fs::read("../test_files/program.blfm").unwrap();
        let program = blueflame_program::unpack_blueflame(&data).unwrap();
        let (mut mem, mut prox) =
            init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 2000000000).unwrap();
        let mut cpu = Processor::default();
        let mut core = Core {
            cpu: &mut cpu,
            mem: &mut mem,
            proxies: &mut prox,
        };
        core.setup().unwrap();

        core.pmdm_item_get("Obj_KorokNuts", 100, 0, 0x0).unwrap();
        core.pmdm_item_get("Obj_KorokNuts", 100, 0, 0x0).unwrap();
        core.pmdm_item_get("Obj_KorokNuts", 100, 0, 0x0).unwrap();
        core.pmdm_item_get("Obj_KorokNuts", 100, 0, 0x0).unwrap();
        core.pmdm_item_get("Obj_KorokNuts", 100, 0, 0x0).unwrap();

        let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
        let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
        let new_item = pmdm.mLastAddedItem.deref(&core.mem).unwrap();
        assert_eq!(new_item.get_name(), "Obj_KorokNuts");
        assert_eq!(new_item.get_type(), PouchItemType::KeyItem);
        let trigger_param = core
            .proxies
            .get_trigger_param(&core.mem, core.mem.get_trigger_param_addr())
            .unwrap();
        assert_eq!(
            *trigger_param
                .get_s32_flag_by_name(String::from("KorokNutsNum"))
                .unwrap()
                .get(),
            500
        );
    }

    #[test]
    pub fn test_cook_item_get() {
        let data = std::fs::read("../test_files/program.blfm").unwrap();
        let program = blueflame_program::unpack_blueflame(&data).unwrap();
        let (mut mem, mut prox) =
            init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 2000000000).unwrap();
        let mut cpu = Processor::default();
        let mut core = Core {
            cpu: &mut cpu,
            mem: &mut mem,
            proxies: &mut prox,
        };
        core.setup().unwrap();

        let name = "Item_Cook_C_17";
        let life_recover = 1.0;
        let effect_time = 1;
        let sell_price = 1;
        let effect_id = CookEffectId::MovingSpeed;
        let vitality_boost = 1.0;
        let is_crit = false;

        core.cook_item_get(
            name,
            life_recover,
            effect_time,
            sell_price,
            effect_id,
            vitality_boost,
            is_crit,
        )
        .unwrap();

        let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
        let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());

        let mut iter = pmdm.get_active_item_iter();

        println!("{0}", iter.count);

        while iter.has_next() {
            let pi = iter.next(&core.mem).unwrap().deref(&core.mem).unwrap();
            assert_eq!(pi.get_name(), name);
            assert_eq!(pi.mValue, 1);
        }
    }

    struct TestItem {
        item_name: String,
        value: i32,
        modifier_flags: u32,
        modifier_value: i32,
        item_type: PouchItemType,
        item_use: ItemUse,
    }

    impl TestItem {
        pub fn new(
            item_name: &str,
            value: i32,
            modifier_flags: u32,
            modifier_value: i32,
            item_type: PouchItemType,
            item_use: ItemUse,
        ) -> Self {
            TestItem {
                item_name: String::from(item_name),
                value,
                modifier_flags,
                modifier_value,
                item_type,
                item_use,
            }
        }
    }

    #[test]
    pub fn test_item_get_iter() {
        let data = std::fs::read("../test_files/program.blfm").unwrap();
        let program = blueflame_program::unpack_blueflame(&data).unwrap();
        let (mut mem, mut prox) =
            init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 2000000000).unwrap();
        let mut cpu = Processor::default();
        let mut core = Core {
            cpu: &mut cpu,
            mem: &mut mem,
            proxies: &mut prox,
        };
        core.setup().unwrap();

        let mut item_list: Vec<TestItem> = vec![];

        item_list.push(TestItem::new(
            "Weapon_Sword_009",
            50,
            0,
            0,
            PouchItemType::Sword,
            ItemUse::WeaponSmallSword,
        ));
        item_list.push(TestItem::new(
            "Weapon_Bow_027",
            500,
            0,
            0,
            PouchItemType::Bow,
            ItemUse::WeaponBow,
        ));
        item_list.push(TestItem::new(
            "Weapon_Shield_005",
            50,
            0,
            0,
            PouchItemType::Shield,
            ItemUse::WeaponShield,
        ));
        item_list.push(TestItem::new(
            "Item_Fruit_B",
            500,
            0,
            0,
            PouchItemType::Material,
            ItemUse::CureItem,
        ));

        for ti in &item_list {
            core.pmdm_item_get(
                &ti.item_name,
                ti.value,
                ti.modifier_flags,
                ti.modifier_value,
            )
            .unwrap();
        }

        let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
        let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());

        let mut iter = pmdm.get_active_item_iter();
        assert_eq!(
            iter.count,
            TryInto::<i32>::try_into(item_list.len()).unwrap()
        );
        let mut idx = 0;
        while iter.has_next() {
            let ti = item_list.get(idx).unwrap();
            let pi = iter.next(&core.mem).unwrap().deref(&core.mem).unwrap();
            assert_eq!(ti.item_name, pi.get_name());
            assert_eq!(ti.value, pi.mValue);
            assert_eq!(ti.item_type, pi.get_type());
            assert_eq!(ti.item_use, pi.get_use());

            idx += 1;
        }

        core.load_from_game_data().unwrap();

        let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
        let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());

        let mut iter = pmdm.get_active_item_iter();
        assert_eq!(
            iter.count,
            TryInto::<i32>::try_into(item_list.len()).unwrap()
        );
        let mut idx = 0;
        while iter.has_next() {
            let ti = item_list.get(idx).unwrap();
            let pi = iter.next(&core.mem).unwrap().deref(&core.mem).unwrap();
            assert_eq!(ti.item_name, pi.get_name());
            assert_eq!(ti.value, pi.mValue);
            assert_eq!(ti.item_type, pi.get_type());
            assert_eq!(ti.item_use, pi.get_use());

            idx += 1;
        }
    }

    #[test]
    pub fn test_init_common_flags() {
        let data = std::fs::read("../test_files/program.blfm").unwrap();
        let program = blueflame_program::unpack_blueflame(&data).unwrap();

        let (mut mem, mut prox) =
            init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 2000000000).unwrap();
        let mut cpu = Processor::default();
        let mut core = Core {
            cpu: &mut cpu,
            mem: &mut mem,
            proxies: &mut prox,
        };

        core.init_common_flags().unwrap();
    }

    #[test]
    pub fn test_get_actor_type() {
        let data = std::fs::read("../test_files/program.blfm").unwrap();
        let program = blueflame_program::unpack_blueflame(&data).unwrap();

        let (mut mem, mut prox) =
            init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 2000000000).unwrap();
        let mut cpu = Processor::default();
        let mut core = Core {
            cpu: &mut cpu,
            mem: &mut mem,
            proxies: &mut prox,
        };
        core.setup().unwrap();

        let item_type = core.info_data_get_type("Armor_008_Upper").unwrap();
        assert_eq!(item_type, PouchItemType::ArmorUpper);
        let item_type = core.info_data_get_type("Item_Fruit_B").unwrap();
        assert_eq!(item_type, PouchItemType::Material);
        let item_type = core.info_data_get_type("Item_Roast_07").unwrap();
        assert_eq!(item_type, PouchItemType::Food);
    }

    #[test]
    pub fn test_hash_gen() {
        let data = std::fs::read("../test_files/program.blfm").unwrap();
        let program = blueflame_program::unpack_blueflame(&data).unwrap();

        let (mut mem, mut prox) =
            init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 2000000000).unwrap();
        let mut cpu = Processor::default();
        let mut core = Core {
            cpu: &mut cpu,
            mem: &mut mem,
            proxies: &mut prox,
        };
        core.setup().unwrap();
        let hash = core.get_hash_for_actor("Armor_008_Upper").unwrap();

        let table = generate_crc32_table();
        let mem_table: [u32; 0x100] = Ptr::new(39835392).deref(&mem).unwrap();
        assert_eq!(table, mem_table);
        assert_eq!(
            hash,
            compute_hash(
                &mut crate::HashContext { hash: 0xFFFFFFFF },
                "Armor_008_Upper".as_bytes()
            )
        );
    }
}

fn generate_crc32_table() -> [u32; 0x100] {
    let mut table = [0u32; 0x100];
    for i in 0..0x100 {
        let mut val = i as u32;
        for _ in 0..8 {
            val = if (val & 1) == 0 {
                val >> 1
            } else {
                (val >> 1) ^ 0xEDB88320
            };
        }
        table[i] = val;
    }
    table
}

pub struct HashContext {
    pub hash: u32,
}

pub fn compute_hash(context: &mut HashContext, data: &[u8]) -> u32 {
    let mut hash = context.hash;

    let s_table = generate_crc32_table();

    for &byte in data {
        let xor_val = (byte as u32) ^ hash;
        let index = (xor_val & 0xFF) as usize;
        hash = s_table[index] ^ (hash >> 8);
    }

    context.hash = hash;
    !hash // Return the final hash (bitwise NOT)
}
