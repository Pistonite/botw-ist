#![allow(clippy::too_many_arguments)]
#![allow(non_snake_case)]

mod gdt;
pub use gdt::*;
mod info_data;
pub use info_data::*;
mod string;
pub use string::*;
mod pouch;
pub use pouch::*;
mod container;
pub use container::*;


#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i32)]
pub enum PouchCategory {
    Sword = 0x0,
    Bow = 0x1,
    Shield = 0x2,
    Armor = 0x3,
    Material = 0x4,
    Food = 0x5,
    KeyItem = 0x6,
    Invalid = -1,
}


// impl PauseMenuDataMgr {
//     pub fn get_last_item_added_name(&self, mem: &Memory) -> Result<String, Error> {
//         let last_pouch_item = self.mLastAddedItem.deref(mem)?;
//         Ok(last_pouch_item.mName.to_string())
//     }
//
//     pub fn get_newly_added_name(&self) -> String {
//         self.mNewlyAddedItem.get_name()
//     }
//
//     pub fn get_newly_added_item(&self) -> PouchItem {
//         self.mNewlyAddedItem
//     }
//
//     pub fn get_active_item_iter(&self) -> OffsetListIter<PouchItem> {
//         self.mItemLists.get_active_item_iter()
//     }
//
//     pub fn get_inactive_item_iter(&self) -> OffsetListIter<PouchItem> {
//         self.mItemLists.get_inactive_item_iter()
//     }
// }

// pub struct PorchWeaponData {
//     pub modifier: i32,
//     pub modifier_value: i32,
// }
//
// pub struct PorchCookData {
//     pub health_recover: i32,
//     pub effect_duration: i32,
//     pub sell_price: i32,
//     pub effect: (f32, f32),
//     pub ingredients: [String; 5],
// }
//
// pub struct PorchItem {
//     pub r#type: PouchItemType,
//     pub value: i32,
//     pub name: String,
//     pub cook_data: Option<PorchCookData>,
//     pub weapon_data: Option<PorchWeaponData>,
// }
// impl PorchItem {
//     #[allow(dead_code)]
//     fn default(name: String, typ: PouchItemType, value: i32) -> Self {
//         PorchItem {
//             name,
//             value,
//             r#type: typ,
//             cook_data: None,
//             weapon_data: None,
//         }
//     }
//     #[allow(dead_code)]
//     fn weapon(name: String, typ: PouchItemType, value: i32, data: PorchWeaponData) -> Self {
//         PorchItem {
//             name,
//             value,
//             r#type: typ,
//             cook_data: None,
//             weapon_data: Some(data),
//         }
//     }
//     #[allow(dead_code)]
//     fn food(name: String, typ: PouchItemType, value: i32, data: PorchCookData) -> Self {
//         PorchItem {
//             name,
//             value,
//             r#type: typ,
//             cook_data: Some(data),
//             weapon_data: None,
//         }
//     }
// }

