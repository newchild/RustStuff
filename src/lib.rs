pub mod player;
pub mod targetable;
pub mod spells;

use player::Player;

/// Returns the active player
///
/// # Example
///
/// ```
/// use elosuite::Player;
/// let user = get_player();
/// println!(user.name);
/// ```
pub fn get_player() -> Player{
    let player = Player{
        max_mana : 0,
        max_health : 0,
        current_mana : 0,
        current_health : 0,
        name : "Test".to_string(),
        champion : "Tryndamere".to_string(),
        handle : 12
    };
    player
}


