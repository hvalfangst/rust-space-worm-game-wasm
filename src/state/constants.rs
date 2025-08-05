pub mod graphics {
    pub const SCALED_WINDOW_WIDTH: usize = 960;
    pub const SCALED_WINDOW_HEIGHT: usize = 540;
    pub const ART_WIDTH: usize = 256;
    pub const ART_HEIGHT: usize = 224;
    pub const SNAKE_BODY_WIDTH: f32 = 6.0;
    pub const SNAKE_BODY_HEIGHT: f32 = 8.0;
}

pub mod physics {
    pub const LOWER_BOUND_X: f32 = 2.0;
    pub const UPPER_BOUND_X: f32 = 256.0;
    pub const LOWER_BOUND_Y: f32 = 2.0;
    pub const UPPER_BOUND_Y: f32 = 224.0;
    pub const COLLISION_TOLERANCE: f32 = 1.0;
}

pub mod text {
    pub const SCORE: &str = "Score: ";
    pub const SELECT_POWERUP: &str = "Select Powerup";
    pub const POWERUP_NEED_4_SPEED: (&str, &str) = ("Need 4 Speed", "+25% movement speed");
    pub const POWERUP_HUNGRY_WORM: (&str, &str) = ("Hungry Worm", "2x score from food");
}

pub mod audio {

    // - - - - - - - - - - - - - - | MUSIC | - - - - - - - - - - - - - -
    pub const SPACE_WORM_FILE: &str = "assets/audio/space_worm.mp3";


    // - - - - - - - - - - - - - - | FX | - - - - - - - - - - - - - -
    pub const NEW_POWERUP_FILE: &str = "assets/audio/new_perk.mp3";
    pub const NEED_FOR_SPEED_POWERUP_CHOSEN_FILE: &str = "assets/audio/turbo.mp3";
    pub const HUNGRY_WORM_POWERUP_CHOSEN_FILE: &str = "assets/audio/hungry_worm.mp3";

    pub const SNAKE_EAT_FOOD_FILE: &str = "assets/audio/eat.mp3";
    pub const GAME_OVER_FILE: &str = "assets/audio/game_over.mp3";
}

pub mod state {
    pub const FRAME_RATE_SLEEP_DURATION: u64 = 16; // 16 ms for ~60 FPS
    pub const LOOT_CRATE_SPAWN_INTERVAL: u32 = 100; // Every 100 points, check for loot crate spawn
    pub const LOOT_CRATE_SPAWN_CHANCE: u8 = 20; // 20% chance to spawn loot crate
}

