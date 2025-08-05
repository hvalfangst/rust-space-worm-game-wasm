use crate::state::constants::graphics::{SNAKE_BODY_HEIGHT, SNAKE_BODY_WIDTH};

#[derive(Debug, Clone, Copy)]
pub struct PlatformInstant(f64);

#[derive(Debug, Clone, Copy)]
pub struct PlatformDuration(f64);

impl PlatformInstant {
    pub fn now() -> Self {
        PlatformInstant(js_sys::Date::now())
    }

    pub fn duration_since(&self, earlier: PlatformInstant) -> PlatformDuration {
        PlatformDuration(self.0 - earlier.0)
    }

    pub fn elapsed(&self) -> PlatformDuration {
        PlatformDuration(js_sys::Date::now() - self.0)
    }
}

impl PlatformDuration {
    pub fn as_secs_f32(&self) -> f32 {
        self.0 as f32 / 1000.0 // Convert milliseconds to seconds
    }
    
    pub fn as_millis(&self) -> u64 {
        self.0 as u64 // Value is already in milliseconds
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub struct Snake {
    pub direction: Direction,
    pub body: Vec<Vector2D>,
    pub move_timer: f32,
    pub move_interval: f32,
    pub body_sprite_frame_index: usize,
    pub body_last_sprite_frame_index_update_time: f64,
    pub head_sprite_frame_index: usize,
    pub head_last_sprite_frame_index_update_time: f64,
    pub food_near: bool,
}

impl Snake {
    pub fn new(x: f32, y: f32, initial_direction: Direction) -> Self {

        let body = match initial_direction {
            Direction::Right => vec![
                Vector2D { x, y },
                Vector2D { x: x - SNAKE_BODY_WIDTH * 2.0, y },
                Vector2D { x: x - SNAKE_BODY_WIDTH * 3.0, y },
            ],
            Direction::Left => vec![
                Vector2D { x, y },
                Vector2D { x: x + SNAKE_BODY_WIDTH * 2.0, y },
                Vector2D { x: x + SNAKE_BODY_WIDTH * 3.0, y },
            ],
            Direction::Down => vec![
                Vector2D { x, y },
                Vector2D { x, y: y - SNAKE_BODY_HEIGHT * 2.0 },
                Vector2D { x, y: y - SNAKE_BODY_HEIGHT * 3.0 },
            ],
            Direction::Up => vec![
                Vector2D { x, y },
                Vector2D { x, y: y + SNAKE_BODY_HEIGHT * 2.0 },
                Vector2D { x, y: y + SNAKE_BODY_HEIGHT * 3.0 },
            ],
        };

        Snake {
            direction: initial_direction,
            body,
            move_timer: 0.0,
            move_interval: 0.1, // Default is 10 moves per second
            body_sprite_frame_index: 0,
            body_last_sprite_frame_index_update_time: 0.0,
            head_sprite_frame_index: 0,
            head_last_sprite_frame_index_update_time: 0.0,
            food_near: false,
        }
    }
}

pub struct Food {
    pub position: Vector2D,
    pub is_active: bool,
    pub food_sprite_frame_index: usize,
    pub food_last_sprite_frame_index_update_time: f64,
}

pub struct LootCrate {
    pub position: Vector2D,
    pub is_active: bool,
    pub sprite_frame_index: usize,
    pub last_sprite_frame_index_update_time: f64,
}


pub struct GameState {
    pub player: Snake,
    pub food: Food,
    pub loot_crate: LootCrate,
    pub delta_time: f32,
    pub last_frame_time: Option<PlatformInstant>,
    pub game_over: bool,
    pub score: u32,
    pub food_score_value: u32,
    pub last_loot_spawn_score: u32,
    pub powerup_eligibility: bool,
    pub selected_powerup: Option<crate::state::core::perks::Perk>,
    pub powerup_history: std::collections::HashMap<u32, crate::state::core::perks::Perk>,
}

impl GameState {
    pub fn new(player: Snake) -> Self {
        GameState {
            player,
            food: Food {
                position: Vector2D { x: 100.0, y: 100.0 },
                is_active: false,
                food_sprite_frame_index: 0,
                food_last_sprite_frame_index_update_time: 0.0,
            },
            loot_crate: LootCrate {
                position: Vector2D { x: 0.0, y: 0.0 },
                is_active: false,
                sprite_frame_index: 0,
                last_sprite_frame_index_update_time: 0.0,
            },
            delta_time: 0.0,
            last_frame_time: None,
            game_over: false,
            score: 0,
            food_score_value: 100,
            last_loot_spawn_score: 0,
            powerup_eligibility: false,
            selected_powerup: None,
            powerup_history: std::collections::HashMap::new(),
        }
    }

    pub fn restart_level(&mut self) {
        self.player = Snake::new(40.0, 150.0, Direction::Right);
        self.food = Food {
            position: Vector2D { x: 100.0, y: 100.0 },
            is_active: false,
            food_sprite_frame_index: 0,
            food_last_sprite_frame_index_update_time: 0.0,
        };
        self.loot_crate = LootCrate {
            position: Vector2D { x: 0.0, y: 0.0 },
            is_active: false,
            sprite_frame_index: 0,
            last_sprite_frame_index_update_time: 0.0,
        };
        self.score = 0;
        self.game_over = false;
        self.last_loot_spawn_score = 0;
        self.powerup_eligibility = false;
        self.selected_powerup = None;
        self.powerup_history.clear();
    }
}
