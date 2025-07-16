// Platform-agnostic audio manager
use crate::platform::PlatformAudio;

// Define for WASM since the manager module is not compiled
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MusicId {
    Music0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SfxId {
    NewPerk,
    Eat,
}

pub struct UnifiedAudioManager<T: PlatformAudio> {
    platform_audio: T,
}

impl<T: PlatformAudio> UnifiedAudioManager<T> {
    pub fn new() -> Result<Self, T::Error> {
        let platform_audio = T::new()?;
        Ok(Self { platform_audio })
    }
    
    pub fn play_music(&self, music_id: MusicId) -> Result<(), T::Error> {
        self.platform_audio.play_music(music_id)
    }
    
    pub fn play_sfx(&self, sfx_id: SfxId) -> Result<(), T::Error> {
        self.platform_audio.play_sfx(sfx_id)
    }
    
    pub fn stop_music(&self) {
        self.platform_audio.stop_music();
    }
    
    pub fn set_music_volume(&self, volume: f32) {
        self.platform_audio.set_music_volume(volume);
    }
    
    pub fn is_music_playing(&self) -> bool {
        self.platform_audio.is_music_playing()
    }
    
    // Placeholder methods for backward compatibility
    pub fn preload_all_music(&self) -> Result<(), T::Error> {
        // Platform-specific implementations will handle preloading in their new() method
        Ok(())
    }
    
    pub fn preload_all_sfx(&self) -> Result<(), T::Error> {
        // Platform-specific implementations will handle preloading in their new() method
        Ok(())
    }
}

// Type alias for WASM platform
pub type AudioManager = UnifiedAudioManager<crate::platform::WebAudio>;