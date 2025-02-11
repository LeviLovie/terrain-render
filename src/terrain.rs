use super::state::Vertex;
use egui_wgpu::wgpu::Texture;
use tracing::{debug, trace, trace_span};

pub fn texture_to_vertices(texture: Texture, buffer: Vec<f64>) -> (Vec<Vertex>, Vec<u16>) {
    let span = trace_span!("texture_to_vertices");
    let _enter = span.enter();

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let size = texture.size();
    let width = size.width as f32;
    let height = size.height as f32;

    let minimum_value = buffer.iter().cloned().fold(f64::INFINITY, f64::min);
    debug!("Minimum value: {}", minimum_value);

    for y in 0..height as u32 {
        for x in 0..width as u32 {
            let pixel = (buffer[(y * width as u32 + x) as usize] - minimum_value) / 30.0;
            trace!("Pixel value: {}", pixel);
            vertices.push(Vertex {
                position: [x as f32, pixel as f32, y as f32],
                tex_coords: [x as f32 / width as f32, y as f32 / height as f32],
            });
        }
    }

    for y in 0..height as i32 - 1 {
        if y % 2 == 0 {
            for x in 0..width as i32 {
                indices.push((y as f32 * width + x as f32) as u16);
                indices.push(((y + 1) as f32 * width + x as f32) as u16);
            }
        } else {
            // Reverse the direction of the row
            for x in (0..width as i32).rev() {
                indices.push((y as f32 * width + x as f32) as u16);
                indices.push(((y + 1) as f32 * width + x as f32) as u16);
            }
        }
    }

    (vertices, indices)
}
