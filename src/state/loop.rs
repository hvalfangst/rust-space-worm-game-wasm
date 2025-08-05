use crate::state::structs::{Snake, Food, LootCrate, Direction};
use crate::state::constants::graphics::{SNAKE_BODY_WIDTH, SNAKE_BODY_HEIGHT};
use crate::state::constants::physics::{LOWER_BOUND_X, LOWER_BOUND_Y, UPPER_BOUND_X, UPPER_BOUND_Y};
use crate::state::constants::graphics::{ART_WIDTH, ART_HEIGHT};

pub fn update_snake_movement(player: &mut Snake, delta_time: f32) {
    // Update snake movement timer
    player.move_timer += delta_time;
    
    // Move snake when timer exceeds interval
    if player.move_timer >= player.move_interval {
        player.move_timer = 0.0;
        
        // Move body segments first (from tail to neck)
        let body_size = player.body.len();
        if body_size > 1 {
            for i in 1..body_size {
                let target_index = body_size - i;
                let source_index = body_size - i - 1;
                player.body[target_index] = player.body[source_index];
            }
        }
        
        // Then move the head
        if let Some(head) = player.body.first_mut() {
            match player.direction {
                Direction::Right => head.x += 6.0,
                Direction::Left => head.x -= 6.0,
                Direction::Up => head.y -= 8.0,
                Direction::Down => head.y += 8.0,
            }
            
            // Handle bounds wrapping like the original
            // Wrap horizontally
            if head.x < LOWER_BOUND_X {
                head.x = UPPER_BOUND_X - SNAKE_BODY_WIDTH; // Appear at right
            } else if head.x > UPPER_BOUND_X {
                head.x = LOWER_BOUND_X + SNAKE_BODY_WIDTH; // Appear at left
            }
            
            // Wrap vertically  
            if head.y < LOWER_BOUND_Y {
                head.y = UPPER_BOUND_Y - SNAKE_BODY_HEIGHT; // Appear at top
            } else if head.y > UPPER_BOUND_Y {
                head.y = LOWER_BOUND_Y + SNAKE_BODY_HEIGHT; // Appear at bottom
            }
        }
    }
}

pub fn check_self_collision(player: &Snake) -> bool {
    // Check for self-collision (snake hitting itself)
    if let Some(head) = player.body.first() {
        // Check if head collides with anybody segment (skip the head itself)
        for i in 1..player.body.len() {
            let body_segment = &player.body[i];
            let dx = head.x - body_segment.x;
            let dy = head.y - body_segment.y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance < 6.0 { // Collision threshold
                return true;
            }
        }
    }
    false
}

pub fn check_food_collision(player: &mut Snake, food: &mut Food, score: &mut u32, food_score_value: u32) -> bool {
    // Check food collision and proximity
    if food.is_active {
        if let Some(head) = player.body.first() {
            let dx = head.x - food.position.x;
            let dy = head.y - food.position.y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            // Check if food is near (within 24 pixels like the original)
            if distance < 24.0 {
                player.food_near = true;
                
                // Check if food is eaten (within 12 pixels like the original)
                if distance < 12.0 {
                    food.is_active = false;
                    *score += food_score_value;
                    
                    // Grow snake by adding a segment
                    if let Some(tail) = player.body.last() {
                        player.body.push(*tail);
                    }
                    
                    // Respawn food at random location
                    food.position.x = (js_sys::Math::random() * (ART_WIDTH as f64 - 60.0)) as f32;
                    food.position.y = (js_sys::Math::random() * (ART_HEIGHT as f64 - 60.0)) as f32;
                    food.is_active = true;
                    food.food_sprite_frame_index = 0;
                    food.food_last_sprite_frame_index_update_time = js_sys::Date::now();
                    
                    return true; // Food was eaten
                }
            } else {
                player.food_near = false;
            }
        }
    }
    false
}

pub fn update_background_animation(
    stars_offset_x: &mut usize,
    stars_sprite_frame_index: &mut usize,
    stars_last_sprite_frame_update_time: &mut f64,
    globe_sprite_frame_index: &mut usize,
    globe_last_sprite_frame_update_time: &mut f64,
    _delta_time: f32,
) {
    let current_time = js_sys::Date::now();
    
    // Update stars animation frame (blinking effect every 250ms)
    if current_time - *stars_last_sprite_frame_update_time >= 250.0 {
        *stars_sprite_frame_index = (*stars_sprite_frame_index + 1) % 6;
        *stars_last_sprite_frame_update_time = current_time;
    }
    
    // Update globe animation frame (rotation effect every 1000ms)
    if current_time - *globe_last_sprite_frame_update_time >= 1000.0 {
        *globe_sprite_frame_index = (*globe_sprite_frame_index + 1) % 6;
        *globe_last_sprite_frame_update_time = current_time;
    }
    
    // Update parallax offset (continuous scrolling)
    *stars_offset_x = stars_offset_x.wrapping_add(1);
}

pub fn update_head_sprite_animation(player: &mut Snake, _delta_time: f32) {
    if player.food_near {
        // When food is near, use sprite frame 3
        player.head_sprite_frame_index = 3;
    } else {
        // Simple animation: alternate between frames 0 and 1 every 500ms
        let current_time = js_sys::Date::now();
        let cycle_position = (current_time as u64) % 1000; // 1 second cycle
        
        player.head_sprite_frame_index = if cycle_position < 500 { 0 } else { 1 };
    }
}

pub fn update_food_sprite_animation(food: &mut Food) {
    if !food.is_active {
        return; // No need to update if food is not active
    }

    let current_time = js_sys::Date::now();

    // Update food animation frame (toggle every 500ms)
    if current_time - food.food_last_sprite_frame_index_update_time >= 500.0 {
        food.food_sprite_frame_index = (food.food_sprite_frame_index + 1) % 2;
        food.food_last_sprite_frame_index_update_time = current_time;
    }
}

pub fn update_game_over_animation(
    game_over_frame: &mut usize,
    game_over_darkness: &mut f32,
    game_over_animation_time: &mut f64,
) -> bool {
    let current_time = js_sys::Date::now();
    
    // Check if enough time has passed for next frame (600ms per frame)
    if current_time - *game_over_animation_time >= 600.0 {
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

pub fn check_loot_crate_collision(
    player: &Snake, 
    loot_crate: &mut LootCrate,
    powerup_eligibility: &mut bool,
    in_powerup_selection: &mut bool,
    highlighted_powerup: &mut Option<usize>,
) -> bool {
    // Check loot crate collision (same pattern as food)
    if loot_crate.is_active {
        if let Some(head) = player.body.first() {
            let dx = head.x - loot_crate.position.x;
            let dy = head.y - loot_crate.position.y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            // Check if loot crate is eaten (within 12 pixels, same as food)
            if distance < 12.0 {
                loot_crate.is_active = false;
                
                // Trigger powerup selection (instead of adding score like food does)
                *powerup_eligibility = true;
                *in_powerup_selection = true;
                *highlighted_powerup = Some(1); // Default to first powerup
                
                return true; // Loot crate was eaten
            }
        }
    }
    false
}

pub fn update_loot_crate_sprite_animation(loot_crate: &mut LootCrate) {
    let current_time = js_sys::Date::now();

    // Update loot crate animation frame (toggle every 750 for glowing effect)
    if current_time - loot_crate.last_sprite_frame_index_update_time >= 750.0 {
        loot_crate.sprite_frame_index = (loot_crate.sprite_frame_index + 1) % 2;
        loot_crate.last_sprite_frame_index_update_time = current_time;
    }
}