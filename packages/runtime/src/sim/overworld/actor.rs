use blueflame::game::WeaponModifierInfo;
use derive_more::{Deref, DerefMut};

use crate::iv;


/// Simulates an actor in the game
#[derive(Debug, Default, Clone)]
pub struct Actor {
    /// Name of the actor
    pub name: String,
    /// Value for weapons (1 for materials)
    pub value: i32,
    /// Weapon modifier if any, None if not weapon
    pub modifier: Option<WeaponModifierInfo>,
}

impl Actor {
    pub fn to_equipped_iv(&self) -> iv::OverworldItem {
        iv::OverworldItem::Equipped {
            actor: self.name.clone(),
            value: self.value,
            modifier: self
                .modifier
                .map(|x| iv::WeaponModifier {
                    flag: x.flags as i32,
                    value: x.value,
                })
                .unwrap_or_default(),
        }
    }
    pub fn to_ground_weapon_iv(&self) -> iv::OverworldItem {
        iv::OverworldItem::GroundEquipment {
            actor: self.name.clone(),
            value: self.value,
            modifier: self
                .modifier
                .map(|x| iv::WeaponModifier {
                    flag: x.flags as i32,
                    value: x.value,
                })
                .unwrap_or_default(),
        }
    }
    pub fn to_held_iv(&self) -> iv::OverworldItem {
        iv::OverworldItem::Held {
            actor: self.name.clone(),
        }
    }
    pub fn to_ground_item_iv(&self, is_despawning: bool) -> iv::OverworldItem {
        iv::OverworldItem::GroundItem {
            actor: self.name.clone(),
            despawning: is_despawning,
        }
    }
}

/// Same as [`Actor`], with the invariant
/// that this represents the actor was successfully spawned.
///
/// This is used to simulate the action creation process
/// in the game, which can fail. See [`ActorCreator`]
#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct SpawnedActor(Actor);
impl AsRef<Actor> for SpawnedActor {
    fn as_ref(&self) -> &Actor {
        &self.0
    }
}
impl From<SpawnedActor> for Actor {
    fn from(value: SpawnedActor) -> Self {
        value.0
    }
}

/// Simulate creating actors
#[derive(Debug, Clone)]
pub struct ActorCreator {
    /// If actor can be spawned successfully.
    ///
    /// This is used to implement menu overloading.
    /// When overloaded, the actor system has exhausted
    /// the ProcHandle pool, causing certain creation requests
    /// to fail. We can't simulate the pool correctly because
    /// we don't create actors in the same way as the game,
    /// so, we are using this flag to simulate if the pool is exhausted
    pub is_actor_creation_allowed: bool,
}

impl Default for ActorCreator {
    fn default() -> Self {
        Self {
            is_actor_creation_allowed: true
        }
    }
}

impl ActorCreator {
    /// Try spawning the actor, if actor creation is allowed
    #[inline]
    pub fn try_spawn(&self, actor: Actor) -> Result<SpawnedActor, Actor> {
        if !self.is_actor_creation_allowed {
            Err(actor)
        } else {
            Ok(SpawnedActor(actor))
        }
    }

    /// Try spawning the actor with a value of 1
    #[inline]
    pub fn try_spawn_value_1(&self, name: String) -> Result<SpawnedActor, Actor> {
        self.try_spawn(Actor {
            name, value: 1, modifier: None
        })
    }

    /// Force spawn an actor, can be used to simulate where an actor
    /// is already spawned before, or for things like shooting arrows while
    /// in overworld
    #[inline]
    pub fn force_spawn(&self, actor: Actor) -> SpawnedActor {
        SpawnedActor(actor)
    }
}
