use teleparse::{Span, tp};

use crate::cir;
use crate::error::{ErrorReport, cir_error, cir_warning};
use crate::syn;

use super::{MetaParser, enum_name};

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

    /// If the item is currently being held
    pub held: Option<bool>,
    // If new meta properties are added for matching,
    // they need to be updated in screen.rs!
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
            && self.held == other.held
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
        self.held.hash(state);
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
    pub fn parse_syn(meta: &syn::Meta, errors: &mut Vec<ErrorReport>) -> Self {
        let mut parser = ItemMeta::default();
        parser.parse(meta, errors);
        parser
    }

    pub fn parse(&mut self, meta: &syn::Meta, errors: &mut Vec<ErrorReport>) {
        cir::parse_meta(meta, self, errors);
    }

    pub fn life_recover_f32(&self) -> Option<f32> {
        self.life_recover.map(|x| x as f32)
    }

    pub fn effect_id_f32(&self) -> Option<f32> {
        self.effect_id.map(|x| x as f32)
    }

    fn check_add_more_ingr(&self, span: Span, errors: &mut Vec<ErrorReport>) -> bool {
        let ret = self.ingredients.len() < 5;
        if !ret {
            errors.push(cir_error!(span, TooManyIngredients));
        }
        ret
    }

    fn check_slot_idx(&self, slot: i32, span: Span, errors: &mut Vec<ErrorReport>) -> bool {
        let ret = slot >= 0 && slot < 20;
        if !ret {
            errors.push(cir_error!(span, InvalidSlot(slot)));
        }
        ret
    }

    pub fn to_script(&self, out: &mut String) {
        use std::fmt::Write as _;
        let mut has_value = false;
        macro_rules! add {
            ($key:literal, $value:expr) => {
                if !has_value {
                    has_value = true;
                    out.push('[');
                }
                write!(out, "{}={},", $key, $value).unwrap();
            };
        }
        macro_rules! add_opt {
            ($key:literal, $value:expr) => {
                if let Some(x) = $value {
                    add!($key, x);
                }
            };
        }
        add_opt!("value", &self.value);
        add_opt!("equip", &self.equip);
        add_opt!("hp", &self.life_recover);
        add_opt!("time", &self.effect_duration);
        add_opt!("effect", &self.effect_id);
        add_opt!("level", &self.effect_level);
        if !self.ingredients.is_empty() {
            for ingr in &self.ingredients {
                add!("ingr", ingr);
            }
        }
        add_opt!("star", &self.star);
        match &self.position {
            Some(ItemPosition::FromSlot(x)) => {
                add!("slot", x);
            }
            Some(ItemPosition::TabIdxAndSlot(tab, slot)) => {
                add!("tab", tab);
                add!("slot", slot);
            }
            Some(ItemPosition::TabCategoryAndSlot(spec)) => {
                add!("category", spec.category);
                add!("tab", spec.amount);
                add!("row", spec.row);
                add!("col", spec.col);
            }
            None => {}
        }
        add_opt!("held", &self.held);
        if has_value {
            out.pop(); // remove last comma
            out.push(']');
        }
    }
}

impl MetaParser for &mut ItemMeta {
    type Output = Self;

