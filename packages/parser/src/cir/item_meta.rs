use teleparse::{Span, tp};

use crate::cir;
use crate::error::{ErrorReport, cir_push_error, cir_push_warning};
use crate::search;
use crate::syn;

use super::MetaParser;

/// Item metadata used to select or specify item
#[derive(Debug, Clone, Default)]
pub struct ItemMeta {
    /// The value of the item
    ///
    /// settable by:
    /// - `life=100` -> 100
    /// - `value=100` -> 100
    /// - `durability=1` -> 100
    pub value: Option<i32>,

    /// If the item is equipped
    ///
    /// settable by `equip`, `equipped`
    pub equip: Option<bool>,

    /// Settable by key `life_recover, hp, modpower`
    pub life_recover: Option<i32>,
    /// Settable by `time`
    pub effect_duration: Option<i32>,
    /// Settable by `price` (set), `modifier` (add)
    pub sell_price: Option<i32>,
    /// Settable by `effect` name
    pub effect_id: Option<i32>,
    /// Settable by `level`
    pub effect_level: Option<f32>,

    /// Settable by `ingr`
    pub ingredients: Vec<String>,

    /// Number of upgrades on armor
    pub star: Option<i32>,

    /// For constrained item list, manually specify the position
    /// of the item to skip look up
    pub position: Option<ItemPosition>,
}

impl PartialEq for ItemMeta {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && self.equip == other.equip
            && self.life_recover == other.life_recover
            && self.effect_duration == other.effect_duration
            && self.sell_price == other.sell_price
            && self.effect_id == other.effect_id
            && self.effect_level.map(|x| x.to_bits()) == other.effect_level.map(|x| x.to_bits())
            && self.ingredients == other.ingredients
            && self.star == other.star
            && self.position == other.position
    }
}
impl Eq for ItemMeta {}

impl std::hash::Hash for ItemMeta {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.equip.hash(state);
        self.life_recover.hash(state);
        self.effect_duration.hash(state);
        self.sell_price.hash(state);
        self.effect_id.hash(state);
        self.effect_level.map(f32::to_bits).hash(state);
        self.ingredients.hash(state);
        self.star.hash(state);
        self.position.hash(state);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ItemPosition {
    /// Specify the slot number of the item when there are multiple slots. 1-indexed.
    /// 1 means the first slot in inventory order, 2 means the second, etc.
    ///
    /// Specified using the `from-slot` property
    FromSlot(u32),

    /// Specify the tab index (index of the tab in the tab array) and the slot number
    /// in the tab. Both are 0-indexed. Tab index is max 50 (exclusive) and slot is max 20
    /// (exclusive)
    TabIdxAndSlot(u32, u32),

    /// Specify the position of the item by the category, 1-indexed page number
    /// for that tab, and the 1-indexed row and column number
    TabCategoryAndSlot(cir::CategorySpec),
}

impl ItemMeta {
    pub fn parse_syn(meta: &syn::ItemMeta, errors: &mut Vec<ErrorReport>) -> Self {
        let mut parser = ItemMeta::default();
        parser.parse(meta, errors);
        parser
    }

    pub fn parse(&mut self, meta: &syn::ItemMeta, errors: &mut Vec<ErrorReport>) {
        cir::parse_meta(meta, self, errors);
    }

    pub fn life_recover_f32(&self) -> Option<f32> {
        self.life_recover.map(|x| x as f32)
    }
}

impl MetaParser for &mut ItemMeta {
    type Output = Self;

    fn visit_start(&mut self, _meta: &syn::ItemMeta, _errors: &mut Vec<ErrorReport>) {}

