use crate::graphics::sprites::{draw_sprite, draw_sprite_with_gradient_shading};
use crate::graphics::text::{get_font_data, BitFont};
use crate::graphics::sprites::SpriteMaps;
use crate::state::constants::graphics::{ART_WIDTH, ART_HEIGHT};
use crate::state::structs::{Direction, Snake, Food, LootCrate};
use crate::state::core::perks::{get_default_powerups, Perk};

fn get_powerup_sprite_indices() -> (usize, usize) {
    let (powerup1, powerup2) = get_default_powerups();
    (get_powerup_sprite_index(&powerup1), get_powerup_sprite_index(&powerup2))
}

fn get_powerup_sprite_index(powerup: &Perk) -> usize {
    match powerup {
        Perk::NeedForSpeed => 0,
        Perk::HungryWorm => 1,
        _ => 0
    }
}

fn get_powerup_info(powerup: &Perk) -> (&'static str, &'static str) {
    match powerup {
        Perk::NeedForSpeed => ("Need 4 Speed", "+25% movement speed"),
        Perk::HungryWorm => ("Hungry Worm", "2x score from food"),
        _ => ("Need 4 Speed", "+25% movement speed"),
    }
}

pub fn draw_score_text(art_buffer: &mut [u32], score: u32) {
    // Use the same font system as the original game
    let font_data = get_font_data();
    let bit_font = BitFont { chars: font_data };
    
    // Draw just the score number
    let score_text = score.to_string();
    let score_x = (ART_WIDTH as i32 / 2) - (score_text.len() as i32 * 4 / 2); // Center horizontally
    let score_y = 10; // Top of screen
    
    bit_font.draw_text_smooth_scaled(
        art_buffer,
        ART_WIDTH,
        &score_text,
        score_x,
        score_y,
        0xFFFFFFFF, // White color
        1.0 // Normal scale
    );
}

pub fn draw_food(art_buffer: &mut [u32], food: &Food, sprites: &SpriteMaps) {
    if food.is_active {
        if !sprites.food.is_empty() {
            // Use the appropriate sprite frame index, clamped to available sprites
            let sprite_index = food.food_sprite_frame_index;

            draw_sprite(
                food.position.x as usize,
                food.position.y as usize,
                &sprites.food[sprite_index],
                art_buffer,
                ART_WIDTH,
                None,
            );
        }
    }
}

pub fn draw_loot_crate(art_buffer: &mut [u32], loot_crate: &LootCrate, sprites: &SpriteMaps) {
    if loot_crate.is_active && !sprites.loot_crate.is_empty() {
        // Use the appropriate sprite frame index, clamped to available sprites
        let sprite_index = loot_crate.sprite_frame_index.min(sprites.loot_crate.len() - 1);

        draw_sprite(
            loot_crate.position.x as usize,
            loot_crate.position.y as usize,
            &sprites.loot_crate[sprite_index],
            art_buffer,
            ART_WIDTH,
            None,
        );
    }
}

pub fn draw_snake(art_buffer: &mut [u32], player: &Snake, sprites: &SpriteMaps) {
    // Draw snake - head first with directional offset
    if let Some(head_segment) = player.body.first() {
        if !sprites.head.is_empty() {
            // Apply directional offset like the original
            let offset: f32 = match player.direction {
                Direction::Right => 0.0,
                Direction::Left => 10.0,
                Direction::Up => 7.0,
                Direction::Down => 0.0,
            };
            
            // Use the appropriate sprite frame index, clamped to available sprites
            let sprite_index = player.head_sprite_frame_index.min(sprites.head.len() - 1);
            
            draw_sprite(
                (head_segment.x - offset) as usize,
                (head_segment.y - offset) as usize,
                &sprites.head[sprite_index],
                art_buffer,
                ART_WIDTH,
                None,
            );
        }
    }
    
    // Draw body segments (excluding head and tail)
    for i in 1..player.body.len().saturating_sub(1) {
        let segment = &player.body[i];
        if !sprites.body.is_empty() {
            draw_sprite(
                segment.x as usize,
                segment.y as usize,
                &sprites.body[0],
                art_buffer,
                ART_WIDTH,
                None,
            );
        }
    }
    
    // Draw tail (last segment, only if there's more than one segment)
    if player.body.len() > 1 {
        if let Some(tail_segment) = player.body.last() {
            if !sprites.tail.is_empty() {
                // For right and up we draw the first tail sprite frame, left and down we draw the second tail sprite frame
                let tail_sprite_index = if player.direction == Direction::Right || player.direction == Direction::Up {
                    0
                } else {
                    1
                };
                
                draw_sprite(
                    tail_segment.x as usize,
                    tail_segment.y as usize,
                    &sprites.tail[tail_sprite_index],
                    art_buffer,
                    ART_WIDTH,
                    None,
                );
            }
        }
    }
}

