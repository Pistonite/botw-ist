use crate::sim;
use skybook_parser::cir;

impl sim::Actor {
    /// Returns if the overworld actor matches the item selector
    pub fn matches(&self, matcher: &cir::ItemMatchSpec) -> bool {
        if !sim::util::name_spec_matches(&matcher.name, &self.name) {
            return false;
        }
        let meta = matcher.meta.as_ref();
        // matching value for overworld actors is mostly
        // used for weapons, since materials can only have value = 1
        if let Some(wanted_value) = meta.and_then(|x| x.value)
            && wanted_value != self.value
        {
            return false;
        }
        if let Some(wanted_mod_value) = meta.and_then(|x| x.life_recover)
            && self.modifier.is_none_or(|m| m.value != wanted_mod_value)
        {
            return false;
        }

        if let Some(wanted_flags) = meta.and_then(|x| x.sell_price)
            && self.modifier.is_none_or(|m| {
                !sim::util::modifier_meta_matches(&matcher.name, wanted_flags, m.flags as i32)
            })
        {
            return false;
        }

        true
    }
}
