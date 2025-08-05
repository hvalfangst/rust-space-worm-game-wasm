use image::GenericImageView;
use wasm_bindgen::prelude::*;

mod state;
mod graphics;
mod input;
mod audio;
mod platform;

use crate::graphics::sprites::SpriteMaps;
use crate::state::constants::graphics::{ART_HEIGHT, ART_WIDTH, SCALED_WINDOW_HEIGHT, SCALED_WINDOW_WIDTH};
use crate::state::structs::{Direction, Snake};


// Set up console error panic hook for better debugging
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

// #[wasm_bindgen]
// pub async fn load_sprite_from_url(sprite_path: &str, sprite_width: u32, sprite_height: u32) -> Result<js_sys::Array, JsValue> {
//     // Use fetch to load the image
//     let window = web_sys::window().ok_or("No window")?;
//     let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_str(sprite_path)).await?;
//     let resp: web_sys::Response = resp_value.dyn_into()?;
//     let array_buffer = wasm_bindgen_futures::JsFuture::from(resp.array_buffer()?).await?;
//     let uint8_array = js_sys::Uint8Array::new(&array_buffer);
//     let mut bytes = vec![0; uint8_array.length() as usize];
//     uint8_array.copy_to(&mut bytes);
//
//     // Load image from bytes using the image crate
//     let img = image::load_from_memory(&bytes).map_err(|e| JsValue::from_str(&format!("Failed to load image: {}", e)))?;
//     let rgba_img = img.to_rgba8();
//     let (map_width, map_height) = img.dimensions();
//
//     // Extract just the first sprite from the sprite sheet
//     let sprites_x = map_width / sprite_width;
//     let sprites_y = map_height / sprite_height;
//
//     if sprites_x == 0 || sprites_y == 0 {
//         return Err(JsValue::from_str("Invalid sprite dimensions"));
//     }
//
//     // Extract the first sprite (top-left)
//     let mut sprite_data = Vec::new();
//     for y in 0..sprite_height {
//         for x in 0..sprite_width {
//             let src_x = x as usize;
//             let src_y = y as usize;
//
//             if src_x < map_width as usize && src_y < map_height as usize {
//                 let pixel = rgba_img.get_pixel(src_x as u32, src_y as u32);
//                 let r = pixel[0] as u32;
//                 let g = pixel[1] as u32;
//                 let b = pixel[2] as u32;
//                 let a = pixel[3] as u32;
//                 sprite_data.push((a << 24) | (r << 16) | (g << 8) | b);
//             } else {
//                 sprite_data.push(0); // Transparent pixel
//             }
//         }
//     }
//
//     // Return as JS array: [width, height, ...pixel_data]
//     let result = js_sys::Array::new();
//     result.push(&JsValue::from(sprite_width));
//     result.push(&JsValue::from(sprite_height));
//     for pixel in sprite_data {
//         result.push(&JsValue::from(pixel));
//     }
//
//     Ok(result)
// }

#[wasm_bindgen]
pub async fn load_sprite_frame_from_url(sprite_path: &str, sprite_width: u32, sprite_height: u32, frame_index: u32) -> Result<js_sys::Array, JsValue> {
    // Use fetch to load the image
    let window = web_sys::window().ok_or("No window")?;
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_str(sprite_path)).await?;
    let resp: web_sys::Response = resp_value.dyn_into()?;
    let array_buffer = wasm_bindgen_futures::JsFuture::from(resp.array_buffer()?).await?;
    let uint8_array = js_sys::Uint8Array::new(&array_buffer);
    let mut bytes = vec![0; uint8_array.length() as usize];
    uint8_array.copy_to(&mut bytes);

    // Load image from bytes using the image crate
    let img = image::load_from_memory(&bytes).map_err(|e| JsValue::from_str(&format!("Failed to load image: {}", e)))?;
    let rgba_img = img.to_rgba8();
    let (map_width, map_height) = img.dimensions();

    // Calculate sprite sheet dimensions
    let sprites_x = map_width / sprite_width;
    let sprites_y = map_height / sprite_height;

    if sprites_x == 0 || sprites_y == 0 {
        return Err(JsValue::from_str("Invalid sprite dimensions"));
    }

    // Calculate the position of the requested frame
    let frame_x = frame_index % sprites_x;
    let frame_y = frame_index / sprites_x;

    if frame_y >= sprites_y {
        return Err(JsValue::from_str("Frame index out of bounds"));
    }

    // Extract the specific frame
    let mut sprite_data = Vec::new();
    for y in 0..sprite_height {
        for x in 0..sprite_width {
            let src_x = (frame_x * sprite_width + x) as usize;
            let src_y = (frame_y * sprite_height + y) as usize;

            if src_x < map_width as usize && src_y < map_height as usize {
                let pixel = rgba_img.get_pixel(src_x as u32, src_y as u32);
                let r = pixel[0] as u32;
                let g = pixel[1] as u32;
                let b = pixel[2] as u32;
                let a = pixel[3] as u32;
                sprite_data.push((a << 24) | (r << 16) | (g << 8) | b);
            } else {
                sprite_data.push(0); // Transparent pixel
            }
        }
    }

    // Return as JS array: [width, height, ...pixel_data]
    let result = js_sys::Array::new();
    result.push(&JsValue::from(sprite_width));
    result.push(&JsValue::from(sprite_height));
    for pixel in sprite_data {
        result.push(&JsValue::from(pixel));
    }

    Ok(result)
}

