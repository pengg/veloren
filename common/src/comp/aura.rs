use crate::{
    comp::buff::{BuffCategory, BuffData, BuffKind, BuffSource},
    uid::Uid,
};
use serde::{Deserialize, Serialize};
use slotmap::{new_key_type, SlotMap};
use specs::{Component, DerefFlaggedStorage};
use specs_idvs::IdvStorage;
use std::time::Duration;

new_key_type! { pub struct AuraKey; }

/// AuraKind is what kind of effect an aura applies
/// Currently only buffs are implemented
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AuraKind {
    /// The Buff kind is (surprise!) a buff :D
    Buff {
        kind: BuffKind,
        data: BuffData,
        category: BuffCategory,
        source: BuffSource,
    },
    /* TODO: Implement other effects here. Things to think about
     * are terrain/sprite effects, collision and physics, and
     * environmental conditions like temperature and humidity
     * Multiple auras can be given to an entity. */
}

/// Aura
/// Applies a buff to entities in the radius if meeting
/// conditions set forth in the aura system.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Aura {
    /// The kind of aura applied
    pub aura_kind: AuraKind,
    /// The radius of the aura
    pub radius: f32,
    /// How long the aura lasts. None corresponds to an indefinite length
    pub duration: Option<Duration>,
    /* TODO: Add functionality for fading or a gradient */
    /// Used to filter which entities this aura will apply to. For example,
    /// globally neutral auras which affect all entities will have the type
    /// `AuraTarget::All`. Whereas auras which only affect a player's party
    /// members will have the type `AuraTarget::GroupOf`.
    pub target: AuraTarget,
}

/// Information about whether aura addition or removal was requested.
/// This to implement "on_add" and "on_remove" hooks for auras
#[derive(Clone, Debug)]
pub enum AuraChange {
    /// Adds this aura
    Add(Aura),
    /// Removes auras of these indices
    RemoveByKey(Vec<AuraKey>),
}

/// Used by the aura system to filter entities when applying an effect.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AuraTarget {
    /// Targets the group of the entity specified by the `Uid`. This is useful
    /// for auras which should only affect a player's party.
    GroupOf(Uid),

    /// Targets all entities. This is for auras which are global or neutral.
    All,
}

impl Aura {
    /// Creates a new Aura to be assigned to an entity
    pub fn new(
        aura_kind: AuraKind,
        radius: f32,
        duration: Option<Duration>,
        target: AuraTarget,
    ) -> Self {
        Self {
            aura_kind,
            radius,
            duration,
            target,
        }
    }
}

/// Component holding all auras emitted by an entity.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Auras {
    pub auras: SlotMap<AuraKey, Aura>,
}

impl Auras {
    pub fn new(aura: Aura) -> Self {
        let mut auras: SlotMap<AuraKey, Aura> = SlotMap::with_key();
        auras.insert(aura);
        Self { auras }
    }

    pub fn insert(&mut self, aura: Aura) { self.auras.insert(aura); }

    pub fn remove(&mut self, key: AuraKey) { self.auras.remove(key); }
}

impl Component for Auras {
    type Storage = DerefFlaggedStorage<Self, IdvStorage<Self>>;
}
