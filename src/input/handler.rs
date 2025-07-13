use crate::state::structs::{Direction, GameState};
use minifb::{Key, KeyRepeat};

pub fn handle_user_input(game_state: &mut GameState) {
    let key_direction_map = [
        (Key::W, Direction::Up),
        (Key::A, Direction::Left),
        (Key::S, Direction::Down),
        (Key::D, Direction::Right),
    ];

    for (key, direction) in key_direction_map.iter() {
        if game_state.window.is_key_pressed(*key, KeyRepeat::Yes) {
            game_state.player.direction = *direction;
            break; // Only process first pressed key
        }
    }
}