use teleparse::{tp, Span};

use crate::error::{ErrorReport, Error};
use crate::item_search::ItemResolver;
use crate::syn;
use crate::cir;

use super::MetaParser;

/// Item metadata used to select or specify item
#[derive(Debug, Default, PartialEq)]
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

impl ItemMeta {
    pub async fn parse<R: ItemResolver>(meta: &syn::ItemMeta, resolver: &R, errors: &mut Vec<ErrorReport>) -> ItemMeta {
        let parser = Parser {
            meta: ItemMeta::default(),
            resolver,
        };
        cir::parse_meta(meta, parser, errors).await
    }
}

struct Parser<'r, R: ItemResolver> {
    meta: ItemMeta,
    resolver: &'r R,
}

impl<R: ItemResolver> MetaParser for Parser<'_, R> {
    type Output = ItemMeta;

    async fn visit_start(&mut self, _meta: &syn::ItemMeta, _errors: &mut Vec<ErrorReport>) {
    }

    async fn visit_entry(&mut self, span: Span, key: &tp::String<syn::Word>, value: &tp::Option<syn::ItemMetaValue>, errors: &mut Vec<ErrorReport>) {
        let key_str = key.to_ascii_lowercase();
        match key_str.trim() {
            "life" | "value" => {
                match cir::MetaValue::parse_option(value.as_ref()) {
                    Ok(cir::MetaValue::Int(x)) => {
                        self.meta.value = Some(x as i32);
                    }
                    Ok(mv) => {
                        errors.push(
                        Error::InvalidMetaValue(key_str, mv)
                            .spanned(value)
                        );
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            },
            "durability" | "dura" => {
                match cir::MetaValue::parse_option(value.as_ref()) {
                    Ok(cir::MetaValue::Int(x)) => {
                        self.meta.value = Some((x * 100) as i32);
                    }
                    Ok(mv) => {
                        errors.push(
                        Error::InvalidMetaValue(key_str, mv)
                            .spanned(value)
                        );
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            },
            "equip" | "equipped"=> {
                match cir::MetaValue::parse_option(value.as_ref()) {
                    Ok(cir::MetaValue::Bool(x)) => {
                        self.meta.equip = Some(x);
                    }
                    Ok(mv) => {
                        errors.push(
                        Error::InvalidMetaValue(key_str, mv)
                            .spanned(value)
                        );
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            },
            "life_recover" | "hp" | "modpower" => {
                match cir::MetaValue::parse_option(value.as_ref()) {
                    Ok(cir::MetaValue::Int(x)) => {
                        self.meta.life_recover = Some(x as i32);
                    }
                    Ok(mv) => {
                        errors.push(
                        Error::InvalidMetaValue(key_str, mv)
                            .spanned(value)
                        );
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            },
            "time" => {
                match cir::MetaValue::parse_option(value.as_ref()) {
                    Ok(cir::MetaValue::Int(x)) => {
                        self.meta.effect_duration = Some(x as i32);
                    }
                    Ok(mv) => {
                        errors.push(
                        Error::InvalidMetaValue(key_str, mv)
                            .spanned(value)
                        );
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            },
            "price" => {
                match cir::MetaValue::parse_option(value.as_ref()) {
                    Ok(cir::MetaValue::Int(x)) => {
                        self.meta.sell_price = Some(x as i32);
                    }
                    Ok(mv) => {
                        errors.push(
                        Error::InvalidMetaValue(key_str, mv)
                            .spanned(value)
                        );
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            },
            "modifier" | "modtype" => {
                match cir::MetaValue::parse_option(value.as_ref()) {
                    Ok(cir::MetaValue::Int(x)) => {
                        // integer => same as price
                        self.meta.sell_price = Some(x as i32);
                    }
                    Ok(cir::MetaValue::String(x)) => {
                        // string modifier, parse it and add it
                        match parse_weapon_modifier_bits(&x) {
                            Some(m) => self.meta.sell_price = Some(self.meta.sell_price.unwrap_or_default() | m),
                            None => {
                                errors.push(
                                Error::InvalidWeaponModifier(x)
                                    .spanned(value)
                                );
                            }
                        }
                    }
                    Ok(mv) => {
                        errors.push(
                        Error::InvalidWeaponModifier(mv.to_string())
                            .spanned(value)
                        );
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            },
            "effect" => {
                match cir::MetaValue::parse_option(value.as_ref()) {
                    Ok(cir::MetaValue::Int(x)) => {
                        // integer => set it without checking
                        self.meta.effect_id = Some(x as i32);
                    }
                    Ok(cir::MetaValue::String(x)) => {
                        // string modifier, parse it 
                        match parse_cook_effect(&x) {
                            Some(m) => self.meta.effect_id = Some(m),
                            None => {
                                errors.push(
                                Error::InvalidCookEffect(x)
                                    .spanned(value)
                                );
                            }
                        }
                    }
                    Ok(mv) => {
                        errors.push(
                        Error::InvalidWeaponModifier(mv.to_string())
                            .spanned(value)
                        );
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            },
            "level" => {
                match cir::MetaValue::parse_option(value.as_ref()) {
                    Ok(cir::MetaValue::Int(x)) => {
                        self.meta.effect_level = Some(x as f32);
                    }
                    Ok(cir::MetaValue::Float(x)) => {
                        self.meta.effect_level = Some(x as f32);
                    }
                    Ok(mv) => {
                        errors.push(
                        Error::InvalidMetaValue(key_str, mv)
                            .spanned(value)
                        );
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            },
            "ingr" => {
                if self.meta.ingredients.len() >= 5 {
                    errors.push(
                    Error::TooManyIngredients.spanned(value)
                    );
                    return;
                }
                match cir::MetaValue::parse_option(value.as_ref()) {
                    Ok(cir::MetaValue::String(x)) => {
                        todo!()
                        // // currently we only support looking up by english
                        // match self.resolver.resolve(&x).await {
                        //     Some(item) => {
                        //         self.meta.ingredients.push(item);
                        //     }
                        //     None => {
                        //         errors.push(
                        //         Error::InvalidItem(x)
                        //             .spanned(value)
                        //         );
                        //     }
                        // }
                    }
                    Ok(mv) => {
                        errors.push(
                        Error::InvalidMetaValue(key_str, mv)
                            .spanned(value)
                        );
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            }
            "star" => {
                match cir::MetaValue::parse_option(value.as_ref()) {
                    Ok(cir::MetaValue::Int(x)) => {
                        if x < 0 || x > 4 {
                            errors.push(
                            Error::InvalidArmorStarNum(x as i32)
                                .spanned(value)
                            );
                            return;
                        }
                        self.meta.star = Some(x as i32);
                    }
                    Ok(mv) => {
                        errors.push(
                        Error::InvalidMetaValue(key_str, mv)
                            .spanned(value)
                        );
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            },
            _ => {
                errors.push(
                Error::UnusedMetaKey(key_str).spanned_warning(&span)
                );
            }
        }
    }

    async fn visit_end(&mut self, _meta: &syn::ItemMeta, _errors: &mut Vec<ErrorReport>) {
        
    }

    async fn finish(self) -> Self::Output {
        self.meta
    }
}

fn parse_weapon_modifier_bits(value: &str) -> Option<i32> {
    let value = value.replace("_", "").replace("-", "").replace(" ", "").to_ascii_lowercase();
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
    let value = value.replace("_", "").replace("-", "").replace(" ", "").to_ascii_lowercase();
    match value.trim() {
        "hearty" | "lifemaxup" => Some(2),
        "chilly" | "chill" | "resisthot"=> Some(4),
        "spicy" | "resistcold" => Some(5),
        "electro" | "resistelectric" => Some(6),
        "mighty" | "attack" | "attackup" => Some(10),
        "tough" | "defense" | "defenseup" => Some(11),
        "sneaky" | "quiet" | "stealth" | "stealthup" | "quietness" => Some(12),
        "speed" | "speedup" | "allspeed" | "movingspeed"  => Some(13),
        "energizing" | "stamina" | "staminaup" | "stam" | "stamup" | "gutsrecover" | "guts" => Some(14),
        "enduring" | "endura" | "endur" | "exgutsmaxup" | "exguts" => Some(15),
        "fire" | "fireproof" | "resistflame" | "resistfire" => Some(16),
        _ => None,
    }
}
