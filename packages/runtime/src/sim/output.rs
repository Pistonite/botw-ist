use crate::{iv, sim};
use crate::error::{RuntimeViewError, ErrorReport};

#[derive(Clone, Default)]
pub struct RunOutput {
    /// State at each simulation step
    pub states: Vec<sim::State>,
    pub errors: Vec<ErrorReport>,
}

impl RunOutput {
    /// Get the pouch inventory view for the given step in the script
    ///
    /// If there are no steps in the script, an empty pouch list is returned. Otherwise, 
    /// the state at the given step is used to generate the pouch list unless the step is out of
    /// bounds, in which case the last state is used.
    pub fn get_pouch_list(&self, step: usize) -> Result<iv::PouchList, RuntimeViewError> {
        let Some(state) = self.get_state_by_step(step) else {
            return Ok(Default::default());
        };
        let state = match &state.game {
            sim::Game::Uninit => return Ok(Default::default()),
            sim::Game::Running(state) => state,
            sim::Game::Crashed(_) => return Err(RuntimeViewError::Crash),
        };

        sim::view::extract_pouch_view(&state.process)
    }

    /// Get the GDT inventory view for the given step in the script
    ///
    /// Trailing items with empty names are not included
    pub fn get_gdt_inventory(&self, _step: usize) -> Result<iv::Gdt, RuntimeViewError> {
        // mock data
        //
        Ok(iv::Gdt {
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
        })
    }

    pub fn get_overworld_items(&self, _step: usize) -> Result<iv::Overworld, RuntimeViewError> {
        // mock data
        Ok(iv::Overworld {
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
        })
    }

    fn get_state_by_step(&self, step: usize) -> Option<&sim::State> {
        if self.states.is_empty() {
            return None;
        }
        self.states.get(step).or_else(|| self.states.last())
    }
}