    fn visit_entry(
        &mut self,
        span: Span,
        key: &tp::String<syn::ItemMetaKey>,
        value: &tp::Option<syn::ItemMetaValue>,
        errors: &mut Vec<ErrorReport>,
    ) {
        let key_str = key.to_ascii_lowercase();
        match key_str.trim() {
            "life" | "value" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => self.value = Some(x as i32),
                Ok(mv) => cir_push_error!(errors, value, InvalidMetaValue(key_str, mv)),
                Err(e) => errors.push(e),
            },
            "durability" | "dura" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => self.value = Some((x * 100) as i32),
                Ok(mv) => cir_push_error!(errors, value, InvalidMetaValue(key_str, mv)),
                Err(e) => errors.push(e),
            },
            "equip" | "equipped" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Bool(x)) => self.equip = Some(x),
                Ok(mv) => cir_push_error!(errors, value, InvalidMetaValue(key_str, mv)),
                Err(e) => errors.push(e),
            },
            "life-recover" | "hp" | "modpower" => {
                match cir::parse_optional_meta_value(value.as_ref()) {
                    Ok(cir::MetaValue::Int(x)) => self.life_recover = Some(x as i32),
                    Ok(mv) => cir_push_error!(errors, value, InvalidMetaValue(key_str, mv)),
                    Err(e) => errors.push(e),
                }
            }
            "time" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => self.effect_duration = Some(x as i32),
                Ok(mv) => cir_push_error!(errors, value, InvalidMetaValue(key_str, mv)),
                Err(e) => errors.push(e),
            },
            "price" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => self.sell_price = Some(x as i32),
                Ok(mv) => cir_push_error!(errors, value, InvalidMetaValue(key_str, mv)),
                Err(e) => errors.push(e),
            },
            "modifier" | "modtype" => {
                match cir::parse_optional_meta_value(value.as_ref()) {
                    // integer => same as price
                    Ok(cir::MetaValue::Int(x)) => self.sell_price = Some(x as i32),
                    // string modifier, parse it and add it
                    Ok(cir::MetaValue::String(x)) => match parse_weapon_modifier_bits(&x) {
                        Some(m) => self.sell_price = Some(self.sell_price.unwrap_or_default() | m),
                        None => cir_push_error!(errors, value, InvalidWeaponModifier(x)),
                    },
                    Ok(mv) => cir_push_error!(errors, value, InvalidWeaponModifier(mv.to_string())),
                    Err(e) => errors.push(e),
                }
            }
            "effect" => match cir::parse_optional_meta_value(value.as_ref()) {
                // integer => set it without checking
                Ok(cir::MetaValue::Int(x)) => self.effect_id = Some(x as i32),
                // string modifier, parse it
                Ok(cir::MetaValue::String(x)) => match parse_cook_effect(&x) {
                    Some(m) => self.effect_id = Some(m),
                    None => cir_push_error!(errors, value, InvalidCookEffect(x)),
                },
                Ok(mv) => cir_push_error!(errors, value, InvalidCookEffect(mv.to_string())),
                Err(e) => errors.push(e),
            },
            "level" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => self.effect_level = Some(x as f32),
                Ok(cir::MetaValue::Float(x)) => self.effect_level = Some(x as f32),
                Ok(mv) => cir_push_error!(errors, value, InvalidMetaValue(key_str, mv)),
                Err(e) => errors.push(e),
            },
            "ingr" => {
                if self.ingredients.len() >= 5 {
                    cir_push_error!(errors, value, TooManyIngredients);
                    return;
                }
                match cir::parse_optional_meta_value(value.as_ref()) {
                    Ok(cir::MetaValue::String(x)) => match search::search_item_by_ident(&x) {
                        Some(item) => self.ingredients.push(item.actor),
                        None => cir_push_error!(errors, value, InvalidItem(x)),
                    },
                    Ok(mv) => cir_push_error!(errors, value, InvalidMetaValue(key_str, mv)),
                    Err(e) => errors.push(e),
                }
            }
            "star" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => {
                    if x < 0 || x > 4 {
                        cir_push_error!(errors, value, InvalidArmorStarNum(x as i32));
                        return;
                    }
                    self.star = Some(x as i32);
                }
                Ok(mv) => cir_push_error!(errors, value, InvalidMetaValue(key_str, mv)),
                Err(e) => errors.push(e),
            },
            "from-slot" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => {
                    self.position = Some(ItemPosition::FromSlot(x as u32));
                }
                Ok(mv) => cir_push_error!(errors, value, InvalidMetaValue(key_str, mv)),
                Err(e) => errors.push(e),
            },
            "tab" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => match self.position.take() {
                    None | Some(ItemPosition::FromSlot(_)) => {
                        self.position = Some(ItemPosition::TabIdxAndSlot(x as u32, 0))
                    }
                    Some(ItemPosition::TabIdxAndSlot(_, slot)) => {
                        self.position = Some(ItemPosition::TabIdxAndSlot(x as u32, slot))
                    }
                    Some(ItemPosition::TabCategoryAndSlot(mut cat)) => {
                        cat.amount = x;
                        self.position = Some(ItemPosition::TabCategoryAndSlot(cat));
                    }
                },
                Ok(mv) => cir_push_error!(errors, value, InvalidMetaValue(key_str, mv)),
                Err(e) => errors.push(e),
            },
            "slot" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => match self.position.take() {
                    None | Some(ItemPosition::FromSlot(_)) => {
                        self.position = Some(ItemPosition::FromSlot(x as u32))
                    }
                    Some(ItemPosition::TabIdxAndSlot(tab, _)) => {
                        if x < 0 || x >= 20 {
                            cir_push_error!(errors, value, InvalidSlot(x as i32));
                            return;
                        }
                        self.position = Some(ItemPosition::TabIdxAndSlot(tab, x as u32));
                    }
                    Some(ItemPosition::TabCategoryAndSlot(mut cat)) => {
                        if x < 0 || x >= 20 {
                            cir_push_error!(errors, value, InvalidSlot(x as i32));
                            return;
                        }
                        cat.row = (x / 5 + 1) as i8;
                        cat.col = (x % 5 + 1) as i8;
                        self.position = Some(ItemPosition::TabCategoryAndSlot(cat));
                    }
                },
                Ok(mv) => cir_push_error!(errors, value, InvalidMetaValue(key_str, mv)),
                Err(e) => errors.push(e),
            },
            "category" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::String(x)) => {
                    let Some(category) = cir::parse_category_from_str(&x) else {
                        cir_push_error!(errors, value, InvalidCategoryName(x));
                        return;
                    };
                    match self.position.take() {
                        None | Some(ItemPosition::FromSlot(_)) => {
                            self.position =
                                Some(ItemPosition::TabCategoryAndSlot(cir::CategorySpec {
                                    category,
                                    amount: 1,
                                    row: 1,
                                    col: 1,
                                }))
                        }
                        Some(ItemPosition::TabIdxAndSlot(tab, slot)) => {
                            let row = 4.min((slot / 5 + 1) as i8);
                            let col = (slot % 5 + 1) as i8;
                            self.position =
                                Some(ItemPosition::TabCategoryAndSlot(cir::CategorySpec {
                                    category,
                                    amount: tab as i64,
                                    row,
                                    col,
                                }))
                        }
                        Some(ItemPosition::TabCategoryAndSlot(mut cat)) => {
                            cat.category = category;
                            self.position = Some(ItemPosition::TabCategoryAndSlot(cat));
                        }
                    };
                }
                Ok(mv) => cir_push_error!(errors, value, InvalidMetaValue(key_str, mv)),
                Err(e) => errors.push(e),
            },
            // for simplicity we only allow row/col after category
            // is already specified, for now
            "row" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => match self.position.as_mut() {
                    Some(ItemPosition::TabCategoryAndSlot(cat)) => {
                        cat.row = x.clamp(1, 4) as i8;
                    }
                    _ => cir_push_warning!(errors, &span, UnusedMetaKey(key_str)),
                },
                Ok(mv) => cir_push_error!(errors, value, InvalidMetaValue(key_str, mv)),
                Err(e) => errors.push(e),
            },
            "col" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => match self.position.as_mut() {
                    Some(ItemPosition::TabCategoryAndSlot(cat)) => {
                        cat.col = x.clamp(1, 5) as i8;
                    }
                    _ => cir_push_warning!(errors, &span, UnusedMetaKey(key_str)),
                },
                Ok(mv) => cir_push_error!(errors, value, InvalidMetaValue(key_str, mv)),
                Err(e) => errors.push(e),
            },
            _ => {
                cir_push_warning!(errors, &span, UnusedMetaKey(key_str));
            }
        }
    }

    fn visit_end(&mut self, _meta: &syn::ItemMeta, _errors: &mut Vec<ErrorReport>) {}

    fn finish(self) -> Self::Output {
        self
    }
}

