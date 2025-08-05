use crate::state::structs::{Direction, Food, LootCrate, Snake, Vector2D};

pub fn update_game_logic(
    player: &mut Snake,
    food: &mut Food,
    loot_crate: &mut LootCrate,
    score: &mut u32,
    food_score_value: u32,
    powerup_eligibility: &mut bool,
    in_powerup_selection: &mut bool,
    highlighted_powerup: &mut Option<usize>,
    stars_offset_x: &mut usize,
    stars_sprite_frame_index: &mut usize,
    stars_last_sprite_frame_update_time: &mut f64,
    globe_sprite_frame_index: &mut usize,
    globe_last_sprite_frame_update_time: &mut f64,
    last_loot_crate_check_time: &mut f64,
    delta_time: f32,
) -> Result<bool, wasm_bindgen::JsValue> {
    // Update background animation
    crate::state::r#loop::update_background_animation(
        stars_offset_x,
        stars_sprite_frame_index,
        stars_last_sprite_frame_update_time,
        globe_sprite_frame_index,
        globe_last_sprite_frame_update_time,
        delta_time,
    );

    // Update food sprite animation (following original logic)
    crate::state::r#loop::update_food_sprite_animation(food);

    // Update snake movement
    crate::state::r#loop::update_snake_movement(player, delta_time);

    // Check for self-collision (snake hitting itself)
    if crate::state::r#loop::check_self_collision(player) {
        return Ok(true); // Game over
    }

    // Check food collision and proximity
    crate::state::r#loop::check_food_collision(
        player,
        food,
        score,
        food_score_value,
    );

    // Check for timed loot crate spawning (25% chance every 10 seconds, only if none active)
    let current_time = js_sys::Date::now();
    if current_time - *last_loot_crate_check_time >= 10000.0 && !loot_crate.is_active {
        *last_loot_crate_check_time = current_time;
        
        // 25% chance to spawn loot crate
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        current_time.to_bits().hash(&mut hasher);
        let hash = hasher.finish();
        
        if (hash % 100) < 25 {
            crate::state::core::perks::spawn_loot_crate(loot_crate);
            web_sys::console::log_1(&"Loot crate spawned by timer (25% chance)".into());
        } else {
            web_sys::console::log_1(&"Loot crate timer triggered but no spawn (75% chance)".into());
        }
    }

    // Check loot crate collision (same as food collision)
    if crate::state::r#loop::check_loot_crate_collision(player, loot_crate, powerup_eligibility, in_powerup_selection, highlighted_powerup) {
        // Loot crate was eaten, no additional logic needed here
    }

    // Update loot crate sprite animation
    if loot_crate.is_active {
        crate::state::r#loop::update_loot_crate_sprite_animation(loot_crate);
    }

    // Update head sprite animation (following original logic)
    crate::state::r#loop::update_head_sprite_animation(player, delta_time);

    Ok(false) // No game over
}

pub fn restart_game(
    player: &mut Snake,
    food: &mut Food,
    loot_crate: &mut LootCrate,
    score: &mut u32,
    game_over: &mut bool,
    last_frame_time: &mut Option<f64>,
    stars_offset_x: &mut usize,
    stars_sprite_frame_index: &mut usize,
    stars_last_sprite_frame_update_time: &mut f64,
    globe_sprite_frame_index: &mut usize,
    globe_last_sprite_frame_update_time: &mut f64,
    game_over_frame: &mut usize,
    game_over_darkness: &mut f32,
    game_over_animation_time: &mut f64,
    last_loot_spawn_score: &mut u32,
    powerup_eligibility: &mut bool,
    selected_powerup: &mut Option<crate::state::core::perks::Perk>,
    food_score_value: &mut u32,
    in_powerup_selection: &mut bool,
    highlighted_powerup: &mut Option<usize>,
    powerup_selection_keys: &mut std::collections::HashMap<String, bool>,
    last_loot_crate_check_time: &mut f64,
) {
    // Reset the game state

    *player = Snake::new(40.0, 150.0, Direction::Right);
    *food = Food {
        position: Vector2D { x: 200.0, y: 200.0 },
        is_active: true,
        food_sprite_frame_index: 0,
        food_last_sprite_frame_index_update_time: 0.0,
    };

    *loot_crate = LootCrate {
        position: Vector2D { x: 0.0, y: 0.0 },
        is_active: false,
        sprite_frame_index: 0,
        last_sprite_frame_index_update_time: 0.0,
    };

    *score = 0;
    *last_loot_spawn_score = 0;
    *game_over = false;
    *last_frame_time = None;

    // Reset background animation
    *stars_offset_x = 0;
    *stars_sprite_frame_index = 0;
    *stars_last_sprite_frame_update_time = 0.0;
    *globe_sprite_frame_index = 0;
    *globe_last_sprite_frame_update_time = 0.0;

    // Reset game over animation
    *game_over_frame = 0;
    *game_over_darkness = 0.5;
    *game_over_animation_time = 0.0;

    // Reset powerup system
    *powerup_eligibility = false;
    *selected_powerup = None;
    *food_score_value = 100;
    *in_powerup_selection = false;
    *highlighted_powerup = None;
    powerup_selection_keys.clear();

    // Reset loot crate timer
    *last_loot_crate_check_time = js_sys::Date::now();
}

pub fn update_game_over_animation(
    game_over_frame: &mut usize,
    game_over_darkness: &mut f32,
    game_over_animation_time: &mut f64,
) -> bool {
    let current_time = js_sys::Date::now();

    // Check if enough time has passed for next frame (500ms per frame)
    if current_time - *game_over_animation_time >= 500.0 {
        *game_over_frame += 1;
        *game_over_darkness = (*game_over_darkness + 0.1).min(0.8);
        *game_over_animation_time = current_time;

        // After 8 frames, restart the game
        if *game_over_frame >= 8 {
            return true; // Signal to restart
        }
    }
    false
}
