use crate::state::constants::graphics::{SNAKE_BODY_HEIGHT, SNAKE_BODY_WIDTH};
use crate::state::constants::physics::{LOWER_BOUND_X, LOWER_BOUND_Y, UPPER_BOUND_X, UPPER_BOUND_Y};
use crate::state::core::CoreLogic;
use crate::state::structs::GameState;

pub struct VerticalBounds;

impl CoreLogic for VerticalBounds {
    fn execute(&self, game_state: &mut GameState) {
        let head = &mut game_state.player.body[0];

        // Wrap vertically with sprite height consideration
        if head.y < LOWER_BOUND_Y {
            head.y = UPPER_BOUND_Y - SNAKE_BODY_HEIGHT; // Appear at top
        } else if head.y > UPPER_BOUND_Y {
            head.y = LOWER_BOUND_Y + SNAKE_BODY_HEIGHT; // Appear at bottom
        }
    }
}

pub struct HorizontalBounds;

impl CoreLogic for HorizontalBounds {
    fn execute(&self, game_state: &mut GameState) {
        let head = &mut game_state.player.body[0];

        // Wrap horizontally with sprite width consideration
        if head.x < LOWER_BOUND_X {
            head.x = UPPER_BOUND_X - SNAKE_BODY_WIDTH; // Appear at right
        } else if head.x > UPPER_BOUND_X {
            head.x = LOWER_BOUND_X + SNAKE_BODY_WIDTH; // Appear at left
        }
    }
}