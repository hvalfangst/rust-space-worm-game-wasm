use crate::state::structs::{Snake, Food, Direction, Vector2D};

pub fn update_game_logic(
    player: &mut Snake,
    food: &mut Food,
    score: &mut u32,
    food_score_value: u32,
    perk_eligibility: &mut bool,
    in_perk_selection: &mut bool,
    highlighted_perk: &mut Option<usize>,
    perk_required_score: u32,
    stars_offset_x: &mut usize,
    stars_sprite_frame_index: &mut usize,
    stars_last_sprite_frame_update_time: &mut f64,
    globe_sprite_frame_index: &mut usize,
    globe_last_sprite_frame_update_time: &mut f64,
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
    let food_eaten = crate::state::r#loop::check_food_collision(
        player,
        food,
        score,
        food_score_value,
    );

    if food_eaten {
        // Check if player is eligible for a perk based on the score
        if crate::state::core::perks::check_perk_eligibility(*score, perk_required_score) {
            *perk_eligibility = true;
            *in_perk_selection = true;
            *highlighted_perk = Some(1); // Default to first perk
        }
    }

    // Update head sprite animation (following original logic)
    crate::state::r#loop::update_head_sprite_animation(player, delta_time);

    Ok(false) // No game over
}

pub fn restart_game(
    player: &mut Snake,
    food: &mut Food,
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
    perk_eligibility: &mut bool,
    selected_perk: &mut Option<usize>,
    food_score_value: &mut u32,
    in_perk_selection: &mut bool,
    highlighted_perk: &mut Option<usize>,
    perk_selection_keys: &mut std::collections::HashMap<String, bool>,
) {
    // Reset the game state
    *player = Snake::new(40.0, 150.0, Direction::Right);
    *food = Food {
        position: Vector2D { x: 200.0, y: 200.0 },
        is_active: true,
        food_sprite_frame_index: 0,
        food_last_sprite_frame_index_update_time: 0.0,
    };
    *score = 0;
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
    // Reset perk system
    *perk_eligibility = false;
    *selected_perk = None;
    *food_score_value = 100;
    *in_perk_selection = false;
    *highlighted_perk = None;
    perk_selection_keys.clear();
}

pub fn update_game_over_animation(
    game_over_frame: &mut usize,
    game_over_darkness: &mut f32,
    game_over_animation_time: &mut f64,
) -> bool {
    let current_time = js_sys::Date::now();

    // Check if enough time has passed for next frame (200ms per frame)
    if current_time - *game_over_animation_time >= 200.0 {
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
