use blueflame::game::WeaponModifierInfo;

#[derive(Default, Clone)]
pub struct OverworldSystem {
    pub weapon: Option<OverworldActor>,
    pub bow: Option<OverworldActor>,
    pub shield: Option<OverworldActor>,

    pub ground: Vec<OverworldActor>,
    pub holding: Vec<OverworldActor>,
}

/// Simulates an actor in the overworld
#[derive(Default, Clone)]
pub struct OverworldActor {
    /// Name of the actor
    pub name: String,
    /// Value for weapons (1 for materials)
    pub value: i32,
    /// Weapon modifier if any, None if not weapon
    pub modifier: Option<WeaponModifierInfo>,
}

impl OverworldSystem {
    /// Spawn additional items held by the player (does not replacing existing)
    pub fn spawn_held_items(&mut self, items: Vec<String>) {
        for item in items {
            self.holding.push(OverworldActor {
                name: item,
                value: 1,
                modifier: None,
            });
        }
    }

    /// Drop items held by the player to the ground
    pub fn drop_held_items(&mut self) {
        self.ground.extend(std::mem::take(&mut self.holding))
    }
}
