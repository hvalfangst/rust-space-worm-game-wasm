use image::GenericImageView;
use wasm_bindgen::prelude::*;

mod state;
mod graphics;
mod input;
mod audio;
mod platform;

use crate::graphics::sprites::SpriteMaps;
use crate::state::constants::graphics::{ART_HEIGHT, ART_WIDTH, SCALED_WINDOW_HEIGHT, SCALED_WINDOW_WIDTH};
use crate::state::constants::state::SCORE_PERK_THRESHOLD_LEVEL_1;
use crate::state::structs::{Direction, Snake};


// Set up console error panic hook for better debugging
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub async fn load_sprite_from_url(sprite_path: &str, sprite_width: u32, sprite_height: u32) -> Result<js_sys::Array, JsValue> {
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

    // Extract just the first sprite from the sprite sheet
    let sprites_x = map_width / sprite_width;
    let sprites_y = map_height / sprite_height;

    if sprites_x == 0 || sprites_y == 0 {
        return Err(JsValue::from_str("Invalid sprite dimensions"));
    }

    // Extract the first sprite (top-left)
    let mut sprite_data = Vec::new();
    for y in 0..sprite_height {
        for x in 0..sprite_width {
            let src_x = x as usize;
            let src_y = y as usize;

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
    // Perk system variables
    perk_eligibility: bool,
    selected_perk: Option<crate::state::core::perks::Perk>,
    current_threshold: Option<u32>,
    perk_required_score: u32,
    food_score_value: u32,
    in_perk_selection: bool,
    highlighted_perk: Option<usize>,
    perk_selection_keys: std::collections::HashMap<String, bool>,
    perk_sound_played: bool,
    granted_perks: Vec<u32>,
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
            perks: vec![],
            choose_perk: vec![],
        };

        Ok(WasmGame {
            canvas,
            context,
            pixel_buffer,
            player,
            food,
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
            // Initialize perk system variables
            perk_eligibility: false,
            selected_perk: None,
            current_threshold: None,
            perk_required_score: SCORE_PERK_THRESHOLD_LEVEL_1,
            food_score_value: 100,
            in_perk_selection: false,
            highlighted_perk: None,
            perk_selection_keys: std::collections::HashMap::new(),
            perk_sound_played: false,
            granted_perks: vec![],
        })
    }

    #[wasm_bindgen]
    pub fn add_body_sprite(&mut self, width: u32, height: u32, data: Vec<u32>) -> Result<(), JsValue> {
        crate::graphics::sprites::add_body_sprite(&mut self.sprites, width, height, data)
    }

    #[wasm_bindgen]
    pub fn add_head_sprite(&mut self, width: u32, height: u32, data: Vec<u32>) -> Result<(), JsValue> {
        crate::graphics::sprites::add_head_sprite(&mut self.sprites, width, height, data)
    }

    #[wasm_bindgen]
    pub fn add_food_sprite(&mut self, width: u32, height: u32, data: Vec<u32>) -> Result<(), JsValue> {
        crate::graphics::sprites::add_food_sprite(&mut self.sprites, width, height, data)?;
        web_sys::console::log_1(&format!("Food sprite added: {}x{} (frame {})", width, height, self.sprites.food.len() - 1).into());
        Ok(())
    }

    #[wasm_bindgen]
    pub fn add_tail_sprite(&mut self, width: u32, height: u32, data: Vec<u32>) -> Result<(), JsValue> {
        crate::graphics::sprites::add_tail_sprite(&mut self.sprites, width, height, data)
    }

    #[wasm_bindgen]
    pub fn add_background_sprite(&mut self, width: u32, height: u32, data: Vec<u32>) -> Result<(), JsValue> {
        crate::graphics::sprites::add_background_sprite(&mut self.sprites, width, height, data)
    }

    #[wasm_bindgen]
    pub fn add_globe_sprite(&mut self, width: u32, height: u32, data: Vec<u32>) -> Result<(), JsValue> {
        crate::graphics::sprites::add_globe_sprite(&mut self.sprites, width, height, data)
    }

    #[wasm_bindgen]
    pub fn add_game_over_sprite(&mut self, width: u32, height: u32, data: Vec<u32>) -> Result<(), JsValue> {
        crate::graphics::sprites::add_game_over_sprite(&mut self.sprites, width, height, data)
    }

    #[wasm_bindgen]
    pub fn add_perk_sprite(&mut self, width: u32, height: u32, data: Vec<u32>) -> Result<(), JsValue> {
        crate::graphics::sprites::add_perk_sprite(&mut self.sprites, width, height, data)
    }

    #[wasm_bindgen]
    pub fn add_choose_perk_sprite(&mut self, width: u32, height: u32, data: Vec<u32>) -> Result<(), JsValue> {
        crate::graphics::sprites::add_choose_perk_sprite(&mut self.sprites, width, height, data)
    }


    #[wasm_bindgen]
    pub fn get_canvas(&self) -> web_sys::HtmlCanvasElement {
        self.canvas.clone()
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) -> Result<(), JsValue> {
        if self.game_over {
            // Handle game over animation
            self.update_game_over_animation();
            self.render()?;
            return Ok(());
        }

        // Handle perk selection
        if self.in_perk_selection {
            self.handle_perk_selection();
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
        let previous_in_perk_selection = self.in_perk_selection;
        let previous_game_over = self.game_over;
        
        let game_over = crate::state::core::tick::update_game_logic(
            &mut self.player,
            &mut self.food,
            &mut self.score,
            self.food_score_value,
            &mut self.perk_eligibility,
            &mut self.in_perk_selection,
            &mut self.highlighted_perk,
            self.perk_required_score,
            &mut self.stars_offset_x,
            &mut self.stars_sprite_frame_index,
            &mut self.stars_last_sprite_frame_update_time,
            &mut self.globe_sprite_frame_index,
            &mut self.globe_last_sprite_frame_update_time,
            delta_time,
            &mut self.granted_perks,
            &mut self.current_threshold,
        )?;

        // Check if food was eaten (score increased)
        if self.score > previous_score {
            self.play_eat_sound();
        }



        // Check if perk selection just started - pause music and play perk sound
        if !previous_in_perk_selection && self.in_perk_selection {
            self.pause_music();
            self.play_new_perk_sound();
        }

        // Check if perk selection just ended - resume music
        if previous_in_perk_selection && !self.in_perk_selection {
            self.resume_music();
        }

        // Check if game just ended
        if game_over {
            self.game_over = true;
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
            crate::graphics::update::draw_game_over_screen(
                &mut art_buffer,
                &self.sprites,
                self.game_over_frame,
                self.game_over_darkness,
                self.score,
            );
        } else if self.in_perk_selection {
            // Draw perk selection screen
            crate::graphics::update::draw_perk_selection_screen(
                &mut art_buffer,
                &self.sprites,
                self.highlighted_perk,
                self.current_threshold.unwrap_or(1000),
            );
        } else {
            // Draw background with parallax effect
            crate::graphics::update::draw_parallax_background(
                &mut art_buffer,
                &self.sprites,
                self.stars_offset_x,
                self.stars_sprite_frame_index,
                self.globe_sprite_frame_index,
            );

            // Draw food
            crate::graphics::update::draw_food(&mut art_buffer, &self.food, &self.sprites);

            // Draw snake
            crate::graphics::update::draw_snake(&mut art_buffer, &self.player, &self.sprites);

            // Draw score text BEFORE scaling (only in normal game mode)
            crate::graphics::update::draw_score_text(&mut art_buffer, self.score);
        }

        // Scale the art buffer to the screen buffer
        crate::graphics::render::scale_buffer_to_screen(&art_buffer, &mut self.pixel_buffer);

        // Convert pixel buffer to ImageData and draw to canvas
        crate::graphics::render::update_canvas(&self.pixel_buffer, &self.context)?;

        Ok(())
    }

    #[wasm_bindgen]
    pub fn handle_key_down(&mut self, key_code: &str) {
        if self.game_over {
            // Allow restarting the game with Space key
            if crate::input::handler::handle_game_over_input(key_code) {
                self.restart_game();
            }
            return;
        }

        crate::input::handler::handle_key_down(
            key_code,
            &mut self.player.direction,
            self.game_over,
            self.in_perk_selection,
            &mut self.perk_selection_keys,
        );
    }

    #[wasm_bindgen]
    pub fn play_eat_sound(&self) {
        let js_code = "if (window.playSound) { window.playSound('eat'); }";
        js_sys::eval(js_code).unwrap_or_else(|_| wasm_bindgen::JsValue::UNDEFINED);
    }

    #[wasm_bindgen]
    pub fn play_new_perk_sound(&self) {
        let js_code = "if (window.playSound) { window.playSound('new_perk'); }";
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
        
        crate::state::core::tick::restart_game(
            &mut self.player,
            &mut self.food,
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
            &mut self.perk_eligibility,
            &mut self.selected_perk,
            &mut self.food_score_value,
            &mut self.in_perk_selection,
            &mut self.highlighted_perk,
            &mut self.perk_selection_keys,
        );
        
        // Resume background music after restart
        self.resume_music();
    }

    fn handle_perk_selection(&mut self) {
        if crate::state::core::perks::handle_perk_selection(
            &mut self.perk_selection_keys,
            &mut self.highlighted_perk,
            &mut self.selected_perk,
            &mut self.perk_eligibility,
            &mut self.in_perk_selection,
            self.current_threshold.unwrap_or(1000),
        ) {
            // A perk was selected, apply its effect
            if let Some(ref perk) = self.selected_perk {
                crate::state::core::perks::apply_perk_effect(perk, &mut self.player.move_interval, &mut self.food_score_value);
            }
        }
    }


    fn update_game_over_animation(&mut self) {
        if crate::state::core::tick::update_game_over_animation(
            &mut self.game_over_frame,
            &mut self.game_over_darkness,
            &mut self.game_over_animation_time,
        ) {
            self.restart_game();
        }
    }




}