// use std::collections::HashMap;
//
// use blueflame::processor::instruction_registry::ExecutableInstruction;
//
// #[cfg(test)]
// mod simulator_tests {
//     use std::time::Instant;
//
//     use blueflame::boot::init_memory;
//     use blueflame::memory::traits::MemWrite;
//     use blueflame::memory::traits::Ptr;
//     use blueflame::processor::Processor;
//     use blueflame::proxy::trigger_param::GdtTriggerParam;
//     use blueflame::structs::{
//         CookEffectId, GameDataItemIter, OffsetListIter, PauseMenuDataMgr, PouchItem,
//         PouchItemTypeLookup,
//     };
//     use blueflame::Core;
//
//     use crate::estimate_cache_memory;
//
//     struct TestItem {
//         item_name: String,
//         value: i32,
//         modifier_flags: u32,
//         modifier_value: i32,
//     }
//
//     impl TestItem {
//         pub fn new(item_name: &str, value: i32, modifier_flags: u32, modifier_value: i32) -> Self {
//             TestItem {
//                 item_name: String::from(item_name),
//                 value,
//                 modifier_flags,
//                 modifier_value,
//             }
//         }
//     }
//
//     fn get_items(core: &mut Core<'_, '_, '_>, item_list: &Vec<TestItem>) {
//         for ti in item_list {
//             core.pmdm_item_get(
//                 &ti.item_name,
//                 ti.value,
//                 ti.modifier_flags,
//                 ti.modifier_value,
//             )
//             .unwrap();
//         }
//
//         assert_inventory(core, item_list, item_list.len());
//     }
//
//     /// Asserts visible inventory matches order and content of item_list
//     // fn assert_items(core: &Core<'_, '_, '_>, item_list: &Vec<TestItem>, sz: usize) {
//     //     let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
//     //     let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
//
//     //     let mut iter = pmdm.get_active_item_iter();
//     //     assert_eq!(iter.count, TryInto::<i32>::try_into(sz).unwrap());
//     //     let mut idx = 0;
//     //     while iter.has_next() {
//     //         let ti = item_list.get(idx).unwrap();
//     //         let pi = iter.next(&core.mem).unwrap().deref(&core.mem).unwrap();
//     //         assert_eq!(ti.item_name, pi.get_name());
//     //         assert_eq!(ti.value, pi.mValue);
//
//     //         idx += 1;
//     //     }
//     // }
//
//     /// Used to assert hidden items as well
//     fn assert_inventory(core: &Core<'_, '_, '_>, item_list: &Vec<TestItem>, visible_sz: usize) {
//         let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
//         let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
//
//         let mut iter = pmdm.get_active_item_iter();
//         assert_eq!(iter.count, TryInto::<i32>::try_into(visible_sz).unwrap());
//         let mut idx = 0;
//         let total_items = item_list.len();
//         while idx < total_items {
//             let ti = item_list.get(idx).unwrap();
//             let pi = iter.next(&core.mem).unwrap().deref(&core.mem).unwrap();
//             assert_eq!(ti.item_name, pi.get_name());
//             assert_eq!(ti.value, pi.mValue);
//
//             idx += 1;
//         }
//     }
//
//     fn do_save(core: &mut Core<'_, '_, '_>, trigger_param_addr: u64) -> GdtTriggerParam {
//         let trigger_param = core
//             .proxies
//             .mut_trigger_param(&mut core.mem, trigger_param_addr)
//             .unwrap();
//         let save = trigger_param.clone();
//         save
//     }
//
//     fn do_reload(core: &mut Core<'_, '_, '_>, trigger_param_addr: u64, value: GdtTriggerParam) {
//         let trigger_param = core
//             .proxies
//             .mut_trigger_param(&mut core.mem, trigger_param_addr)
//             .unwrap();
//         *trigger_param = value;
//         core.load_from_game_data().unwrap();
//         core.create_player_equipment().unwrap();
//     }
//
//     fn sync_to_gamedata(core: &mut Core<'_, '_, '_>) {
//         core.save_to_game_data().unwrap();
//     }
//
//     fn break_slots(core: &mut Core<'_, '_, '_>, pmdm: &mut PauseMenuDataMgr, num_slots: i32) {
//         pmdm.mItemLists.list1.mCount -= num_slots;
//         let mut writer = core.mem.write(core.mem.get_pmdm_addr(), None).unwrap();
//         pmdm.write_to_mem(&mut writer).unwrap();
//     }
//
//     fn find_item_address(
//         core: &Core<'_, '_, '_>,
//         mut iter: OffsetListIter<PouchItem>,
//         name: &str,
//     ) -> u64 {
//         while iter.has_next() {
//             let item_ptr = iter.next(&core.mem).unwrap();
//             let address = item_ptr.get_addr();
//             let item = item_ptr.deref(&core.mem).unwrap();
//             if item.get_name() == name {
//                 return address;
//             }
//         }
//         panic!("Could not find item in inventory!");
//     }
//
//     #[test]
//     pub fn dup_orbs() {
//         let data = std::fs::read("./test_files/program.blfm").unwrap();
//         let program = blueflame_program::unpack_blueflame(&data).unwrap();
//         let (mut mem, mut prox) =
//             init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 20000000).unwrap();
//         let mut cpu = Processor::default();
//         let mut core = Core {
//             cpu: &mut cpu,
//             mem: &mut mem,
//             proxies: &mut prox,
//         };
//         core.setup().unwrap();
//
//         let trigger_param_addr = core.mem.get_trigger_param_addr();
//
//         // TEST: https://github.com/Pistonite/botw-ist/blob/main/legacy/src/__tests__/dup_orbs.in.txt
//
//         // START: Get 5 Diamond 1 Slate 1 Glider 4 SpiritOrb
//         let items = vec![
//             TestItem::new("Item_Ore_A", 5, 0, 0),
//             TestItem::new("Obj_DRStone_Get", 1, 0, 0),
//             TestItem::new("PlayerStole2", 1, 0, 0),
//             TestItem::new("Obj_DungeonClearSeal", 4, 0, 0),
//         ];
//         get_items(&mut core, &items);
//         // END: Get 5 Diamond 1 Slate 1 Glider 4 SpiritOrb
//
//         // START: Save
//         let save = do_save(&mut core, trigger_param_addr);
//         // END: Save
//
//         // START: Break 4 Slots
//         let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
//         let mut pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
//         break_slots(&mut core, pmdm.as_mut(), 4);
//         // END: Break 4 Slots
//
//         // START: Reload
//         do_reload(&mut core, trigger_param_addr, save);
//         let dup_one = vec![
//             TestItem::new("Item_Ore_A", 5, 0, 0),
//             TestItem::new("Item_Ore_A", 5, 0, 0),
//             TestItem::new("Obj_DRStone_Get", 1, 0, 0),
//             TestItem::new("PlayerStole2", 1, 0, 0),
//             TestItem::new("Obj_DungeonClearSeal", 4, 0, 0),
//             TestItem::new("Obj_DungeonClearSeal", 4, 0, 0),
//         ];
//         assert_inventory(&core, &dup_one, 2);
//         // END: Reload
//
//         // START: Save
//         let save = do_save(&mut core, trigger_param_addr);
//         // END: Save
//
//         // START:
//         do_reload(&mut core, trigger_param_addr, save);
//         let dup_two = vec![
//             TestItem::new("Item_Ore_A", 5, 0, 0),
//             TestItem::new("Obj_DRStone_Get", 1, 0, 0),
//             TestItem::new("PlayerStole2", 1, 0, 0),
//             TestItem::new("Obj_DungeonClearSeal", 4, 0, 0),
//             TestItem::new("Obj_DungeonClearSeal", 4, 0, 0),
//             TestItem::new("Obj_DungeonClearSeal", 4, 0, 0),
//         ];
//         assert_inventory(&core, &dup_two, 2);
//         // END: Reload
//     }
//
//     #[test]
//     pub fn wmc_4() {
//         let data = std::fs::read("./test_files/program.blfm").unwrap();
//         let program = blueflame_program::unpack_blueflame(&data).unwrap();
//         let (mut mem, mut prox) =
//             init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 20000000).unwrap();
//         let mut cpu = Processor::default();
//         let mut core = Core {
//             cpu: &mut cpu,
//             mem: &mut mem,
//             proxies: &mut prox,
//         };
//         core.setup().unwrap();
//
//         let trigger_param_addr = core.mem.get_trigger_param_addr();
//
//         // TEST: https://github.com/Pistonite/botw-ist/blob/main/legacy/src/__tests__/wmc_4.in.txt
//
//         //START: init 58 food 1 hasty elixir 1 baked apple
//         let now = Instant::now();
//
//         let mut item_list: Vec<TestItem> = vec![];
//
//         for _ in 0..58 {
//             item_list.push(TestItem::new("Item_Cook_O_01", 1, 0, 0));
//         }
//
//         for ti in &item_list {
//             core.pmdm_item_get(
//                 &ti.item_name,
//                 ti.value,
//                 ti.modifier_flags,
//                 ti.modifier_value,
//             )
//             .unwrap();
//         }
//
//         let name = "Item_Cook_C_17";
//         let life_recover = 1.0;
//         let effect_time = 2;
//         let sell_price = 10;
//         let effect_id = CookEffectId::MovingSpeed;
//         let vitality_boost = 3.0;
//         let is_crit = false;
//
//         core.cook_item_get(
//             name,
//             life_recover,
//             effect_time,
//             sell_price,
//             effect_id,
//             vitality_boost,
//             is_crit,
//         )
//         .unwrap();
//
//         let ti = TestItem::new("Item_Roast_03", 1, 0, 0);
//
//         core.pmdm_item_get(
//             &ti.item_name,
//             ti.value,
//             ti.modifier_flags,
//             ti.modifier_value,
//         )
//         .unwrap();
//
//         // println!("Adding wmc_4 items took: {0}ms", now.elapsed().as_millis());
//
//         let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
//         let mut pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
//
//         let mut iter = pmdm.get_active_item_iter();
//
//         assert_eq!(iter.count, 60);
//
//         let mut idx = 0;
//
//         while idx < 58 {
//             let ti = item_list.get(idx).unwrap();
//             let pi = iter.next(&core.mem).unwrap().deref(&core.mem).unwrap();
//             assert_eq!(ti.item_name, pi.get_name());
//             assert_eq!(ti.value, pi.mValue);
//
//             idx += 1;
//         }
//
//         let pi = iter.next(&core.mem).unwrap().deref(&core.mem).unwrap();
//         assert_eq!("Item_Cook_C_17", pi.get_name());
//         assert_eq!(1, pi.mValue);
//         assert_eq!(pi.mData.mEffect, vec![13.0, 3.0]);
//
//         let pi = iter.next(&core.mem).unwrap().deref(&core.mem).unwrap();
//         assert_eq!("Item_Roast_03", pi.get_name());
//         assert_eq!(1, pi.mValue);
//         //END: init 58 food 1 hasty elixir 1 baked apple
//
//         // START: Break 1 Slot
//         let t = pmdm.as_mut();
//         t.mItemLists.list1.mCount -= 1;
//         let mut writer = core.mem.write(core.mem.get_pmdm_addr(), None).unwrap();
//         t.write_to_mem(&mut writer).unwrap();
//         // END: Break 1 Slot
//
//         // START: Save
//         let trigger_param = core
//             .proxies
//             .get_trigger_param(&mut core.mem, trigger_param_addr)
//             .unwrap();
//         let save = trigger_param.clone();
//         // END: Save
//
//         // START: eat baked apple
//         core.remove_item("Item_Roast_03").unwrap();
//
//         let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
//         let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
//
//         let mut iter = pmdm.get_active_item_iter();
//
//         let mut idx = 0;
//
//         while iter.has_next() {
//             let ti = item_list.get(idx).unwrap();
//             let pi = iter.next(&core.mem).unwrap().deref(&core.mem).unwrap();
//             assert_eq!(ti.item_name, pi.get_name());
//             assert_eq!(ti.value, pi.mValue);
//
//             idx += 1;
//         }
//
//         let pi = iter.next(&core.mem).unwrap().deref(&core.mem).unwrap();
//         assert_eq!(name, pi.get_name());
//         assert_eq!(1, pi.mValue);
//         assert_eq!(pi.mData.mEffect, vec![13.0, 3.0]);
//         // END: eat baked apple
//
//         // START: Reload
//         let trigger_param = core
//             .proxies
//             .mut_trigger_param(&mut core.mem, trigger_param_addr)
//             .unwrap();
//         *trigger_param = save;
//         core.load_from_game_data().unwrap();
//
//         let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
//         let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
//
//         let mut iter = pmdm.get_active_item_iter();
//
//         let pi = iter.next(&core.mem).unwrap().deref(&core.mem).unwrap();
//         assert_eq!("Item_Cook_C_17", pi.get_name());
//         assert_eq!(1, pi.mValue);
//         assert_eq!(pi.mData.mEffect, vec![13.0, 3.0]);
//
//         let mut idx = 0;
//
//         while idx < 58 {
//             let ti = item_list.get(idx).unwrap();
//             let pi = iter.next(&core.mem).unwrap().deref(&core.mem).unwrap();
//             assert_eq!(ti.item_name, pi.get_name());
//             assert_eq!(ti.value, pi.mValue);
//
//             idx += 1;
//         }
//
//         let pi = iter.next(&core.mem).unwrap().deref(&core.mem).unwrap();
//         assert_eq!("Item_Cook_C_17", pi.get_name());
//         assert_eq!(1, pi.mValue);
//         assert_eq!(pi.mData.mEffect, vec![-1.0, 0.0]);
//
//         println!("Running wmc_4 took: {0}ms", now.elapsed().as_millis());
//         println!(
//             "Inst cache size: {0}",
//             estimate_cache_memory(core.cpu.inst_cache.clone())
//         );
//         // END: Reload
//
//         // START Sync gamedata
//         // core.save_to_game_data().unwrap();
//         // core.load_from_game_data().unwrap();
//
//         // let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
//         // let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
//
//         // let mut iter = pmdm.get_active_item_iter();
//
//         // while iter.has_next() {
//         //     let pi = iter.next(&core.mem).unwrap().deref(&core.mem).unwrap();
//         //     let mEffect = pi.mData.mEffect;
//         //     println!("{0}: {1} {2}", pi.to_string(), mEffect.x, mEffect.y);
//         // }
//
//         // let pi = iter.next(&core.mem).unwrap().deref(&core.mem).unwrap();
//         // let mEffect = pi.mData.mEffect;
//         // println!("LAST {0}: {1} {2}", pi.to_string(), mEffect.x, mEffect.y);
//         // assert!(false);
//         // END Sync gamedata
//     }
//
//     #[test]
//     pub fn apples_999() {
//         let data = std::fs::read("./test_files/program.blfm").unwrap();
//         let program = blueflame_program::unpack_blueflame(&data).unwrap();
//         let (mut mem, mut prox) =
//             init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 20000000).unwrap();
//         let mut cpu = Processor::default();
//         let mut core = Core {
//             cpu: &mut cpu,
//             mem: &mut mem,
//             proxies: &mut prox,
//         };
//         core.setup().unwrap();
//
//         let trigger_param_addr = core.mem.get_trigger_param_addr();
//
//         // TEST: https://github.com/Pistonite/botw-ist/blob/main/legacy/src/__tests__/apples_999.in.txt
//         // Initialize 1 Shield 1 Apple 1 Slate 1 Glider
//         let items = vec![
//             TestItem::new("Weapon_Shield_001", 1000, 0, 0),
//             TestItem::new("Item_Fruit_A", 1, 0, 0),
//             TestItem::new("Obj_DRStone_Get", 1, 0, 0),
//             TestItem::new("PlayerStole2", 1, 0, 0),
//         ];
//         get_items(&mut core, &items);
//
//         // Equip shield
//         let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
//         let pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
//         let iter = pmdm.get_active_item_iter();
//         let shield_address = find_item_address(&core, iter, "Weapon_Shield_001");
//         core.equip_weapon(shield_address).unwrap();
//
//         // Break 3 slots
//         let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
//         let mut pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
//         break_slots(&mut core, &mut pmdm, 3);
//
//         // Save
//         let save = do_save(&mut core, trigger_param_addr);
//
//         // Unequip shield
//         core.unequip(shield_address).unwrap();
//
//         // Drop apple
//         core.remove_item("Item_Fruit_A").unwrap();
//
//         // Reload
//         // assert based on GameData (should see 999 apples here - or maybe 1000)
//         do_reload(&mut core, trigger_param_addr, save);
//         let items2 = vec![
//             TestItem::new("Weapon_Shield_001", 1000, 0, 0),
//             TestItem::new("Weapon_Shield_001", 1000, 0, 0),
//             TestItem::new("Item_Fruit_A", 1, 0, 0),
//             TestItem::new("Obj_DRStone_Get", 1, 0, 0),
//             TestItem::new("PlayerStole2", 1, 0, 0),
//         ];
//         assert_inventory(&core, &items2, 2);
//
//         // Save
//         let save = do_save(&mut core, trigger_param_addr);
//
//         // Drop apple
//         core.remove_item("Item_Fruit_A").unwrap();
//
//         // Reload
//         do_reload(&mut core, trigger_param_addr, save);
//         let items3 = vec![
//             TestItem::new("Weapon_Shield_001", 1000, 0, 0),
//             TestItem::new("Weapon_Shield_001", 1000, 0, 0),
//             TestItem::new("Item_Fruit_A", 999, 0, 0),
//             TestItem::new("Obj_DRStone_Get", 1, 0, 0),
//             TestItem::new("PlayerStole2", 1, 0, 0),
//         ];
//         assert_inventory(&core, &items3, 2);
//
//         let pouch_item_lookup = PouchItemTypeLookup::new().unwrap();
//         let mut gd_iter = GameDataItemIter::new(&core, pouch_item_lookup);
//
//         // Verify gamedata value is uncapped for the apples
//         while gd_iter.has_next() {
//             let pi = gd_iter.next_item().unwrap();
//             if pi.name == "Item_Fruit_A" {
//                 assert_eq!(pi.value, 1000);
//             }
//         }
//     }
//
//     #[test]
//     pub fn mswmc_simple() {
//         let data = std::fs::read("./test_files/program.blfm").unwrap();
//         let program = blueflame_program::unpack_blueflame(&data).unwrap();
//         let (mut mem, mut prox) =
//             init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 20000000).unwrap();
//         let mut cpu = Processor::default();
//         let mut core = Core {
//             cpu: &mut cpu,
//             mem: &mut mem,
//             proxies: &mut prox,
//         };
//         core.setup().unwrap();
//
//         #[allow(unused_variables)]
//         let trigger_param_addr = core.mem.get_trigger_param_addr();
//         // TODO:
//
//         // TEST: https://github.com/Pistonite/botw-ist/blob/main/legacy/src/__tests__/MSWMC_simple.in.txt
//     }
//
//     #[test]
//     pub fn inventory_nuking_1() {
//         let data = std::fs::read("./test_files/program.blfm").unwrap();
//         let program = blueflame_program::unpack_blueflame(&data).unwrap();
//         let (mut mem, mut prox) =
//             init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 20000000).unwrap();
//         let mut cpu = Processor::default();
//         let mut core = Core {
//             cpu: &mut cpu,
//             mem: &mut mem,
//             proxies: &mut prox,
//         };
//         core.setup().unwrap();
//
//         let trigger_param_addr = core.mem.get_trigger_param_addr();
//
//         // TEST: https://github.com/Pistonite/botw-ist/blob/main/legacy/src/__tests__/inventoryNuking1.in.txt
//
//         // note below test uses swords rather than slates. Adding multiple
//         // copies of the shiekah slate is generally not possible as there
//         // are multiple sanity checks for copied key items. Test still
//         // works and matches previous simulator, but may differ from linked
//         // test above
//         // initialize 1 apple 2 hylianshroom 3 sword
//         let items = vec![
//             TestItem::new("Weapon_Sword_058", 2700, 0, 0),
//             TestItem::new("Weapon_Sword_058", 2700, 0, 0),
//             TestItem::new("Weapon_Sword_058", 2700, 0, 0),
//             TestItem::new("Item_Fruit_A", 1, 0, 0),
//             TestItem::new("Item_Mushroom_E", 2, 0, 0),
//         ];
//         get_items(&mut core, &items);
//
//         // break 5 slots
//         let pmdm_ptr: Ptr<PauseMenuDataMgr> = Ptr::new(core.mem.get_pmdm_addr());
//         let mut pmdm = Box::new(pmdm_ptr.deref(&core.mem).unwrap());
//         break_slots(&mut core, &mut pmdm, 5);
//
//         // sync gamedata
//         sync_to_gamedata(&mut core);
//
//         // save
//         let save = do_save(&mut core, trigger_param_addr);
//
//         // reload
//         do_reload(&mut core, trigger_param_addr, save);
//         assert_inventory(&mut core, &items, 0);
//     }
// }
//
// fn estimate_cache_memory(
//     cache: HashMap<u64, Vec<Option<Box<dyn ExecutableInstruction>>>>,
// ) -> usize {
//     let mut total = 0;
//     for (_block_addr, block) in cache {
//         total += std::mem::size_of::<Vec<Option<Box<dyn ExecutableInstruction>>>>();
//         total += block.capacity() * std::mem::size_of::<Option<Box<dyn ExecutableInstruction>>>();
//
//         for maybe_inst in block {
//             if maybe_inst.is_some() {
//                 total += std::mem::size_of::<Box<dyn ExecutableInstruction>>();
//                 total += 0x38;
//             }
//         }
//     }
//     total
// }
