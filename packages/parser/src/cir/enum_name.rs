//! These are non-keyword "enum" names that can be matched
//! by commands in the right context

/// Parse cook effect name for the `effect` meta property for items
#[rustfmt::skip]
pub fn parse_cook_effect(value: &str) -> Option<i32> {
    match clean_ident(value).as_str() {
        // @manual-generator-hint cook-effects
        // No cook effect
        "none"
        => Some(-1),
        // "Hearty" Cook Effect
        "hearty" | "lifemaxup"
        => Some(2),
        // "Chilly" Cook Effect
        "chilly" | "chill" | "resisthot" 
        => Some(4),
        // "Spicy" Cook Effect
        "spicy" | "resistcold"
        => Some(5),
        // "Electro" Cook Effect
        "electro" | "resistelectric" 
        => Some(6),
        // "Mighty" (Attack Up) Cook Effect
        "mighty" | "attack" | "attackup"
        => Some(10),
        // "Tough" (Defense Up) Cook Effect
        "tough" | "defense" | "defenseup"
        => Some(11),
        // "Sneaky" (Stealth Up) Cook Effect
        "sneaky" | "quiet" | "stealth" | "stealthup" | "quietness"
        => Some(12),
        // "Hasty" (Speed Up) Cook Effect
        "hasty" | "speed" | "speedup" | "allspeed" | "movingspeed"
        => Some(13),
        // "Energizing" (Stamina Up) Cook Effect
        "stamina" | "energizing" | "staminaup" | "stam" | "stamup" | "gutsrecover" | "guts"
        => Some(14),
        // "Enduring" (Yellow Stamina) Cook Effect
        "endura" | "enduring" | "endur" | "exgutsmaxup" | "exguts"
        => Some(15),
        // "Fireproof" Cook Effect
        "fire" | "fireproof" | "resistflame" | "resistfire"
        => Some(16),
        // @manual-generator-hint end
        _ => None,
    }
}

/// Parse weapon modifier bit flag for the `modifier` meta property for items
#[rustfmt::skip]
pub fn parse_weapon_modifier_bits(value: &str) -> Option<i32> {
    match clean_ident(value).as_str() {
        // @manual-generator-hint weapon-modifiers
        // No modifier
        "none"
        => Some(0),
        // Attack Up (Weapon or Bow)
        "attack" | "attackup" | "addpower" 
        => Some(0x1), 
        // Attack Up+ (Weapon or Bow)
        "addpowerplus" 
        => Some(0x80000001u32 as i32),
        // Durability Up
        "durability" | "durabilityup" | "addlife" 
        => Some(0x2),
        // Durability Up+
        "addlifeplus" 
        => Some(0x80000002u32 as i32),
        // Critical Hit
        "critical" | "criticalhit" 
        => Some(0x4),
        // Long Throw
        "longthrow" | "throw" 
        => Some(0x8),
        // Bow Multishot
        "multishot" | "spreadfire" 
        => Some(0x10),
        // Bow Zoom
        "zoom" => Some(0x20),
        // Bow Quickshot
        "quickshot" | "rapidfire" 
        => Some(0x40),
        // Shield Surf Up
        "surfmaster" | "surf" | "shieldsurf" | "shieldsurfup" | "surfup"
        => Some(0x80),
        // Shield Guard Up
        "guard" | "guardup" | "addguard" 
        => Some(0x100),
        // Shield Guard Up+
        "addguardplus"
        => Some(0x80000100u32 as i32),
        // Make the modifier "Yellow"
        "plus" | "yellow"
        => Some(0x80000000u32 as i32),
        // @manual-generator-hint end
        _ => None,
    }
}

fn clean_ident(value: &str) -> String {
    value
        .trim()
        .replace(['_', '-', ' '], "")
        .to_ascii_lowercase()
}
