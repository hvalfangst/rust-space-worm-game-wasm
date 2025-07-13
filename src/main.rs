use minifb::{Window, WindowOptions};
use winit::event_loop::EventLoop;
use winit::monitor::MonitorHandle;

use crate::state::constants::graphics::{ART_WIDTH, SCALED_WINDOW_HEIGHT, SCALED_WINDOW_WIDTH};

use crate::state::structs::{Direction, GameState, Snake};
use crate::{
    graphics::sprites::SpriteMaps,
    state::core::initialize_core_logic_map,
    state::r#loop::start_event_loop,
};

use rodio::{OutputStream, Sink};
use crate::audio::manager::AudioManager;

mod state;
mod graphics;
mod input;
mod audio;

fn main() {
    // Initialize the audio output stream and sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sprites = SpriteMaps::new();

    let player = Snake::new(40.0, 150.0, Direction::Right);
    let core_logic = initialize_core_logic_map();
    let fullscreen = false;

    // Determine window size based on fullscreen flag
    let (window_width, window_height) = if fullscreen {
        let primary_monitor: MonitorHandle =  EventLoop::new().primary_monitor().expect("Failed to get primary monitor");
        let screen_size = primary_monitor.size();
        (screen_size.width as usize, screen_size.height as usize)
    } else {
        (SCALED_WINDOW_WIDTH, SCALED_WINDOW_HEIGHT)
    };

    // Create a window with the dimensions of the primary monitor
    let mut window = Window::new(
        "Space Worm",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });


    // Initialize window and scaled buffer
    let mut window_buffer = vec![0; ART_WIDTH * ART_WIDTH];
    let mut scaled_buffer = vec![0; window_width * window_height];

    // Create audio manager and preload audio files
    let audio_manager = AudioManager::new().unwrap();
    audio_manager.preload_all_music().unwrap();
    audio_manager.preload_all_sfx().unwrap();

    let game_state = GameState::new(
        player,
        sprites,
        &mut window_buffer,
        window_width,
        window_height,
        &mut window,
        &mut scaled_buffer,
        audio_manager,
    );

    // Sleep for a second just to allow the audio manager to initialize properly
    std::thread::sleep(std::time::Duration::from_secs(1));

    start_event_loop(game_state, core_logic);
}