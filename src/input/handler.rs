use crate::state::structs::Direction;
use std::collections::HashMap;

pub fn handle_key_down(
    key_code: &str,
    player_direction: &mut Direction,
    game_over: bool,
    in_powerup_selection: bool,
    powerup_selection_keys: &mut HashMap<String, bool>,
) {
    if game_over {
        return; // Don't handle input when game is over
    }

    // Handle powerup selection keys
    if in_powerup_selection {
        powerup_selection_keys.insert(key_code.to_string(), true);
        return;
    }

    let new_direction = match key_code {
        "KeyW" => Some(Direction::Up),
        "KeyS" => Some(Direction::Down),
        "KeyA" => Some(Direction::Left),
        "KeyD" => Some(Direction::Right),
        _ => None,
    };

    // Only change direction if it's not opposite to current direction
    if let Some(direction) = new_direction {
        let can_change = match (*player_direction, direction) {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) |
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => false,
            _ => true,
        };

        if can_change {
            *player_direction = direction;
        }
    }
}

pub fn handle_game_over_input(key_code: &str) -> bool {
    // Allow restarting the game with Space key
    key_code == "Space"
}