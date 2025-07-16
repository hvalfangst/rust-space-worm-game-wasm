use crate::state::constants::graphics::{ART_WIDTH, ART_HEIGHT, SCALED_WINDOW_WIDTH, SCALED_WINDOW_HEIGHT};

pub fn scale_buffer_to_screen(src_buffer: &[u32], pixel_buffer: &mut [u32]) {
    let x_scale = SCALED_WINDOW_WIDTH as f32 / ART_WIDTH as f32;
    let y_scale = SCALED_WINDOW_HEIGHT as f32 / ART_HEIGHT as f32;
    
    for y in 0..SCALED_WINDOW_HEIGHT {
        for x in 0..SCALED_WINDOW_WIDTH {
            let src_x = (x as f32 / x_scale).floor() as usize;
            let src_y = (y as f32 / y_scale).floor() as usize;
            
            if src_x < ART_WIDTH && src_y < ART_HEIGHT {
                let src_pixel = src_buffer[src_y * ART_WIDTH + src_x];
                pixel_buffer[y * SCALED_WINDOW_WIDTH + x] = src_pixel;
            }
        }
    }
}

pub fn update_canvas(
    pixel_buffer: &[u32],
    context: &web_sys::CanvasRenderingContext2d,
) -> Result<(), wasm_bindgen::JsValue> {
    // Convert ARGB to RGBA for web
    let mut rgba_data = Vec::with_capacity(pixel_buffer.len() * 4);
    for &pixel in pixel_buffer {
        let a = ((pixel >> 24) & 0xFF) as u8;
        let r = ((pixel >> 16) & 0xFF) as u8;
        let g = ((pixel >> 8) & 0xFF) as u8;
        let b = (pixel & 0xFF) as u8;
        
        rgba_data.push(r);
        rgba_data.push(g);
        rgba_data.push(b);
        rgba_data.push(a);
    }
    
    // Create ImageData from the pixel buffer
    let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&rgba_data),
        SCALED_WINDOW_WIDTH as u32,
        SCALED_WINDOW_HEIGHT as u32,
    )?;
    
    // Draw to canvas
    context.put_image_data(&image_data, 0.0, 0.0)?;
    
    Ok(())
}