// pub struct GameDataItemIter<'a,  'y, 'z> {
//     index: usize,
//     num_swords: usize,
//     num_bows: usize,
//     num_shields: usize,
//     num_food: usize,
//
//     core: &'a Core< 'y, 'z>,
//     pouch_item_lookup: PouchItemTypeLookup,
// }
// impl<'a,  'y, 'z> GameDataItemIter<'a,  'y, 'z> {
//     const MAX_LEN: usize = 420;
//
//     pub fn new(core: &'a Core< 'y, 'z>, pouch_item_lookup: PouchItemTypeLookup) -> Self {
//         GameDataItemIter {
//             index: 0,
//             num_swords: 0,
//             num_bows: 0,
//             num_shields: 0,
//             num_food: 0,
//
//             core,
//             pouch_item_lookup,
//         }
//     }
//
//     pub fn next_item(&mut self) -> Result<PorchItem, Error> {
//         let trigger_param_addr = self.core.mem.get_trigger_param_addr();
//         let trigger_param = self
//             .core
//             .proxies
//             .get_trigger_param(self.core.mem, trigger_param_addr)?;
//         let item_name = trigger_param
//             .get_string64_array_value("PorchItem", self.index)
//             .ok_or_else(|| {
//                 Error::Mem(crate::memory::Error::Unexpected(format!(
//                     "item at index {} does not exist",
//                     self.index
//                 )))
//             })?;
//         let typ = self.pouch_item_lookup.get_type(&item_name);
//         let value = trigger_param
//             .get_s32_array_value("PorchItem_Value1", self.index)
//             .ok_or_else(|| {
//                 Error::Mem(crate::memory::Error::Unexpected(format!(
//                     "item at index {} does not exist",
//                     self.index
//                 )))
//             })?;
//         match typ {
//             PouchItemType::Sword => {
//                 let modifier = trigger_param
//                     .get_s32_array_value("PorchSword_FlagSp", self.num_swords)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "sword at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let modifier_value = trigger_param
//                     .get_s32_array_value("PorchSword_ValueSp", self.num_swords)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "sword at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let weapon_data = PorchWeaponData {
//                     modifier,
//                     modifier_value,
//                 };
//                 self.num_swords += 1;
//                 self.index += 1;
//                 Ok(PorchItem::weapon(item_name, typ, value, weapon_data))
//             }
//             PouchItemType::Bow => {
//                 let modifier = trigger_param
//                     .get_s32_array_value("PorchBow_FlagSp", self.num_bows)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "bow at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let modifier_value = trigger_param
//                     .get_s32_array_value("PorchBow_ValueSp", self.num_bows)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "bow at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let weapon_data = PorchWeaponData {
//                     modifier,
//                     modifier_value,
//                 };
//                 self.num_bows += 1;
//                 self.index += 1;
//                 Ok(PorchItem::weapon(item_name, typ, value, weapon_data))
//             }
//             PouchItemType::Shield => {
//                 let modifier = trigger_param
//                     .get_s32_array_value("PorchShield_FlagSp", self.num_shields)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "shield at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let modifier_value = trigger_param
//                     .get_s32_array_value("PorchShield_ValueSp", self.num_shields)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "shield at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let weapon_data = PorchWeaponData {
//                     modifier,
//                     modifier_value,
//                 };
//                 self.num_shields += 1;
//                 self.index += 1;
//                 Ok(PorchItem::weapon(item_name, typ, value, weapon_data))
//             }
//             PouchItemType::Food => {
//                 let (health_recover, effect_duration) = trigger_param
//                     .get_vector2f_array_value("StaminaRecover", self.num_food)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "food at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let effect = trigger_param
//                     .get_vector2f_array_value("CookEffect0", self.num_food)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "food at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let (sell_price, _) = trigger_param
//                     .get_vector2f_array_value("CookEffect1", self.num_food)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "food at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let m1 = trigger_param
//                     .get_string64_array_value("CookMaterialName0", self.num_food)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "food at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let m2 = trigger_param
//                     .get_string64_array_value("CookMaterialName1", self.num_food)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "food at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let m3 = trigger_param
//                     .get_string64_array_value("CookMaterialName2", self.num_food)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "food at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let m4 = trigger_param
//                     .get_string64_array_value("CookMaterialName3", self.num_food)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "food at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let m5 = trigger_param
//                     .get_string64_array_value("CookMaterialName4", self.num_food)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "food at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 self.num_food += 1;
//                 self.index += 1;
//                 Ok(PorchItem::food(
//                     item_name,
//                     typ,
//                     value,
//                     PorchCookData {
//                         health_recover: health_recover as i32,
//                         effect_duration: effect_duration as i32,
//                         sell_price: sell_price as i32,
//                         effect,
//                         ingredients: [m1, m2, m3, m4, m5],
//                     },
//                 ))
//             }
//             PouchItemType::ArmorHead
//             | PouchItemType::ArmorUpper
//             | PouchItemType::ArmorLower
//             | PouchItemType::Arrow
//             | PouchItemType::Material
//             | PouchItemType::KeyItem
//             | PouchItemType::Invalid => {
//                 self.index += 1;
//                 Ok(PorchItem::default(item_name, typ, value))
//             }
//         }
//     }
//
//     pub fn has_next(&self) -> bool {
//         self.index < Self::MAX_LEN
//     }
// }
//
// pub struct PouchItemTypeLookup {
//     profiles: HashMap<String, String>,
//     tags: HashMap<String, Vec<String>>,
// }
// impl PouchItemTypeLookup {
//     pub fn new() -> Self {
//         // TODO: --cleanup: unwraps
//         let file = std::fs::File::open("res/item_data.json").unwrap();
//         let reader = std::io::BufReader::new(file);
//         let json: HashMap<String, serde_json::Value> = serde_json::from_reader(reader).unwrap();
//         let mut profiles = HashMap::new();
//         for (key, value) in json {
//             if let Some(profile) = value.get("profile").and_then(|c| c.as_str()) {
//                 profiles.insert(key, profile.to_string());
//             }
//         }
//
//         let file = std::fs::File::open("res/actor_tags.json").unwrap();
//         let reader = std::io::BufReader::new(file);
//         let tags: HashMap<String, Vec<String>> = serde_json::from_reader(reader).unwrap();
//
//         Self { profiles, tags }
//     }
//
//     pub fn get_type(&self, name: &String) -> PouchItemType {
//         let profile = self.profiles.get(name);
//         let tags = self.tags.get(name);
//         if let (Some(profile), Some(tags)) = (profile, tags) {
//             if tags.contains(&String::from("Arrow")) {
//                 return PouchItemType::Arrow;
//             }
//
//             if profile == "WeaponSmallSword"
//                 || profile == "WeaponLongSword"
//                 || profile == "WeaponSpear"
//             {
//                 return PouchItemType::Sword;
//             }
//             if profile == "WeaponBow" {
//                 return PouchItemType::Bow;
//             }
//             if profile == "WeaponShield" {
//                 return PouchItemType::Shield;
//             }
//             if profile == "ArmorHead" {
//                 return PouchItemType::ArmorHead;
//             }
//             if profile == "ArmorUpper" {
//                 return PouchItemType::ArmorUpper;
//             }
//             if profile == "ArmorLower" {
//                 return PouchItemType::ArmorLower;
//             }
//             if profile == "HorseReins" {
//                 return PouchItemType::KeyItem;
//             }
//
//             if tags.contains(&String::from("CookResult"))
//                 || tags.contains(&String::from("RoastItem"))
//             {
//                 return PouchItemType::Food;
//             }
//
//             if tags.contains(&String::from("Important")) {
//                 return PouchItemType::KeyItem;
//             }
//
//             PouchItemType::Material
//         } else {
//             PouchItemType::Invalid
//         }
//     }
// }