#[wasm_bindgen]
pub struct WasmGame {
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
    pixel_buffer: Vec<u32>,
    player: Snake,
    food: state::structs::Food,
    loot_crate: state::structs::LootCrate,
    sprites: SpriteMaps,
    score: u32,
    game_over: bool,
    last_frame_time: Option<f64>,
    // Game over animation variables
    game_over_frame: usize,
    game_over_darkness: f32,
    game_over_animation_time: f64,
    // Background parallax and animation variables
    stars_offset_x: usize,
    stars_sprite_frame_index: usize,
    stars_last_sprite_frame_update_time: f64,
    globe_sprite_frame_index: usize,
    globe_last_sprite_frame_update_time: f64,
    // Powerup system variables
    powerup_eligibility: bool,
    selected_powerup: Option<crate::state::core::perks::Perk>,
    last_loot_spawn_score: u32,
    food_score_value: u32,
    in_powerup_selection: bool,
    highlighted_powerup: Option<usize>,
    powerup_selection_keys: std::collections::HashMap<String, bool>,
    powerup_sound_played: bool,
    // Loot crate timer
    last_loot_crate_check_time: f64,
    // Game over crash sound state
    crash_sound_played: bool,
}

#[wasm_bindgen]
impl WasmGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WasmGame, JsValue> {
        let document = web_sys::window()
            .ok_or("No global window object")?
            .document()
            .ok_or("No document found")?;

        let canvas = document
            .create_element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;

        canvas.set_width(SCALED_WINDOW_WIDTH as u32);
        canvas.set_height(SCALED_WINDOW_HEIGHT as u32);
        canvas.set_id("game-canvas");

        let context = canvas
            .get_context("2d")?
            .ok_or("Failed to get 2d context")?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        // Create pixel buffer for scaled resolution
        let buffer_size = (SCALED_WINDOW_WIDTH * SCALED_WINDOW_HEIGHT) as usize;
        let pixel_buffer = vec![0xFF000000u32; buffer_size]; // Black background with full alpha

        // Initialize game state
        let player = Snake::new(40.0, 150.0, Direction::Right);
        let food = crate::state::structs::Food {
            position: crate::state::structs::Vector2D { x: 200.0, y: 200.0 },
            is_active: true,
            food_sprite_frame_index: 0,
            food_last_sprite_frame_index_update_time: 0.0,
        };

        // Start with empty sprites, actual loading happens separately
        let sprites = SpriteMaps {
            body: vec![],
            food: vec![],
            head: vec![],
            tail: vec![],
            game_over_screen: vec![],
            stars: vec![],
            planet: vec![],
            blue_strip: vec![],
            powerups: vec![],
            choose_powerup: vec![],
            loot_crate: vec![],
        };

