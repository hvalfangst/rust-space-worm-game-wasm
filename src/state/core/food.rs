use std::time::Instant;
use crate::state::core::CoreLogic;
use crate::state::constants::physics::{LOWER_BOUND_X, LOWER_BOUND_Y, UPPER_BOUND_X, UPPER_BOUND_Y};
use crate::state::structs::{Direction, Food, GameState, Vector2D};
use rand::Rng;
use crate::audio::manager::SfxId;
use crate::state::constants::graphics::{SNAKE_BODY_HEIGHT, SNAKE_BODY_WIDTH};

pub struct SpawnFood;

impl CoreLogic for SpawnFood {
    fn execute(&self, game_state: &mut GameState) {
        if game_state.food.is_active {
            return;
        }

        game_state.food = Food {
            position: Vector2D {
                x: rand::rng().random_range(LOWER_BOUND_X + 10.0..UPPER_BOUND_X - 10.0),
                y: rand::rng().random_range(LOWER_BOUND_Y + 10.0..UPPER_BOUND_Y - 10.0)
            },
            is_active: true,
            food_sprite_frame_index: 0,
            food_last_sprite_frame_index_update_time: Instant::now(),
        };
    }
}

pub struct CheckIfFoodWasEaten;

impl CoreLogic for CheckIfFoodWasEaten {
    fn execute(&self, game_state: &mut GameState) {
        if !game_state.food.is_active {
            return;
        }

        let head_position = &game_state.player.body[0];
        let food_position = &game_state.food.position;

        if (head_position.x - food_position.x).abs() < 24.0 && (head_position.y - food_position.y).abs() < 24.0 {
            game_state.player.food_near = true;

            if (head_position.x - food_position.x).abs() < 12.0 && (head_position.y - food_position.y).abs() < 12.0 {
                game_state.food.is_active = false;
                game_state.score += game_state.food_score_value;

                // Check if one is eligible for a perk based on the score
                if game_state.score % game_state.perk_required_score == 0 {
                    game_state.perk_eligibility = true;
                }

                let tail_position = game_state.player.body.last().unwrap();

                // Add a new segment to the snake's body at the tail position based on the current direction
                let new_segment = match game_state.player.direction {
                    Direction::Left => Vector2D {
                        x: tail_position.x + SNAKE_BODY_WIDTH,
                        y: tail_position.y,
                    },
                    Direction::Right => Vector2D {
                        x: tail_position.x - SNAKE_BODY_WIDTH,
                        y: tail_position.y,
                    },
                    Direction::Up => Vector2D {
                        x: tail_position.x,
                        y: tail_position.y + SNAKE_BODY_HEIGHT,
                    },
                    Direction::Down => Vector2D {
                        x: tail_position.x,
                        y: tail_position.y - SNAKE_BODY_HEIGHT,
                    },
                };

                game_state.player.body.push(new_segment);

                // Play sound effect for eating food
                game_state.audio_manager.play_sfx(SfxId::Eat)
                    .expect("Failed to play eat food sound effect");
            }
        } else {
            game_state.player.food_near = false;
        }
    }
}

pub struct AlternateBetweenFoodSpriteFrames;

impl CoreLogic for AlternateBetweenFoodSpriteFrames {
    fn execute(&self, game_state: &mut GameState) {
        if !game_state.food.is_active {
            return;
        }

        if game_state.food.food_last_sprite_frame_index_update_time.elapsed().as_millis() >= 500 {
            game_state.food.food_sprite_frame_index = 1 - game_state.food.food_sprite_frame_index;
            game_state.food.food_last_sprite_frame_index_update_time = Instant::now();
        }
    }
}

