use targetable::IsTargetable;

pub enum SpellSlot{
    Q,
    W,
    E,
    R
}


pub trait SpellCaster{
    fn cast_spell_target<T: IsTargetable>(&self, slot : SpellSlot, target : T);
}