        Ok(WasmGame {
            canvas,
            context,
            pixel_buffer,
            player,
            food,
            loot_crate: state::structs::LootCrate {
                position: state::structs::Vector2D { x: 0.0, y: 0.0 },
                is_active: false,
                sprite_frame_index: 0,
                last_sprite_frame_index_update_time: 0.0,
            },
            sprites,
            score: 0,
            game_over: false,
            last_frame_time: None,
            // Initialize game over animation
            game_over_frame: 0,
            game_over_darkness: 0.5,
            game_over_animation_time: 0.0,
            // Initialize background animation variables
            stars_offset_x: 0,
            stars_sprite_frame_index: 0,
            stars_last_sprite_frame_update_time: 0.0,
            globe_sprite_frame_index: 0,
            globe_last_sprite_frame_update_time: 0.0,
            // Initialize powerup system variables
            powerup_eligibility: false,
            selected_powerup: None,
            last_loot_spawn_score: 0,
            food_score_value: 100,
            in_powerup_selection: false,
            highlighted_powerup: None,
            powerup_selection_keys: std::collections::HashMap::new(),
            powerup_sound_played: false,
            // Initialize loot crate timer
            last_loot_crate_check_time: 0.0,
            // Initialize crash sound state
            crash_sound_played: false,
        })
    }

    #[wasm_bindgen]
    pub fn add_body_sprite(&mut self, width: u32, height: u32, data: Vec<u32>) -> Result<(), JsValue> {
        graphics::sprites::add_body_sprite(&mut self.sprites, width, height, data)
    }

    #[wasm_bindgen]
    pub fn add_head_sprite(&mut self, width: u32, height: u32, data: Vec<u32>) -> Result<(), JsValue> {
        graphics::sprites::add_head_sprite(&mut self.sprites, width, height, data)
    }

    #[wasm_bindgen]
    pub fn add_food_sprite(&mut self, width: u32, height: u32, data: Vec<u32>) -> Result<(), JsValue> {
        graphics::sprites::add_food_sprite(&mut self.sprites, width, height, data)?;
        web_sys::console::log_1(&format!("Food sprite added: {}x{} (frame {})", width, height, self.sprites.food.len() - 1).into());
        Ok(())
    }

    #[wasm_bindgen]
    pub fn add_tail_sprite(&mut self, width: u32, height: u32, data: Vec<u32>) -> Result<(), JsValue> {
        graphics::sprites::add_tail_sprite(&mut self.sprites, width, height, data)
    }

    #[wasm_bindgen]
    pub fn add_background_sprite(&mut self, width: u32, height: u32, data: Vec<u32>) -> Result<(), JsValue> {
        graphics::sprites::add_background_sprite(&mut self.sprites, width, height, data)
    }

    #[wasm_bindgen]
    pub fn add_globe_sprite(&mut self, width: u32, height: u32, data: Vec<u32>) -> Result<(), JsValue> {
        graphics::sprites::add_globe_sprite(&mut self.sprites, width, height, data)
    }

    #[wasm_bindgen]
    pub fn add_game_over_sprite(&mut self, width: u32, height: u32, data: Vec<u32>) -> Result<(), JsValue> {
        graphics::sprites::add_game_over_sprite(&mut self.sprites, width, height, data)
    }

    #[wasm_bindgen]
    pub fn add_powerup_sprite(&mut self, width: u32, height: u32, data: Vec<u32>) -> Result<(), JsValue> {
        graphics::sprites::add_powerup_sprite(&mut self.sprites, width, height, data)
    }

    #[wasm_bindgen]
    pub fn add_choose_powerup_sprite(&mut self, width: u32, height: u32, data: Vec<u32>) -> Result<(), JsValue> {
        graphics::sprites::add_choose_powerup_sprite(&mut self.sprites, width, height, data)
    }

    #[wasm_bindgen]
    pub fn add_loot_crate_sprite(&mut self, width: u32, height: u32, data: Vec<u32>) -> Result<(), JsValue> {
        graphics::sprites::add_loot_crate_sprite(&mut self.sprites, width, height, data)?;
        web_sys::console::log_1(&format!("Loot crate sprite added: {}x{} (frame {})", width, height, self.sprites.loot_crate.len() - 1).into());
        Ok(())
    }


    #[wasm_bindgen]
    pub fn get_canvas(&self) -> web_sys::HtmlCanvasElement {
        self.canvas.clone()
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) -> Result<(), JsValue> {
        if self.game_over {
            // Play crash sound once when game over starts
            if !self.crash_sound_played {
                self.play_crash_sound();
                self.crash_sound_played = true;
                web_sys::console::log_1(&"Game over - playing crash sound".into());
            }
            
            // Handle game over animation (will be paused until sound finishes)
            self.update_game_over_animation();
            self.render()?;
            return Ok(());
        }

        // Handle powerup selection
        if self.in_powerup_selection {
            self.handle_powerup_selection();
            self.render()?;
            return Ok(());
        }

        // Calculate delta time
        let current_time = js_sys::Date::now();
        let delta_time = if let Some(last_time) = self.last_frame_time {
            (current_time - last_time) / 1000.0 // Convert to seconds
        } else {
            0.016 // ~60 FPS fallback
        };
        self.last_frame_time = Some(current_time);

        // Update game logic
        self.update_game_logic(delta_time as f32)?;

        // Render the game
        self.render()?;

        Ok(())
    }

    fn update_game_logic(&mut self, delta_time: f32) -> Result<(), JsValue> {
        // Store previous values to detect state changes
        let previous_score = self.score;
        let previous_in_powerup_selection = self.in_powerup_selection;
        let _previous_game_over = self.game_over;
        
        let game_over = crate::state::core::tick::update_game_logic(
            &mut self.player,
            &mut self.food,
            &mut self.loot_crate,
            &mut self.score,
            self.food_score_value,
            &mut self.powerup_eligibility,
            &mut self.in_powerup_selection,
            &mut self.highlighted_powerup,
            &mut self.stars_offset_x,
            &mut self.stars_sprite_frame_index,
            &mut self.stars_last_sprite_frame_update_time,
            &mut self.globe_sprite_frame_index,
            &mut self.globe_last_sprite_frame_update_time,
            &mut self.last_loot_crate_check_time,
            delta_time,
        )?;

        // Check if food was eaten (score increased)
        if self.score > previous_score {
            self.play_eat_sound();
        }



        // Check if powerup selection just started - pause music and play powerup sound
        if !previous_in_powerup_selection && self.in_powerup_selection {
            web_sys::console::log_1(&"Powerup selection started, pausing music".into());
            self.pause_music();
            self.play_new_powerup_sound();
        }

        // Check if powerup selection just ended - resume music
        if previous_in_powerup_selection && !self.in_powerup_selection {
            web_sys::console::log_1(&"Powerup selection ended, resuming music".into());
            self.resume_music();
        }

        // Check if game just ended
        if game_over {
            self.game_over = true;
            // Stop music immediately when game over occurs
            self.stop_music();
        }

        Ok(())
    }


    fn render(&mut self) -> Result<(), JsValue> {
        // Clear pixel buffer
        self.pixel_buffer.fill(0xFF000000); // Black background

        // Create a temporary art-resolution buffer for rendering
        let mut art_buffer = vec![0xFF000000u32; ART_WIDTH * ART_HEIGHT];

        if self.game_over {
            // Draw game over screen
            graphics::update::draw_game_over_screen(
                &mut art_buffer,
                &self.sprites,
                self.game_over_frame,
                self.game_over_darkness,
                self.score,
            );
        } else if self.in_powerup_selection {
            // Draw powerup selection screen
            graphics::update::draw_powerup_selection_screen(
                &mut art_buffer,
                &self.sprites,
                self.highlighted_powerup,
            );
        } else {
            // Draw background with parallax effect
            graphics::update::draw_parallax_background(
                &mut art_buffer,
                &self.sprites,
                self.stars_offset_x,
                self.stars_sprite_frame_index,
                self.globe_sprite_frame_index,
            );

            // Draw food
            graphics::update::draw_food(&mut art_buffer, &self.food, &self.sprites);

            // Draw loot crate if active
            graphics::update::draw_loot_crate(&mut art_buffer, &self.loot_crate, &self.sprites);


            // Draw snake
            graphics::update::draw_snake(&mut art_buffer, &self.player, &self.sprites);

            // Draw score text BEFORE scaling (only in normal game mode)
            graphics::update::draw_score_text(&mut art_buffer, self.score);
        }

        // Scale the art buffer to the screen buffer
        graphics::render::scale_buffer_to_screen(&art_buffer, &mut self.pixel_buffer);

        // Convert pixel buffer to ImageData and draw to canvas
        graphics::render::update_canvas(&self.pixel_buffer, &self.context)?;

        Ok(())
    }

    #[wasm_bindgen]
    pub fn handle_key_down(&mut self, key_code: &str) {
        if self.game_over {
            // Allow restarting the game with Space key
            if input::handler::handle_game_over_input(key_code) {
                self.restart_game();
            }
            return;
        }

        input::handler::handle_key_down(
            key_code,
            &mut self.player.direction,
            self.game_over,
            self.in_powerup_selection,
            &mut self.powerup_selection_keys,
        );
    }

    #[wasm_bindgen]
    pub fn play_eat_sound(&self) {
        let js_code = "if (window.playSound) { window.playSound('eat'); }";
        js_sys::eval(js_code).unwrap_or_else(|_| wasm_bindgen::JsValue::UNDEFINED);
    }

    #[wasm_bindgen]
    pub fn play_new_powerup_sound(&self) {
        let js_code = "if (window.playSound) { window.playSound('divine_intervention'); }";
        js_sys::eval(js_code).unwrap_or_else(|_| wasm_bindgen::JsValue::UNDEFINED);
    }

    #[wasm_bindgen]
    pub fn play_powerup_sound(&self, sound_name: &str) {
        let js_code = format!("if (window.playPowerupSound) {{ window.playPowerupSound('{}'); }}", sound_name);
        js_sys::eval(&js_code).unwrap_or_else(|_| wasm_bindgen::JsValue::UNDEFINED);
    }

    #[wasm_bindgen]
    pub fn play_crash_sound(&self) {
        let js_code = "if (window.playCrashSound) { window.playCrashSound(); }";
        js_sys::eval(js_code).unwrap_or_else(|_| wasm_bindgen::JsValue::UNDEFINED);
    }


    #[wasm_bindgen]
    pub fn stop_music(&self) {
        let js_code = "if (window.stopMusic) { window.stopMusic(); }";
        js_sys::eval(js_code).unwrap_or_else(|_| wasm_bindgen::JsValue::UNDEFINED);
    }

    #[wasm_bindgen]
    pub fn pause_music(&self) {
        let js_code = "if (window.pauseMusic) { window.pauseMusic(); }";
        js_sys::eval(js_code).unwrap_or_else(|_| wasm_bindgen::JsValue::UNDEFINED);
    }

    #[wasm_bindgen]
    pub fn resume_music(&self) {
        let js_code = "if (window.resumeMusic) { window.resumeMusic(); }";
        js_sys::eval(js_code).unwrap_or_else(|_| wasm_bindgen::JsValue::UNDEFINED);
    }

    fn restart_game(&mut self) {
        // Stop any playing music and restart background music
        self.stop_music();
        
        state::core::tick::restart_game(
            &mut self.player,
            &mut self.food,
            &mut self.loot_crate,
            &mut self.score,
            &mut self.game_over,
            &mut self.last_frame_time,
            &mut self.stars_offset_x,
            &mut self.stars_sprite_frame_index,
            &mut self.stars_last_sprite_frame_update_time,
            &mut self.globe_sprite_frame_index,
            &mut self.globe_last_sprite_frame_update_time,
            &mut self.game_over_frame,
            &mut self.game_over_darkness,
            &mut self.game_over_animation_time,
            &mut self.last_loot_spawn_score,
            &mut self.powerup_eligibility,
            &mut self.selected_powerup,
            &mut self.food_score_value,
            &mut self.in_powerup_selection,
            &mut self.highlighted_powerup,
            &mut self.powerup_selection_keys,
            &mut self.last_loot_crate_check_time,
        );
        
        // Reset crash sound state
        self.crash_sound_played = false;
        
        // Resume background music after restart
        self.resume_music();
    }

    fn handle_powerup_selection(&mut self) {
        if state::core::perks::handle_powerup_selection(
            &mut self.powerup_selection_keys,
            &mut self.highlighted_powerup,
            &mut self.selected_powerup,
            &mut self.powerup_eligibility,
            &mut self.in_powerup_selection,
        ) {
            // A powerup was selected, apply its effect
            if let Some(ref powerup) = self.selected_powerup {
                state::core::perks::apply_powerup_effect(powerup, &mut self.player.move_interval, &mut self.food_score_value);
                
                // Play special sound for each powerup
                match powerup {
                    state::core::perks::Perk::HungryWorm => {
                        web_sys::console::log_1(&"Hungry Worm selected, playing apple sound".into());
                        self.play_powerup_sound("apple"); // This will resume music when sound ends
                    }
                    state::core::perks::Perk::NeedForSpeed => {
                        web_sys::console::log_1(&"Need 4 Speed selected, playing turbo sound".into());
                        self.play_powerup_sound("turbo"); // This will resume music when sound ends
                    }
                }
            } else {
                // No powerup selected, resume music
                web_sys::console::log_1(&"No powerup selected, resuming music".into());
                self.resume_music();
            }
        }
    }


    fn update_game_over_animation(&mut self) {
        if state::core::tick::update_game_over_animation(
            &mut self.game_over_frame,
            &mut self.game_over_darkness,
            &mut self.game_over_animation_time,
        ) {
            self.restart_game();
        }
    }




}