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
    pub const SELECT_PERK: &str = "Select Perk";
    pub const PERK_NEED_4_SPEED: (&str, &str) = ("Need 4 Speed", "+25% movement speed");
    pub const PERK_HUNGRY_WORM: (&str, &str) = ("Hungry Worm", "2x score from food");
    pub const PERK_CURSE_OF_GLOSSY: (&str, &str) = ("Curse of Glossy", "Death by shiny things");
}

pub mod audio {

    // - - - - - - - - - - - - - - | MUSIC | - - - - - - - - - - - - - -
    pub const MUSIC_0_FILE: &str = "assets/audio/music_0.mp3";
    pub const MUSIC_1_FILE: &str = "assets/audio/music_1.mp3";


    // - - - - - - - - - - - - - - | FX | - - - - - - - - - - - - - -
    pub const NEW_PERK_FILE: &str = "assets/audio/new_perk.mp3";
    pub const NEED_FOR_SPEED_PERK_CHOSEN_FILE: &str = "assets/audio/need_for_speed.mp3";
    pub const HUNGRY_WORM_PERK_CHOSEN_FILE: &str = "assets/audio/hungry_worm.mp3";

    pub const SNAKE_EAT_FOOD_FILE: &str = "assets/audio/eat.mp3";
    pub const GAME_OVER_FILE: &str = "assets/audio/game_over.mp3";
}

pub mod state {
    pub const FRAME_RATE_SLEEP_DURATION: u64 = 16; // 16 ms for ~60 FPS

    pub const SCORE_PERK_THRESHOLD_LEVEL_1: u32 = 100;
    pub const SCORE_PERK_THRESHOLD_LEVEL_2: u32 = 300;
    pub const SCORE_PERK_THRESHOLD_LEVEL_3: u32 = 600;
    pub const SCORE_PERK_THRESHOLD_LEVEL_4: u32 = 800;
}

