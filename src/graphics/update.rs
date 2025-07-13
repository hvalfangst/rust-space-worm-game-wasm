use crate::graphics::sprites::{draw_sprite, draw_sprite_with_gradient_shading};
use crate::graphics::text::{get_font_data, BitFont};
use crate::state::constants::graphics::{ART_HEIGHT, ART_WIDTH};
use crate::state::constants::text::{PERK_CURSE_OF_GLOSSY, PERK_HUNGRY_WORM, PERK_NEED_4_SPEED, SCORE, SELECT_PERK};
use crate::state::structs::{Direction, GameState};

pub fn update_pixel_buffer(game_state: &mut GameState) {
    draw_background(game_state);
    draw_food(game_state);
    draw_player(game_state);
    draw_score(game_state);
}

fn draw_score(game_state: &mut GameState) {
    let score_text = game_state.score.to_string();
    let char_width = 6;
    let char_spacing = 2;

    let total_width = (score_text.len() * char_width) + ((score_text.len() - 1) * char_spacing);
    let start_x = (ART_WIDTH / 2) - (total_width / 2);
    let start_y = 10;

    for (i, ch) in score_text.chars().enumerate() {
        let x_pos = start_x + (i * (char_width + char_spacing));
        let font_data = get_font_data();
        let bit_font = BitFont { chars: font_data };
        bit_font.draw_text_smooth_scaled(&mut game_state.window_buffer, ART_WIDTH, &ch.to_string(), x_pos as i32, start_y, 0xFFFFFF, 1.0);
    }
}

fn draw_food(game_state: &mut GameState) {
    let food_x = game_state.food.position.x;
    let darkness = calculate_darkness(food_x);

    // Draw the food sprite at the food's position
    draw_sprite(
        food_x as usize,
        game_state.food.position.y as usize,
        &game_state.sprites.food[*&game_state.food.food_sprite_frame_index],
        game_state.window_buffer,
        ART_WIDTH,
        darkness
    );
}

fn draw_player(game_state: &mut GameState) {

    let head_position = &game_state.player.body[0];
    let darkness = calculate_darkness(head_position.x);

    // Magic number offset based on direction
    let offset: f32 = match game_state.player.direction {
        Direction::Right => 0.0,
        Direction::Left => 10.0,
        Direction::Up => 7.0,
        Direction::Down => 0.0,
    };

    // Draw head first
    draw_sprite(
        (head_position.x - offset) as usize,
        (head_position.y - offset) as usize,
        &game_state.sprites.head[game_state.player.head_sprite_frame_index],
        game_state.window_buffer,
        ART_WIDTH,
        darkness
    );


    // Draw the body segments from neck to buttocks
    for i in 1..game_state.player.body.len() -1 {
        let body_x = game_state.player.body[i].x;
        let darkness = calculate_darkness(body_x);

        draw_sprite(
            body_x as usize,
            game_state.player.body[i].y as usize,
            &game_state.sprites.body[game_state.player.body_sprite_frame_index],
            game_state.window_buffer,
            ART_WIDTH,
            darkness
        );
    }

    // For right and up we draw the first tail sprite frame, left and down we draw the second tail sprite frame
    let tail_sprite_index = if game_state.player.direction == Direction::Right || game_state.player.direction == Direction::Up {
        0
    } else {
        1
    };

    let tail_index = game_state.player.body.len();
    if tail_index > 0 {
        let tail_position = &game_state.player.body[tail_index - 1];
        let darkness = calculate_darkness(tail_position.x);
        draw_sprite(
            tail_position.x as usize,
            tail_position.y as usize,
            &game_state.sprites.tail[tail_sprite_index],
            game_state.window_buffer,
            ART_WIDTH,
            darkness
        );
    }
}

pub fn draw_game_over_screen(game_state: &mut GameState, index: usize, darkness_factor: Option<f32>) {

    draw_sprite(
        0,
        0,
        &game_state.sprites.game_over_screen[index],
        game_state.window_buffer,
        ART_WIDTH,
        darkness_factor
    );

    // Draw the score underneath the "Game Over" screen
    let score_text = format!("{}{}", SCORE, game_state.score);
    let x_position = (ART_WIDTH as i32/ 2) - (score_text.len() as i32 * 4); // Adjust for centering
    let y_position = ART_HEIGHT - 20; // Position near the bottom
    let font_data = get_font_data();
    let bit_font = BitFont { chars: font_data };
    bit_font.draw_text_smooth_scaled(&mut game_state.window_buffer, ART_WIDTH, &score_text, x_position, y_position as i32, 0xFFFFFF, 1.0);
}

