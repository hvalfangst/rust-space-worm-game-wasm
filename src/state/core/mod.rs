pub mod movement;
pub mod bounds;
pub mod food;
pub mod tick;
pub mod termination;
pub mod collision;
mod background;
mod perks;
pub mod snake;

use crate::state::structs::GameState;
use rodio::Sink;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub trait CoreLogic {
    fn execute(&self, game_state: &mut GameState);
}

pub fn execute_core_logic(game_state: &mut GameState, core_logic_operations: &HashMap<String, Rc<RefCell<dyn CoreLogic>>>) {
    for (_, core_logic_operation) in core_logic_operations.iter() {
        core_logic_operation.borrow().execute(game_state);
    }
}

pub fn initialize_core_logic_map() -> HashMap<String, Rc<RefCell<dyn CoreLogic>>> {
    let mut logic_map: HashMap<String, Rc<RefCell<dyn CoreLogic>>> = HashMap::new();

    // Game state updates
    logic_map.insert("UpdateDeltaTime".to_string(), Rc::new(RefCell::new(tick::UpdateDeltaTime)));

    // Movement
    logic_map.insert("ModifyCoordinatesOfBodyParts".to_string(), Rc::new(RefCell::new(movement::ModifyCoordinatesOfBodyParts)));

    // Bounds checking
    logic_map.insert("VerticalBounds".to_string(), Rc::new(RefCell::new(bounds::VerticalBounds)));
    logic_map.insert("HorizontalBounds".to_string(), Rc::new(RefCell::new(bounds::HorizontalBounds)));

    // Food system
    logic_map.insert("SpawnFood".to_string(), Rc::new(RefCell::new(food::SpawnFood)));
    logic_map.insert("CheckIfFoodWasEaten".to_string(), Rc::new(RefCell::new(food::CheckIfFoodWasEaten)));
    logic_map.insert("AlternateBetweenFoodSpriteFrames".to_string(), Rc::new(RefCell::new(food::AlternateBetweenFoodSpriteFrames)));

    // Collision detection
    logic_map.insert("CheckSelfCollision".to_string(), Rc::new(RefCell::new(collision::CheckSelfCollision)));

    // Snake sprite logic
    logic_map.insert("AlternateBodySpriteFrameIndex".to_string(), Rc::new(RefCell::new(snake::AlternateBodySpriteFrameIndex)));
    logic_map.insert("AlternateHeadSpriteFrameIndex".to_string(), Rc::new(RefCell::new(snake::AlternateHeadSpriteFrameIndex)));

    // Background sprite frames
    logic_map.insert("AlternateBackgroundSpriteFrame".to_string(), Rc::new(RefCell::new(background::AlternateGlobeSpriteFrame)));
    logic_map.insert("AlternateStarsSpriteFrame".to_string(), Rc::new(RefCell::new(background::AlternateStarsSpriteFrame)));

    // Game over logic
    logic_map.insert("CheckGameOver".to_string(), Rc::new(RefCell::new(termination::CheckGameOver)));

    // Perks
    logic_map.insert("CheckNewPerk".to_string(), Rc::new(RefCell::new(perks::CheckNewPerk)));

    logic_map
}