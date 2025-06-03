use std::sync::Arc;

use crate::{iv, sim, ErrorReport};

#[derive(Clone, Default)]
pub struct RunOutput {
    /// State at each simulation step
    pub states: Vec<sim::State>,
    errors: Vec<ErrorReport>,
}

impl RunOutput {
    // TODO: error
    pub fn get_pouch_list(&self, step: usize) -> iv::PouchList {
        // mock data
        let mock_item1 = iv::PouchItem {
            common: iv::CommonItem {
                actor_name: "Weapon_Sword_070".to_string(),
                value: 4000,
                is_equipped: true,
            },
            item_type: 0,
            item_use: 0,
            is_in_inventory: true,
            is_no_icon: false,
            data: iv::ItemData {
                effect_value: 0,
                effect_duration: 0,
                sell_price: 0,
                effect_id: 0f32,
                effect_level: 0f32,
            },
            ingredients: [
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
            holding_count: 0,
            prompt_entangled: false,
            node_addr: 0.into(),
            node_valid: true,
            node_pos: 419,
            node_prev: 0.into(),
            node_next: 0.into(),
            allocated_idx: 0,
            unallocated_idx: -1,
            tab_idx: 0,
            tab_slot: 0,
            accessible: true,
            dpad_accessible: true,
        };

        let mock_item2 = iv::PouchItem {
            common: iv::CommonItem {
                actor_name: "NormalArrow".to_string(),
                value: 25,
                is_equipped: false,
            },
            item_type: 2,
            item_use: 8,
            is_in_inventory: true,
            is_no_icon: false,
            data: iv::ItemData {
                effect_value: 0,
                effect_duration: 0,
                sell_price: 0,
                effect_id: 0f32,
                effect_level: 0f32,
            },
            ingredients: [
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
            holding_count: 0,
            prompt_entangled: false,
            node_addr: 0.into(),
            node_valid: true,
            node_pos: 418,
            node_prev: 0.into(),
            node_next: 0.into(),
            allocated_idx: 1,
            unallocated_idx: -1,
            tab_idx: 1,
            tab_slot: 0,
            accessible: true,
            dpad_accessible: true,
        };

        let mock_item3 = iv::PouchItem {
            common: iv::CommonItem {
                actor_name: "Item_Fruit_A".to_string(),
                value: 999,
                is_equipped: false,
            },
            item_type: 7,
            item_use: 8,
            is_in_inventory: true,
            is_no_icon: false,
            data: iv::ItemData {
                effect_value: 0,
                effect_duration: 0,
                sell_price: 0,
                effect_id: 0f32,
                effect_level: 0f32,
            },
            ingredients: [
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
            holding_count: 0,
            prompt_entangled: false,
            node_addr: 0.into(),
            node_valid: true,
            node_pos: 418,
            node_prev: 0.into(),
            node_next: 0.into(),
            allocated_idx: 2,
            unallocated_idx: -1,
            tab_idx: 3,
            tab_slot: 0,
            accessible: true,
            dpad_accessible: true,
        };

        let mut items = vec![];
        items.push(mock_item1);
        if step >= 1 {
            items.push(mock_item2);
        }
        if step >= 2 {
            items.push(mock_item3);
        }

        let tabs = match step {
            0 => vec![iv::PouchTab {
                item_idx: 0,
                tab_type: 0, // sword
            }],
            1 => vec![
                iv::PouchTab {
                    item_idx: 0,
                    tab_type: 0, // sword
                },
                iv::PouchTab {
                    item_idx: 1,
                    tab_type: 1, //bow
                },
                iv::PouchTab {
                    item_idx: -1,
                    tab_type: 3, //shield
                },
            ],
            _ => vec![
                iv::PouchTab {
                    item_idx: 0,
                    tab_type: 0, // sword
                },
                iv::PouchTab {
                    item_idx: 1,
                    tab_type: 1, //bow
                },
                iv::PouchTab {
                    item_idx: -1,
                    tab_type: 3, //shield
                },
                iv::PouchTab {
                    item_idx: 2,
                    tab_type: 7, //material
                },
            ],
        };

        iv::PouchList {
            count: items.len() as i32 - 1,
            items,
            are_tabs_valid: true,
            num_tabs: tabs.len() as i32,
            tabs,
        }

        // let Some(state) = self.get_state_by_step(step) else {
        //     return Ok(vec![]);
        // };
    }

    /// Get the GDT inventory view for the given step in the script
    ///
    /// Trailing items with empty names are not included
    pub fn get_gdt_inventory(&self, step: usize) -> iv::Gdt {
        // mock data
        //
        return iv::Gdt {
            items: vec![
                iv::GdtItem {
                    common: iv::CommonItem {
                        actor_name: "Weapon_Sword_070".to_string(),
                        value: 4000,
                        is_equipped: true,
                    },
                    idx: 0,
                    data: iv::GdtItemData::Sword {
                        idx: 0,
                        info: iv::WeaponModifier { flag: 0, value: 0 },
                    },
                },
                iv::GdtItem {
                    common: iv::CommonItem {
                        actor_name: "Item_Fruit_A".to_string(),
                        value: 999,
                        is_equipped: false,
                    },
                    idx: 1,
                    data: iv::GdtItemData::None,
                },
            ],
            master_sword: iv::GdtMasterSword {
                is_true_form: true,
                add_power: 30,
                add_beam_power: 10,
                recover_time: 0f32,
            },
            info: iv::GdtInvInfo {
                num_weapon_slots: 8,
                num_bow_slots: 5,
                num_shield_slots: 4,

                sword_tab_discovered: true,
                bow_tab_discovered: true,
                shield_tab_discovered: true,
                armor_tab_discovered: false,
                material_tab_discovered: true,
                food_tab_discovered: false,
                key_item_tab_discovered: false,
            },
        };
    }

    pub fn get_overworld_items(&self, step: usize) -> iv::Overworld {
        // mock data
        iv::Overworld {
            items: vec![
                iv::OverworldItem::Equipped {
                    actor: "Weapon_Sword_070".to_string(),
                    value: 3000,
                    modifier: Default::default(),
                },
                iv::OverworldItem::Held {
                    actor: "Item_Fruit_A".to_string(),
                },
                iv::OverworldItem::GroundEquipment {
                    actor: "Weapon_Sword_018".to_string(),
                    value: 2600,
                    modifier: iv::WeaponModifier {
                        flag: 0x1,
                        value: 100,
                    },
                },
                iv::OverworldItem::GroundItem {
                    actor: "Item_Fruit_A".to_string(),
                },
            ],
        }
    }

    // fn get_state_by_step(&self, step: usize) -> Option<sim::State> {
    //     match self.states.get(step) {
    //         Some(state) => Some(Arc::clone(state)),
    //         None => Some(Arc::clone(self.states.last()?)),
    //     }
    // }
}
