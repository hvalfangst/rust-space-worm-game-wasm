use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::collections::HashMap;
use std::fs::File;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MusicId {
    Music0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SfxId {
    NewPerk,
    Eat
}

pub struct AudioManager {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    music_sink: Arc<Mutex<Sink>>,
    fx_sink: Arc<Mutex<Option<Sink>>>,
    // State tracking to avoid expensive checks
    music_playing: Arc<Mutex<bool>>,
    last_music_check: Arc<Mutex<Instant>>,
    music_check_interval: Duration,
    // Preloaded audio data by ID
    music_cache: Arc<Mutex<HashMap<MusicId, Vec<u8>>>>,
    sfx_cache: Arc<Mutex<HashMap<SfxId, Vec<u8>>>>,
    current_music_id: Arc<Mutex<Option<MusicId>>>,
}

impl AudioManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Get the default audio device with full control
        let (_stream, stream_handle) = OutputStream::try_default()?;

        // Create dedicated sink for background music
        let music_sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle)?));

        // FX sink will be created on-demand
        let fx_sink = Arc::new(Mutex::new(None));

        Ok(AudioManager {
            _stream,
            stream_handle,
            music_sink,
            fx_sink,
            music_playing: Arc::new(Mutex::new(false)),
            last_music_check: Arc::new(Mutex::new(Instant::now())),
            music_check_interval: Duration::from_millis(500),
            music_cache: Arc::new(Mutex::new(HashMap::new())),
            sfx_cache: Arc::new(Mutex::new(HashMap::new())),
            current_music_id: Arc::new(Mutex::new(None)),
        })
    }

    /// Load all music files at startup - call this once during initialization
    pub fn preload_all_music(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Preloading all music files...");

        // Define your music file mappings here
        let music_files = [
            (MusicId::Music0, "assets/audio/music_0.mp3")
        ];

        let mut cache = self.music_cache.lock().unwrap();
        let mut total_size = 0;

        for (music_id, file_path) in music_files.iter() {
            match File::open(file_path) {
                Ok(mut file) => {
                    let mut file_data = Vec::new();
                    std::io::Read::read_to_end(&mut file, &mut file_data)?;
                    total_size += file_data.len();
                    cache.insert(*music_id, file_data.clone());
                    println!("Loaded {:?}: {} ({} KB)", music_id, file_path, file_data.len() / 1024);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to load {:?} from {}: {}", music_id, file_path, e);
                }
            }
        }

        println!("Music preloading complete! Total size: {} MB", total_size / (1024 * 1024));
        Ok(())
    }

    /// Load all sound effects at startup - call this once during initialization
    pub fn preload_all_sfx(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Preloading all sound effects...");

        // Define your SFX file mappings here
        let sfx_files = [
            (SfxId::NewPerk, "assets/audio/new_perk.mp3"),
            (SfxId::Eat, "assets/audio/eat.mp3"),
        ];

        let mut cache = self.sfx_cache.lock().unwrap();
        let mut total_size = 0;

        for (sfx_id, file_path) in sfx_files.iter() {
            match File::open(file_path) {
                Ok(mut file) => {
                    let mut file_data = Vec::new();
                    std::io::Read::read_to_end(&mut file, &mut file_data)?;
                    total_size += file_data.len();
                    cache.insert(*sfx_id, file_data.clone());
                    println!("Loaded {:?}: {} ({} KB)", sfx_id, file_path, file_data.len() / 1024);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to load {:?} from {}: {}", sfx_id, file_path, e);
                }
            }
        }

        println!("SFX preloading complete! Total size: {} KB", total_size / 1024);
        Ok(())
    }

    /// Play music by ID (ultra-fast, no file I/O)
    pub fn play_music(&self, music_id: MusicId) -> Result<(), Box<dyn std::error::Error>> {
        // Check if we're already playing this exact music
        {
            let current_music = self.current_music_id.lock().unwrap();
            if let Some(current_id) = current_music.as_ref() {
                if *current_id == music_id && *self.music_playing.lock().unwrap() {
                    println!("Music {:?} is already playing", music_id);
                    return Ok(());
                }
            }
        }

        // Get preloaded audio data
        let audio_data = {
            let cache = self.music_cache.lock().unwrap();
            match cache.get(&music_id) {
                Some(data) => {
                    println!("Playing preloaded music: {:?}", music_id);
                    data.clone()
                }
                None => {
                    return Err(format!("Music {:?} not found in cache! Did you call preload_all_music()?", music_id).into());
                }
            }
        };

        // Decode from memory (instant)
        let cursor = Cursor::new(audio_data);
        let source = Decoder::new(cursor)?;

        let music_sink = self.music_sink.lock().unwrap();

        // Stop any existing music
        music_sink.stop();

        // Play new music on repeat
        music_sink.append(source.repeat_infinite());
        music_sink.play();

        // Update our state tracking
        *self.music_playing.lock().unwrap() = true;
        *self.current_music_id.lock().unwrap() = Some(music_id);

        Ok(())
    }

    /// Play sound effect by ID (ultra-fast, no file I/O)
    pub fn play_sfx(&self, sfx_id: SfxId) -> Result<(), Box<dyn std::error::Error>> {
        // Get preloaded audio data
        let audio_data = {
            let cache = self.sfx_cache.lock().unwrap();
            match cache.get(&sfx_id) {
                Some(data) => data.clone(),
                None => {
                    return Err(format!("SFX {:?} not found in cache! Did you call preload_all_sfx()?", sfx_id).into());
                }
            }
        };

        let cursor = Cursor::new(audio_data);
        let source = Decoder::new(cursor)?;

        let mut fx_sink_guard = self.fx_sink.lock().unwrap();

        // Stop and replace any existing FX
        if let Some(existing_sink) = fx_sink_guard.as_ref() {
            existing_sink.stop();
        }

        // Create new sink for this FX
        let new_sink = Sink::try_new(&self.stream_handle)?;
        new_sink.append(source);
        new_sink.play();

        *fx_sink_guard = Some(new_sink);

        Ok(())
    }

    /// Play SFX with custom volume
    pub fn play_sfx_with_volume(&self, sfx_id: SfxId, volume: f32) -> Result<(), Box<dyn std::error::Error>> {
        let audio_data = {
            let cache = self.sfx_cache.lock().unwrap();
            match cache.get(&sfx_id) {
                Some(data) => data.clone(),
                None => {
                    return Err(format!("SFX {:?} not found in cache!", sfx_id).into());
                }
            }
        };

        let cursor = Cursor::new(audio_data);
        let source = Decoder::new(cursor)?;

        let mut fx_sink_guard = self.fx_sink.lock().unwrap();

        if let Some(existing_sink) = fx_sink_guard.as_ref() {
            existing_sink.stop();
        }

        let new_sink = Sink::try_new(&self.stream_handle)?;
        new_sink.set_volume(volume.clamp(0.0, 1.0));
        new_sink.append(source);
        new_sink.play();

        *fx_sink_guard = Some(new_sink);

        Ok(())
    }

    /// Get currently playing music ID
    pub fn get_current_music(&self) -> Option<MusicId> {
        *self.current_music_id.lock().unwrap()
    }

    /// Check if specific music is playing
    pub fn is_music_playing_id(&self, music_id: MusicId) -> bool {
        if let Some(current_id) = *self.current_music_id.lock().unwrap() {
            current_id == music_id && self.is_music_playing()
        } else {
            false
        }
    }

    /// Get memory usage info
    pub fn get_memory_usage(&self) -> (usize, usize) {
        let music_size: usize = self.music_cache.lock().unwrap()
            .values()
            .map(|data| data.len())
            .sum();

        let sfx_size: usize = self.sfx_cache.lock().unwrap()
            .values()
            .map(|data| data.len())
            .sum();

        (music_size, sfx_size)
    }

    /// List all preloaded music
    pub fn list_preloaded_music(&self) -> Vec<MusicId> {
        self.music_cache.lock().unwrap().keys().copied().collect()
    }

    /// List all preloaded SFX
    pub fn list_preloaded_sfx(&self) -> Vec<SfxId> {
        self.sfx_cache.lock().unwrap().keys().copied().collect()
    }

    // === ORIGINAL METHODS (unchanged) ===

    pub fn stop_music(&self) {
        let music_sink = self.music_sink.lock().unwrap();
        music_sink.stop();
        *self.music_playing.lock().unwrap() = false;
        *self.current_music_id.lock().unwrap() = None;
    }

    pub fn pause_music(&self) {
        let music_sink = self.music_sink.lock().unwrap();
        music_sink.pause();
    }

    pub fn resume_music(&self) {
        let music_sink = self.music_sink.lock().unwrap();
        music_sink.play();
    }

    pub fn set_music_volume(&self, volume: f32) {
        let music_sink = self.music_sink.lock().unwrap();
        music_sink.set_volume(volume.clamp(0.0, 1.0));
    }

    pub fn stop_fx(&self) {
        let mut fx_sink_guard = self.fx_sink.lock().unwrap();
        if let Some(sink) = fx_sink_guard.as_ref() {
            sink.stop();
        }
        *fx_sink_guard = None;
    }

    pub fn is_fx_playing(&self) -> bool {
        let fx_sink_guard = self.fx_sink.lock().unwrap();
        match fx_sink_guard.as_ref() {
            Some(sink) => !sink.empty(),
            None => false,
        }
    }

    pub fn is_music_playing(&self) -> bool {
        let now = Instant::now();
        let mut last_check = self.last_music_check.lock().unwrap();

        if now.duration_since(*last_check) >= self.music_check_interval {
            let music_sink = self.music_sink.lock().unwrap();
            let actually_playing = !music_sink.empty();
            *self.music_playing.lock().unwrap() = actually_playing;
            *last_check = now;
            actually_playing
        } else {
            *self.music_playing.lock().unwrap()
        }
    }

    pub fn is_music_playing_cached(&self) -> bool {
        *self.music_playing.lock().unwrap()
    }

    pub fn check_music_state_now(&self) -> bool {
        let music_sink = self.music_sink.lock().unwrap();
        let actually_playing = !music_sink.empty();
        *self.music_playing.lock().unwrap() = actually_playing;
        *self.last_music_check.lock().unwrap() = Instant::now();
        actually_playing
    }

    pub fn set_music_check_interval(&mut self, interval: Duration) {
        self.music_check_interval = interval;
    }

    pub fn get_music_volume(&self) -> f32 {
        let music_sink = self.music_sink.lock().unwrap();
        music_sink.volume()
    }
}