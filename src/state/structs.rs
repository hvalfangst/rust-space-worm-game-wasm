use crate::audio::manager::AudioManager;
use crate::graphics::sprites::SpriteMaps;
use minifb::Window;
use std::collections::HashMap;
use std::time::Instant;
use crate::state::constants::graphics::{SNAKE_BODY_HEIGHT, SNAKE_BODY_WIDTH};


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
    pub body_last_sprite_frame_index_update_time: Instant,
    pub head_sprite_frame_index: usize,
    pub head_last_sprite_frame_index_update_time: Instant,
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
            body_last_sprite_frame_index_update_time: Instant::now(),
            head_sprite_frame_index: 0,
            head_last_sprite_frame_index_update_time: Instant::now(),
            food_near: false,
        }
    }
}

pub struct Food {
    pub position: Vector2D,
    pub is_active: bool,
    pub food_sprite_frame_index: usize,
    pub food_last_sprite_frame_index_update_time: Instant,
}

pub enum Perk {
    SpeedBoost,
    DoubleScore,
}

pub struct GameState<'a> {
    pub player: Snake,
    pub sprites: SpriteMaps,
    pub window_buffer: &'a mut Vec<u32>,
    pub window_width: usize,
    pub window_height: usize,
    pub window: &'a mut Window,
    pub scaled_buffer: &'a mut Vec<u32>,
    pub food: Food,
    pub delta_time: f32,
    pub last_frame_time: Option<Instant>,
    pub game_over: bool,
    pub score: u32,
    pub globe_sprite_frame_index: usize,
    pub globe_last_sprite_frame_update_time: Instant,
    pub stars_offset_x: usize,
    pub stars_last_sprite_frame_update_time: Instant,
    pub stars_sprite_frame_index: usize,
    pub perk_eligibility: bool,
    pub selected_perk: Option<usize>,
    pub food_score_value: u32,
    pub perk_history: HashMap<u32, Perk>,
    pub perk_required_score: u32,
    pub audio_manager: AudioManager,
    pub music_disabled: bool
}

impl<'a> GameState<'a> {
    pub fn new(
        player: Snake,
        sprites: SpriteMaps,
        window_buffer: &'a mut Vec<u32>,
        window_width: usize,
        window_height: usize,
        window: &'a mut Window,
        scaled_buffer: &'a mut Vec<u32>,
        audio_manager: AudioManager
    ) -> Self {
        GameState {
            player,
            sprites,
            window_buffer,
            window_width,
            window_height,
            window,
            scaled_buffer,
            delta_time: 0.0,
            last_frame_time: None,
            game_over: false,
            score: 0,
            globe_sprite_frame_index: 0,
            food: Food {
                position: Vector2D { x: 100.0, y: 100.0 },
                is_active: false,
                food_sprite_frame_index: 0,
                food_last_sprite_frame_index_update_time: Instant::now(),
            },
            globe_last_sprite_frame_update_time: Instant::now(),
            stars_offset_x: 0,
            stars_last_sprite_frame_update_time: Instant::now(),
            stars_sprite_frame_index: 0,
            perk_eligibility: false,
            selected_perk: None,
            food_score_value: 100, // Default score value for food is 100
            perk_history: HashMap::new(),
            perk_required_score: 1000, // Every 1000 points, a perk becomes available
            audio_manager,
            music_disabled: false,
        }
    }

    pub fn restart_level(&mut self) {
        self.player = Snake::new(40.0, 150.0, Direction::Right);
        self.food = Food {
            position: Vector2D { x: 100.0, y: 100.0 },
            is_active: false,
            food_sprite_frame_index: 0,
            food_last_sprite_frame_index_update_time: Instant::now(),
        };
        self.score = 0;
        self.game_over = false;
        self.globe_sprite_frame_index = 0;
        self.globe_last_sprite_frame_update_time = Instant::now();
    }
}

