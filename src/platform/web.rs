use super::{PlatformAudio, PlatformWindow, PlatformInput, VirtualKey, MouseButton};
use crate::audio::{MusicId, SfxId};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub struct WebAudio {
    audio_context: AudioContext,
    music_buffers: HashMap<MusicId, AudioBuffer>,
    sfx_buffers: HashMap<SfxId, AudioBuffer>,
    current_music_source: Option<AudioBufferSourceNode>,
    music_volume: f32,
}

impl PlatformAudio for WebAudio {
    type Error = JsValue;
    
    fn new() -> Result<Self, Self::Error> {
        let audio_context = AudioContext::new()?;
        
        Ok(Self {
            audio_context,
            music_buffers: HashMap::new(),
            sfx_buffers: HashMap::new(),
            current_music_source: None,
            music_volume: 1.0,
        })
    }
    
    fn play_music(&self, music_id: MusicId) -> Result<(), Self::Error> {
        if let Some(buffer) = self.music_buffers.get(&music_id) {
            // Stop current music if playing
            if let Some(ref source) = self.current_music_source {
                source.stop()?;
            }
            
            let source = self.audio_context.create_buffer_source()?;
            source.set_buffer(Some(buffer));
            source.set_loop(true);
            
            let gain_node = self.audio_context.create_gain()?;
            // Note: Gain control simplified for now
            // gain_node.gain().set_value(self.music_volume);
            
            source.connect_with_audio_node(&gain_node)?;
            gain_node.connect_with_audio_node(&self.audio_context.destination())?;
            
            source.start()?;
            console_log!("Playing music: {:?}", music_id);
        } else {
            console_log!("Music buffer not found: {:?}", music_id);
        }
        Ok(())
    }
    
    fn play_sfx(&self, sfx_id: SfxId) -> Result<(), Self::Error> {
        if let Some(buffer) = self.sfx_buffers.get(&sfx_id) {
            let source = self.audio_context.create_buffer_source()?;
            source.set_buffer(Some(buffer));
            source.connect_with_audio_node(&self.audio_context.destination())?;
            source.start()?;
            console_log!("Playing SFX: {:?}", sfx_id);
        } else {
            console_log!("SFX buffer not found: {:?}", sfx_id);
        }
        Ok(())
    }
    
    fn stop_music(&self) {
        if let Some(ref source) = self.current_music_source {
            let _ = source.stop();
        }
    }
    
    fn set_music_volume(&self, _volume: f32) {
        // This would require storing the gain node reference
        // For now, just store the volume for future use
    }
    
    fn is_music_playing(&self) -> bool {
        // Web Audio API doesn't provide easy way to check if playing
        // Return true if we have a current source
        self.current_music_source.is_some()
    }
}

pub struct WebWindow {
    pub canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    width: usize,
    height: usize,
    image_data: ImageData,
}

impl PlatformWindow for WebWindow {
    type Error = JsValue;
    type Buffer = Vec<u32>;
    
    fn new(title: &str, width: usize, height: usize) -> Result<Self, Self::Error> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
        // Set document title
        document.set_title(title);
        
        let canvas = document
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()?;
        
        canvas.set_width(width as u32);
        canvas.set_height(height as u32);
        // Note: Styling simplified for now
        // canvas.style().set_property("border", "1px solid black")?;
        
        let body = document.body().unwrap();
        body.append_child(&canvas)?;
        
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;
        
        let image_data = ImageData::new_with_sw(width as u32, height as u32)?;
        
        Ok(Self {
            canvas,
            context,
            width,
            height,
            image_data,
        })
    }
    
    fn update_buffer(&mut self, buffer: &[u32]) -> Result<(), Self::Error> {
        let mut data = self.image_data.data();
        
        for (i, &pixel) in buffer.iter().enumerate() {
            let base = i * 4;
            if base + 3 < data.len() {
                data[base] = ((pixel >> 16) & 0xFF) as u8; // R
                data[base + 1] = ((pixel >> 8) & 0xFF) as u8; // G
                data[base + 2] = (pixel & 0xFF) as u8; // B
                data[base + 3] = 255; // A
            }
        }
        
        self.context.put_image_data(&self.image_data, 0.0, 0.0)?;
        Ok(())
    }
    
    fn should_close(&self) -> bool {
        false // Web apps typically don't close
    }
    
    fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

pub struct WebInput {
    pressed_keys: Rc<RefCell<HashMap<VirtualKey, bool>>>,
    mouse_pos: Rc<RefCell<Option<(f32, f32)>>>,
    mouse_buttons: Rc<RefCell<HashMap<MouseButton, bool>>>,
}

impl WebInput {
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Self, JsValue> {
        let pressed_keys = Rc::new(RefCell::new(HashMap::new()));
        let mouse_pos = Rc::new(RefCell::new(None));
        let mouse_buttons = Rc::new(RefCell::new(HashMap::new()));
        
        // Set up keyboard event listeners
        {
            let pressed_keys_clone = pressed_keys.clone();
            let keydown_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                if let Some(key) = key_code_to_virtual(&event.code()) {
                    pressed_keys_clone.borrow_mut().insert(key, true);
                }
            }) as Box<dyn FnMut(_)>);
            
            let window = web_sys::window().unwrap();
            window.add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())?;
            keydown_closure.forget();
        }
        
        {
            let pressed_keys_clone = pressed_keys.clone();
            let keyup_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                if let Some(key) = key_code_to_virtual(&event.code()) {
                    pressed_keys_clone.borrow_mut().insert(key, false);
                }
            }) as Box<dyn FnMut(_)>);
            
            let window = web_sys::window().unwrap();
            window.add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref())?;
            keyup_closure.forget();
        }
        
        // Set up mouse event listeners
        {
            let mouse_pos_clone = mouse_pos.clone();
            let mousemove_closure = Closure::wrap(Box::new(move |event: MouseEvent| {
                *mouse_pos_clone.borrow_mut() = Some((event.offset_x() as f32, event.offset_y() as f32));
            }) as Box<dyn FnMut(_)>);
            
            canvas.add_event_listener_with_callback("mousemove", mousemove_closure.as_ref().unchecked_ref())?;
            mousemove_closure.forget();
        }
        
        Ok(Self {
            pressed_keys,
            mouse_pos,
            mouse_buttons,
        })
    }
}

impl PlatformInput for WebInput {
    fn is_key_pressed(&self, key: VirtualKey) -> bool {
        self.pressed_keys.borrow().get(&key).copied().unwrap_or(false)
    }
    
    fn get_mouse_pos(&self) -> Option<(f32, f32)> {
        *self.mouse_pos.borrow()
    }
    
    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons.borrow().get(&button).copied().unwrap_or(false)
    }
}

fn key_code_to_virtual(code: &str) -> Option<VirtualKey> {
    match code {
        "KeyW" => Some(VirtualKey::W),
        "KeyA" => Some(VirtualKey::A),
        "KeyS" => Some(VirtualKey::S),
        "KeyD" => Some(VirtualKey::D),
        "Space" => Some(VirtualKey::Space),
        "Enter" => Some(VirtualKey::Enter),
        "Escape" => Some(VirtualKey::Escape),
        _ => None,
    }
}