use minifb::{Key, KeyRepeat};
use crate::audio::manager::SfxId;
use crate::graphics::render::render_pixel_buffer;
use crate::graphics::update::draw_choose_perk_screen_with_highlight;
use crate::state::constants::audio::NEW_PERK_FILE;
use crate::state::core::CoreLogic;
use crate::state::structs::Perk;

pub struct CheckNewPerk;



impl CoreLogic for CheckNewPerk {
    fn execute(&self, game_state: &mut crate::state::structs::GameState) {
        if game_state.perk_eligibility {

            // Play new perk music, stop and disable any existing music
            if game_state.audio_manager.is_music_playing() {
                game_state.audio_manager.stop_music();
            }

            game_state.music_disabled = true;

            game_state.audio_manager.play_sfx(SfxId::NewPerk)
                .expect("Failed to play new perk sound effect");


            game_state.selected_perk = None;
            let mut highlighted_perk: Option<usize> = None;
            let mut perk_selected = false;

            let key_perk_map = [
                (Key::A, -1), // Move left
                (Key::D, 1),  // Move right
            ];

            loop {

                for (key, direction) in key_perk_map.iter() {

                    // Defaults to the first perk if escape is pressed
                    if game_state.window.is_key_pressed(Key::Escape, KeyRepeat::No) {
                        game_state.selected_perk = Some(0);
                        perk_selected = true;
                    }

                    // Keys A and D will map to an index used to highlight and ultimately decide perk
                    if game_state.window.is_key_down(*key) {
                        if let Some(current) = highlighted_perk {
                            let new_perk = (current as isize + direction).clamp(1, 2) as usize;
                            highlighted_perk = Some(new_perk);
                        } else {
                            highlighted_perk = Some(1); // Default to the first perk if none is highlighted
                        }
                    }

                    // Lock in choice, with default being the first perk as is the case for escape
                    if game_state.window.is_key_pressed(Key::Space, KeyRepeat::No) {
                        if let Some(perk) = highlighted_perk {
                            game_state.selected_perk = Some(perk);
                            perk_selected = true;
                            break;
                        }
                    }
                }

                draw_choose_perk_screen_with_highlight(game_state, highlighted_perk);
                render_pixel_buffer(game_state);


                if perk_selected {
                    game_state.perk_eligibility = false;

                    match game_state.selected_perk {
                        Some(1) => {
                            game_state.player.move_interval *= 0.8;
                            game_state.perk_history.insert(game_state.score, Perk::SpeedBoost);
                        },
                        Some(2) => {
                             game_state.food_score_value *= 2;
                            game_state.perk_history.insert(game_state.score, Perk::DoubleScore);
                        }
                        _ => {}
                    }

                    std::thread::sleep(std::time::Duration::from_millis(200));
                    break;
                }
            }
        }
    }
}