use crate::graphics::update::draw_game_over_screen;
use crate::state::core::CoreLogic;
use crate::state::structs::GameState;
use crate::graphics::render::render_pixel_buffer;

pub struct CheckGameOver;

impl CoreLogic for CheckGameOver {
    fn execute(&self, game_state: &mut GameState) {

        if game_state.game_over {
            let mut frame = 0;
            let mut darkness_factor = Some(0.5); // Initial darkness factor

            while frame < 8 {
                draw_game_over_screen(game_state, frame, darkness_factor);
                render_pixel_buffer(game_state);

                // Sleep for 200 ms
                std::thread::sleep(std::time::Duration::from_millis(200));

                frame += 1;
                darkness_factor = darkness_factor.map(|d| (d + 0.1).min(0.8)); // Increase darkness by 10%, cap at 0.8
            }
            game_state.restart_level();
        }
        }
}