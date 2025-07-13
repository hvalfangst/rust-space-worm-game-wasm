use crate::state::core::CoreLogic;
use crate::state::structs::GameState;

pub struct AlternateGlobeSpriteFrame;

impl CoreLogic for AlternateGlobeSpriteFrame {
    fn execute(&self, game_state: &mut GameState) {
        if game_state.globe_last_sprite_frame_update_time.elapsed().as_millis() >= 1000 {
            game_state.globe_sprite_frame_index = (game_state.globe_sprite_frame_index + 1) % 6   ;
            game_state.globe_last_sprite_frame_update_time = std::time::Instant::now();
        }
    }
}

pub struct AlternateStarsSpriteFrame;

impl CoreLogic for AlternateStarsSpriteFrame {
    fn execute(&self, game_state: &mut GameState) {
        if game_state.stars_last_sprite_frame_update_time.elapsed().as_millis() >= 250 {
            game_state.stars_sprite_frame_index = (game_state.stars_sprite_frame_index + 1) % 6;
            game_state.stars_last_sprite_frame_update_time = std::time::Instant::now();
        }
    }
}