pub fn draw_parallax_background(
    art_buffer: &mut [u32],
    sprites: &SpriteMaps,
    stars_offset_x: usize,
    stars_sprite_frame_index: usize,
    globe_sprite_frame_index: usize,
) {
    // Fill with dark background first
    art_buffer.fill(0xFF001122);
    
    // Draw stars with parallax effect (layer 0)
    if !sprites.stars.is_empty() {
        let parallax_divisor = 12; // Same as original
        let offset_x = stars_offset_x / parallax_divisor;
        
        // Get the current star frame for blinking effect
        let star_frame_index = if sprites.stars.len() > 1 {
            stars_sprite_frame_index % sprites.stars.len()
        } else {
            0
        };
        
        let sprite = &sprites.stars[star_frame_index];
        let sprite_width = sprite.width as usize;
        
        // Calculate the actual offset within the sprite width (for seamless scrolling)
        let actual_offset = offset_x % sprite_width;
        
        // Draw multiple copies of the sprite to fill the entire screen width seamlessly
        let mut x_pos = -(actual_offset as i32);
        while x_pos < ART_WIDTH as i32 {
            draw_sprite(
                x_pos.max(0) as usize,
                0,
                sprite,
                art_buffer,
                ART_WIDTH,
                None,
            );
            x_pos += sprite_width as i32;
        }
    }
    

    if !sprites.planet.is_empty() {
        let globe_frame_index = if sprites.planet.len() > 1 {
            globe_sprite_frame_index % sprites.planet.len()
        } else {
            0
        };
        
        let globe_sprite = &sprites.planet[globe_frame_index];
        
        // Draw globe with gradient shading
        draw_sprite_with_gradient_shading(
            0,
            0,
            globe_sprite,
            art_buffer,
            ART_WIDTH,
            |_sprite_col, _sprite_row, world_x, _world_y| {
                let art_width_f = ART_WIDTH as f32;
                let x_f = world_x as f32;
                
                // Create smooth gradient from right side (same as original)
                if x_f > art_width_f / 1.9 {
                    // Calculate how far into the shaded region we are (0.0 to 1.0)
                    let shade_start = art_width_f / 1.9;
                    let shade_end = art_width_f / 1.7;
                    let progress = (x_f - shade_start) / (shade_end - shade_start);
                    let progress = progress.min(1.0).max(0.0);
                    
                    // Interpolate between 0.8 (light shade) and 0.6 (dark shade)
                    let darkness = 0.8 - (progress * 0.2);
                    Some(darkness)
                } else {
                    None
                }
            }
        );
    }
}

pub fn draw_game_over_screen(
    art_buffer: &mut [u32],
    sprites: &SpriteMaps,
    game_over_frame: usize,
    game_over_darkness: f32,
    score: u32,
) {
    // Draw game over screen sprite with darkness factor
    if !sprites.game_over_screen.is_empty() && game_over_frame < sprites.game_over_screen.len() {
        draw_sprite(
            0,
            0,
            &sprites.game_over_screen[game_over_frame],
            art_buffer,
            ART_WIDTH,
            Some(game_over_darkness),
        );
    }
    
    // Draw the score underneath the "Game Over" screen
    let score_text = format!("Score: {}", score);
    let x_position = (ART_WIDTH as i32 / 2) - (score_text.len() as i32 * 4); // Adjust for centering
    let y_position = ART_HEIGHT as i32 - 20; // Position near the bottom
    
    let font_data = get_font_data();
    let bit_font = BitFont { chars: font_data };
    bit_font.draw_text_smooth_scaled(
        art_buffer,
        ART_WIDTH,
        &score_text,
        x_position,
        y_position,
        0xFFFFFFFF, // White color
        1.0 // Normal scale
    );
}

pub fn draw_powerup_selection_screen(
    art_buffer: &mut [u32],
    sprites: &SpriteMaps,
    highlighted_powerup: Option<usize>,
) {
    // Draw the top part of the powerup screen (choose powerup prompt)
    if !sprites.choose_powerup.is_empty() {
        draw_sprite(
            0,
            0,
            &sprites.choose_powerup[0],
            art_buffer,
            ART_WIDTH,
            None,
        );
    }
    
    // Draw the "Select powerup" text at the top of the screen
    let font_data = get_font_data();
    let bit_font = BitFont { chars: font_data };
    bit_font.draw_text_smooth_scaled(
        art_buffer,
        ART_WIDTH,
        "Pick power",
        45,
        25,
        0xFFFFFFFF, // White color
        1.7 // Scale
    );
    
    // Get the default powerup sprite indices
    let powerup_sprite_indices = get_powerup_sprite_indices();
    
    // Draw the two powerup options
    let powerup_positions = [(0, ART_HEIGHT / 2), (128, ART_HEIGHT / 2)];
    let sprite_indices = [powerup_sprite_indices.0, powerup_sprite_indices.1];
    
    for (i, &(x, y)) in powerup_positions.iter().enumerate() {
        let powerup_index = i + 1;
        let is_highlighted = highlighted_powerup == Some(powerup_index);
        let sprite_index = sprite_indices[i];
        
        if sprite_index < sprites.powerups.len() {
            let darkness_factor = if is_highlighted {
                Some(0.8) // Highlighted - slightly dim
            } else {
                Some(0.5) // Normal - more dim
            };
            
            draw_sprite(
                x,
                y,
                &sprites.powerups[sprite_index],
                art_buffer,
                ART_WIDTH,
                darkness_factor,
            );
        }
    }
    
    // Draw information about the highlighted powerup
    if let Some(powerup_index) = highlighted_powerup {
        let (powerup1, powerup2) = get_default_powerups();
        let selected_powerup = match powerup_index {
            1 => &powerup1,
            2 => &powerup2,
            _ => &powerup1, // Default fallback
        };
        let (powerup_title, powerup_description) = get_powerup_info(selected_powerup);
        
        // Draw the first line of powerup information
        bit_font.draw_text_smooth_scaled(
            art_buffer,
            ART_WIDTH,
            powerup_title,
            75, // X position
            55, // Y position
            0xFFFFD700, // Golden color with full alpha
            1.0 // Scale
        );
        
        // Draw the second line of powerup information
        bit_font.draw_text_smooth_scaled(
            art_buffer,
            ART_WIDTH,
            powerup_description,
            52, // X position
            69, // Y position
            0xCCCCCCFF, // Slightly grey color with full alpha
            1.0 // Scale
        );
    }
}