pub fn draw_choose_perk_screen_with_highlight(game_state: &mut GameState, highlighted_perk: Option<usize>) {

    // Draw top part of the perk screen which prompts the user for selection
    draw_sprite(
        0,
        0,
        &game_state.sprites.choose_perk[0],
        game_state.window_buffer,
        ART_WIDTH,
        None
    );

    // Get the font data for drawing text
    let font_data = get_font_data();

    // Create a BitFont instance
    let bit_font = BitFont { chars: font_data };

    // Draw the "Select perk" text at the top of the screen
    bit_font.draw_text_smooth_scaled(&mut game_state.window_buffer, ART_WIDTH, SELECT_PERK, 57, 25, 0xFFFFFF, 1.7); // White color

    // Designate the bottom part of the perk screen which shows the two available perks
    let perk_positions = [
        (0, ART_HEIGHT / 2),
        (128, ART_HEIGHT / 2),
    ];

    // Draw the perks at their designated positions with highlight added based on highlighted_perk
    for (i, &(x, y)) in perk_positions.iter().enumerate() {
        let perk_index = i + 1;
        let is_highlighted = highlighted_perk == Some(perk_index);
        let is_selected = game_state.selected_perk == Some(perk_index);

        if i < game_state.sprites.perks.len() {
            let darkness_factor = if is_selected {
                None // Selected - full brightness
            } else if is_highlighted {
                Some(0.8) // Highlighted - slightly dim
            } else {
                Some(0.5) // Normal - more dim
            };

            draw_sprite(
                x,
                y,
                &game_state.sprites.perks[i],
                game_state.window_buffer,
                ART_WIDTH,
                darkness_factor,
            );
        }
    }

    // Must also draw information about the perk which is highlighted

    if let Some(perk_index) = highlighted_perk {
        let perk_info = match perk_index {
            1 => PERK_NEED_4_SPEED,
            2 => PERK_HUNGRY_WORM,
            _ => PERK_CURSE_OF_GLOSSY,
        };

        // Draw the first line of perk information
        bit_font.draw_text_smooth_scaled(
            &mut game_state.window_buffer,
            ART_WIDTH,
            perk_info.0,
            75, // X position
            55, // Y position
            0xFFD700, // Golden color
            1.0 // Scale
        );

        // Draw the second line of perk information
        bit_font.draw_text_smooth_scaled(
            &mut game_state.window_buffer,
            ART_WIDTH,
            perk_info.1,
            52, // X position
            69, // Y position
            0xCCCCCC, // Slightly grey color
            1.0 // Scale
        );
    }
}


pub fn draw_background(state: &mut GameState) {

    // Always first draw a subset (the top 200 pixels) of our background in order to mitigate void spots
    draw_sprite(
        0,
        0,
        &state.sprites.blue_strip[0],
        state.window_buffer,
        ART_WIDTH,
        None
    );

    // Loop through the layers and draw them based on the player's position
    for (i, divisor) in [12, 1].iter().enumerate() {
        // Calculate offsets for parallax effect
        let (offset_x, offset_y) = if i == 0 {
            (
                state.stars_offset_x / divisor,
                0
            )
        } else {
            (0, 0)
        };

        // Increment the x offset for layer 0
        if i == 0 {
            state.stars_offset_x = state.stars_offset_x + 1;
        }

        // Select the appropriate layer based on the index
        let layer = match i {
            0 => &state.sprites.stars[state.stars_sprite_frame_index],
            1 => &state.sprites.planet[state.globe_sprite_frame_index],
            _ => unreachable!(),
        };

        if i == 0 {
            // Normal draw for first layer
            draw_sprite(
                offset_x,
                offset_y,
                layer,
                state.window_buffer,
                ART_WIDTH,
                None
            );
        } else { // Apply gradient shading based on pixel coordinates for the second layer since it's one unit
            draw_sprite_with_gradient_shading(
                offset_x,
                offset_y,
                layer,
                state.window_buffer,
                ART_WIDTH,
                |_sprite_col, _sprite_row, world_x, _world_y| {
                    let art_width_f = ART_WIDTH as f32;
                    let x_f = world_x as f32;

                    // Create smooth gradient from right side
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
}

/// Calculates the darkness level for a given x-coordinate based on its position
/// relative to the screen width.
///
/// # Parameters
/// - `x`: The x-coordinate as a `f32`.
///
/// # Returns
/// - `Option<f32>`: A darkness value between 0.6 and 0.8 if the x-coordinate falls
///   within specific ranges, or `None` if no darkness should be applied.
fn calculate_darkness(x: f32) -> Option<f32> {
    match x {
        x if x > ART_WIDTH as f32 / 1.7 => Some(0.6),
        x if x > ART_WIDTH as f32 / 1.8 => Some(0.7),
        x if x > ART_WIDTH as f32 / 1.9 => Some(0.8),
        _ => None,
    }
}