use teleparse::{Span, tp};

use crate::cir;
use crate::error::{cir_push_error, cir_push_warning, ErrorReport};
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
        self.effect_level.map(|x| f32::to_bits(x)).hash(state);
        self.ingredients.hash(state);
        self.star.hash(state);
    }
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
                Ok(cir::MetaValue::Int(x)) => {
                    self.value = Some(x as i32);
                }
                Ok(mv) => {
                    cir_push_error!(errors, value, InvalidMetaValue(key_str, mv));
                }
                Err(e) => {
                    errors.push(e);
                }
            },
            "durability" | "dura" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => {
                    self.value = Some((x * 100) as i32);
                }
                Ok(mv) => {
                    cir_push_error!(errors, value, InvalidMetaValue(key_str, mv));
                }
                Err(e) => {
                    errors.push(e);
                }
            },
            "equip" | "equipped" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Bool(x)) => {
                    self.equip = Some(x);
                }
                Ok(mv) => {
                    cir_push_error!(errors, value, InvalidMetaValue(key_str, mv));
                }
                Err(e) => {
                    errors.push(e);
                }
            },
            "life_recover" | "hp" | "modpower" => {
                match cir::parse_optional_meta_value(value.as_ref()) {
                    Ok(cir::MetaValue::Int(x)) => {
                        self.life_recover = Some(x as i32);
                    }
                    Ok(mv) => {
                    cir_push_error!(errors, value, InvalidMetaValue(key_str, mv));
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            }
            "time" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => {
                    self.effect_duration = Some(x as i32);
                }
                Ok(mv) => {
                    cir_push_error!(errors, value, InvalidMetaValue(key_str, mv));
                }
                Err(e) => {
                    errors.push(e);
                }
            },
            "price" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => {
                    self.sell_price = Some(x as i32);
                }
                Ok(mv) => {
                    cir_push_error!(errors, value, InvalidMetaValue(key_str, mv));
                }
                Err(e) => {
                    errors.push(e);
                }
            },
            "modifier" | "modtype" => {
                match cir::parse_optional_meta_value(value.as_ref()) {
                    Ok(cir::MetaValue::Int(x)) => {
                        // integer => same as price
                        self.sell_price = Some(x as i32);
                    }
                    Ok(cir::MetaValue::String(x)) => {
                        // string modifier, parse it and add it
                        match parse_weapon_modifier_bits(&x) {
                            Some(m) => {
                                self.sell_price = Some(self.sell_price.unwrap_or_default() | m)
                            }
                            None => {
                                cir_push_error!(errors, value, InvalidWeaponModifier(x));
                            }
                        }
                    }
                    Ok(mv) => {
                        cir_push_error!(errors, value, InvalidWeaponModifier(mv.to_string()));
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            }
            "effect" => {
                match cir::parse_optional_meta_value(value.as_ref()) {
                    Ok(cir::MetaValue::Int(x)) => {
                        // integer => set it without checking
                        self.effect_id = Some(x as i32);
                    }
                    Ok(cir::MetaValue::String(x)) => {
                        // string modifier, parse it
                        match parse_cook_effect(&x) {
                            Some(m) => self.effect_id = Some(m),
                            None => {
                                cir_push_error!(errors, value, InvalidCookEffect(x));
                            }
                        }
                    }
                    Ok(mv) => {
                        cir_push_error!(errors, value, InvalidCookEffect(mv.to_string()));
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            }
            "level" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => {
                    self.effect_level = Some(x as f32);
                }
                Ok(cir::MetaValue::Float(x)) => {
                    self.effect_level = Some(x as f32);
                }
                Ok(mv) => {
                    cir_push_error!(errors, value, InvalidMetaValue(key_str, mv));
                }
                Err(e) => {
                    errors.push(e);
                }
            },
            "ingr" => {
                if self.ingredients.len() >= 5 {
                    cir_push_error!(errors, value, TooManyIngredients);
                    return;
                }
                match cir::parse_optional_meta_value(value.as_ref()) {
                    Ok(cir::MetaValue::String(x)) => match search::search_item_by_ident(&x) {
                        Some(item) => {
                            self.ingredients.push(item.actor);
                        }
                        None => {
                            cir_push_error!(errors, value, InvalidItem(x));
                        }
                    },
                    Ok(mv) => {
                    cir_push_error!(errors, value, InvalidMetaValue(key_str, mv));
                    }
                    Err(e) => {
                        errors.push(e);
                    }
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
                Ok(mv) => {
                    cir_push_error!(errors, value, InvalidMetaValue(key_str, mv));
                }
                Err(e) => {
                    errors.push(e);
                }
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