fn parse_weapon_modifier_bits(value: &str) -> Option<i32> {
    let value = value
        .replace("_", "")
        .replace("-", "")
        .replace(" ", "")
        .to_ascii_lowercase();
    match value.trim() {
        "attack" | "attackup" | "addpower" => Some(0x1),
        "addpowerplus" => Some(0x80000001u32 as i32),
        "durability" | "durabilityup" | "addlife" => Some(0x2),
        "addlifeplus" => Some(0x80000002u32 as i32),
        "critical" | "criticalhit" => Some(0x4),
        "longthrow" | "throw" => Some(0x8),
        "multishot" | "spreadfire" => Some(0x10),
        "zoom" => Some(0x20),
        "quickshot" | "rapidfire" => Some(0x40),
        "surfmaster" | "surf" | "shieldsurf" | "shieldsurfup" | "surfup" => Some(0x80),
        "guard" | "guardup" | "addguard" => Some(0x100),
        "addguardplus" => Some(0x80000100u32 as i32),
        "plus" | "yellow" => Some(0x80000000u32 as i32),
        _ => None,
    }
}

fn parse_cook_effect(value: &str) -> Option<i32> {
    let value = value
        .replace("_", "")
        .replace("-", "")
        .replace(" ", "")
        .to_ascii_lowercase();
    match value.trim() {
        "hearty" | "lifemaxup" => Some(2),
        "chilly" | "chill" | "resisthot" => Some(4),
        "spicy" | "resistcold" => Some(5),
        "electro" | "resistelectric" => Some(6),
        "mighty" | "attack" | "attackup" => Some(10),
        "tough" | "defense" | "defenseup" => Some(11),
        "sneaky" | "quiet" | "stealth" | "stealthup" | "quietness" => Some(12),
        "speed" | "speedup" | "allspeed" | "movingspeed" => Some(13),
        "energizing" | "stamina" | "staminaup" | "stam" | "stamup" | "gutsrecover" | "guts" => {
            Some(14)
        }
        "enduring" | "endura" | "endur" | "exgutsmaxup" | "exguts" => Some(15),
        "fire" | "fireproof" | "resistflame" | "resistfire" => Some(16),
        _ => None,
    }
}
