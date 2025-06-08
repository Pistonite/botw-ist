use std::borrow::Cow;

/// Given any armor, get the armor actor with the number of stars
///
/// Star is clamped between 0 and 4
pub fn get_armor_with_star(mut actor: &'_ str, star: i32) -> Cow<'_, str> {
    // special case for Snow Boots
    // change it to the version that's upgradable
    if actor == "Armor_140_Lower" {
        actor = "Armor_141_Lower";
    }
    let star = star.clamp(0, 4);
    // if input is not armor, return as is
    let Some(to_search) = actor.strip_prefix("Armor_") else {
        return Cow::Borrowed(actor);
    };
    for armor_group in crate::generated::ARMOR_UPGRADE {
        for i in 0..5 {
            if armor_group[i] == to_search {
                return Cow::Owned(format!("Armor_{}", armor_group[star as usize]));
            }
        }
    }

    Cow::Borrowed(actor)
}