    fn visit_entry(
        &mut self,
        key: &tp::String<syn::MetaKey>,
        value: Option<&syn::MetaValue>,
        v_span: Span,
        errors: &mut Vec<ErrorReport>,
    ) {
        super::cir_match_meta_key_value! { (key, key_str, value, v_span, errors):
            "life" | "value" => required {
                int(x) => self.value = Some(x as i32),
            },
            "durability" | "dura" => required {
                int(x) => self.value = Some((x * 100) as i32),
            },
            "equip" | "equipped" => optional {
                bool(x) => self.equip = Some(x),
            },
            "life-recover" | "hp" | "modpower" => required {
                int(x) => self.life_recover = Some(x as i32),
            },
            "time" => required {
                int(x) => self.effect_duration = Some(x as i32),
            },
            "price" => required {
                int(x) => self.sell_price = Some(x as i32),
            },
            "modifier" | "modtype" => required {
                // integer => same as price
                int(x) => self.sell_price = Some(x as i32),
                // string modifier, parse it and add it
                string(x) => match enum_name::parse_weapon_modifier_bits(&x) {
                    Some(m) => self.sell_price = Some(self.sell_price.unwrap_or_default() | m),
                    None => errors.push(cir_error!(v_span, InvalidWeaponModifier(x))),
                },
            },
            "effect" => required {
                // number => set it without checking
                int(x) => self.effect_id = Some(x as i32),
                float(x) => self.effect_id = Some(x as i32),
                // string modifier, parse it
                string(x) => match enum_name::parse_cook_effect(&x) {
                    Some(m) => self.effect_id = Some(m),
                    None => errors.push(cir_error!(v_span, InvalidCookEffect(x))),
                },
            },
            "level" => required {
                int(x) => self.effect_level = Some(x as f32),
                float(x) => self.effect_level = Some(x as f32),
            },
            "ingr" => required {
                words(x) => {
                    if !self.check_add_more_ingr(v_span, errors) {
                        return;
                    }
                    match cir::search_item_by_ident(&x) {
                        Some(item) => self.ingredients.push(item.actor),
                        None => errors.push(cir_error!(v_span, InvalidItem(x))),
                    }
                },
                angled(x) => {
                    if !self.check_add_more_ingr(v_span, errors) {
                        return;
                    }
                    self.ingredients.push(x)
                },
            },
            "star" => required {
                int(x) => {
                    if x < 0 || x > 4 {
                        errors.push(cir_error!(v_span, InvalidArmorStarNum(x as i32)));
                        return;
                    }
                    self.star = Some(x as i32);
                }
            },
            "held" | "hold" | "holding" => optional {
                bool(x) => self.held = Some(x)
            },
            "from-slot" => required {
                int(x) => self.position = Some(ItemPosition::FromSlot(x as u32)),
            },
            "tab" => required {
                int(x) => match self.position.take() {
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
            },
            "slot" => required {
                int(x) => match self.position.take() {
                    None | Some(ItemPosition::FromSlot(_)) => {
                        self.position = Some(ItemPosition::FromSlot(x as u32))
                    }
                    Some(ItemPosition::TabIdxAndSlot(tab, _)) => {
                        if !self.check_slot_idx(x as i32, v_span, errors) {
                            return;
                        }
                        self.position = Some(ItemPosition::TabIdxAndSlot(tab, x as u32));
                    }
                    Some(ItemPosition::TabCategoryAndSlot(mut cat)) => {
                        if !self.check_slot_idx(x as i32, v_span, errors) {
                            return;
                        }
                        cat.row = (x / 5 + 1) as i8;
                        cat.col = (x % 5 + 1) as i8;
                        self.position = Some(ItemPosition::TabCategoryAndSlot(cat));
                    }
                },
            },
            "category" => required {
                words(x) => {
                    let Some(category) = cir::parse_category_from_str(&x) else {
                        errors.push(cir_error!(v_span, InvalidCategoryName(x)));
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
            },
            // for simplicity we only allow row/col after category
            // is already specified, for now
            "row" => required {
                int(x) => match self.position.as_mut() {
                    Some(ItemPosition::TabCategoryAndSlot(cat)) => {
                        cat.row = x.clamp(1, 4) as i8;
                    }
                    _ => errors.push(cir_warning!(key, UnusedMetaKey(key_str))),
                },
            },
            "col" => required {
                int(x) => match self.position.as_mut() {
                    Some(ItemPosition::TabCategoryAndSlot(cat)) => {
                        cat.col = x.clamp(1, 5) as i8;
                    }
                    _ => errors.push(cir_warning!(key, UnusedMetaKey(key_str))),
                },
            },
        }
    }

    fn visit_end(self, _meta: &syn::Meta, _errors: &mut Vec<ErrorReport>) -> Self {
        self
    }
}
