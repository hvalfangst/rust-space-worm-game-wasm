use std::collections::HashMap;
use crate::state::constants::physics::{LOWER_BOUND_X, UPPER_BOUND_X, LOWER_BOUND_Y, UPPER_BOUND_Y};
use crate::state::structs::{Vector2D, LootCrate};

pub enum Perk {
    NeedForSpeed,
    HungryWorm
}

pub fn get_default_powerups() -> (Perk, Perk) {
    (Perk::NeedForSpeed, Perk::HungryWorm)
}

pub fn handle_powerup_selection(
    powerup_selection_keys: &mut HashMap<String, bool>,
    highlighted_powerup: &mut Option<usize>,
    selected_powerup: &mut Option<Perk>,
    powerup_eligibility: &mut bool,
    in_powerup_selection: &mut bool,
) -> bool {
    let (powerup1, powerup2) = get_default_powerups();

    // Handle A and D keys for powerup navigation
    if powerup_selection_keys.contains_key("KeyA") {
        *highlighted_powerup = Some(1);
        powerup_selection_keys.remove("KeyA");
    }
    if powerup_selection_keys.contains_key("KeyD") {
        *highlighted_powerup = Some(2);
        powerup_selection_keys.remove("KeyD");
    }

    // Handle Space key for powerup selection
    if powerup_selection_keys.contains_key("Space") {
        if let Some(powerup_index) = *highlighted_powerup {
            let chosen_powerup = match powerup_index {
                1 => powerup1,
                2 => powerup2,
                _ => powerup1, // Default fallback
            };
            *selected_powerup = Some(chosen_powerup);
            *powerup_eligibility = false;
            *in_powerup_selection = false;
            *highlighted_powerup = None;
            powerup_selection_keys.clear();
            return true;
        }
    }

    // Handle Escape key for default powerup selection
    if powerup_selection_keys.contains_key("Escape") {
        *selected_powerup = Some(powerup1); // Choose first powerup as default
        *powerup_eligibility = false;
        *in_powerup_selection = false;
        *highlighted_powerup = None;
        powerup_selection_keys.clear();
        return true;
    }

    false
}

pub fn apply_powerup_effect(powerup: &Perk, move_interval: &mut f32, food_score_value: &mut u32) {
    match powerup {
        Perk::NeedForSpeed => {
            // Speed boost: reduce move interval by 25%
            *move_interval *= 0.75;
        }
        Perk::HungryWorm => {
            // Double score: increase food score value by 2x
            *food_score_value *= 2;
        }
        _ => {}
    }
}

pub fn should_spawn_loot_crate_at_threshold() -> bool {
    // 20% chance to spawn loot crate
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    js_sys::Date::now().to_bits().hash(&mut hasher);
    let hash = hasher.finish();
    
    (hash % 100) < 20
}

pub fn spawn_loot_crate(loot_crate: &mut LootCrate) {
    println!("Spawning loot crate at random position");

    // Generate random position within bounds
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    js_sys::Date::now().to_bits().hash(&mut hasher);
    let hash = hasher.finish();
    
    let x_range = UPPER_BOUND_X - LOWER_BOUND_X - 32.0; // Account for sprite width
    let y_range = UPPER_BOUND_Y - LOWER_BOUND_Y - 32.0; // Account for sprite height
    
    let x = LOWER_BOUND_X + 16.0 + ((hash % 1000) as f32 / 1000.0) * x_range;
    let y = LOWER_BOUND_Y + 16.0 + (((hash >> 10) % 1000) as f32 / 1000.0) * y_range;
    
    loot_crate.position = Vector2D { x, y };
    loot_crate.is_active = true;
    loot_crate.sprite_frame_index = 0;
    loot_crate.last_sprite_frame_index_update_time = js_sys::Date::now();
}