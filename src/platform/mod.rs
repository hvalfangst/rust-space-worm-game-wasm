pub mod web;

// Re-export platform-specific modules
pub use web::*;

// Common traits and types that both platforms must implement
pub trait PlatformAudio {
    type Error;
    
    fn new() -> Result<Self, Self::Error> where Self: Sized;
    fn play_music(&self, music_id: crate::audio::MusicId) -> Result<(), Self::Error>;
    fn play_sfx(&self, sfx_id: crate::audio::SfxId) -> Result<(), Self::Error>;
    fn stop_music(&self);
    fn set_music_volume(&self, volume: f32);
    fn is_music_playing(&self) -> bool;
}

pub trait PlatformWindow {
    type Error;
    type Buffer;
    
    fn new(title: &str, width: usize, height: usize) -> Result<Self, Self::Error> where Self: Sized;
    fn update_buffer(&mut self, buffer: &[u32]) -> Result<(), Self::Error>;
    fn should_close(&self) -> bool;
    fn get_size(&self) -> (usize, usize);
}

pub trait PlatformInput {
    fn is_key_pressed(&self, key: VirtualKey) -> bool;
    fn get_mouse_pos(&self) -> Option<(f32, f32)>;
    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VirtualKey {
    Up,
    Down,
    Left,
    Right,
    Space,
    Enter,
    Escape,
    W,
    A,
    S,
    D,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}