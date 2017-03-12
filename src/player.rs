use targetable::IsTargetable;
use spells::{SpellSlot, SpellCaster};

/// Any player
#[allow(dead_code)]
pub struct Player{
    /// Maximum mana the player has available
    pub max_mana : i32,
    /// Maximum health the player has available
    pub max_health : i32,
    /// Players current mana
    pub current_mana : i32,
    /// Players current health
    pub current_health : i32,
    /// Username of the Player
    pub name: String,
    /// Championname of the player
    pub champion : String,
    pub handle : u32
}

impl IsTargetable for Player{
    fn get_handle(&self) -> u32{
        self.handle
    }
}

impl SpellCaster for Player{
    fn cast_spell_target<T: IsTargetable>(&self, slot : SpellSlot, target : T){
        
    }
}
