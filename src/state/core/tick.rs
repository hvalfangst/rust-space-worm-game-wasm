use crate::state::core::CoreLogic;
use crate::state::structs::GameState;
use std::time::Instant;

pub struct UpdateDeltaTime;

impl CoreLogic for UpdateDeltaTime {
    fn execute(&self, game_state: &mut GameState) {
        let current_time = Instant::now();

        game_state.delta_time = if let Some(last_time) = game_state.last_frame_time {
            current_time.duration_since(last_time).as_secs_f32()
        } else {
            1.0 / 60.0
        };

        game_state.last_frame_time = Some(current_time);
    }
}