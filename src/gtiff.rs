use egui_wgpu::wgpu::{
    Device, Queue, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use gdal::Dataset;
use tracing::{debug, debug_span, error, trace};

/// Returns Texture and not normalized buffer with pixel data from a GeoTIFF file
pub fn load_geotiff_as_texture(device: &Device, queue: &Queue, path: &str) -> (Texture, Vec<f64>) {
    let span = debug_span!("gtiff_to_texture", path = path);
    let _enter = span.enter();

    // Open the GeoTIFF file
    let dataset = match Dataset::open(path) {
        Ok(dataset) => {
            trace!("Opened GeoTIFF file");
            dataset
        }
        Err(e) => {
            error!("Failed to open GeoTIFF file: {}", e);
            panic!("Failed to open GeoTIFF file");
        }
    };
    let band = match dataset.rasterband(1) {
        Ok(band) => {
            trace!("Got raster band");
            band
        }
        Err(e) => {
            error!("Failed to get raster band: {}", e);
            panic!("Failed to get raster band");
        }
    };

    // Get image dimensions
    let (width, height) = band.size();
    debug!("GeoTIFF dimensions: {}x{}", width, height);

    // Read the pixel data
    let buffer = match band.read_as::<f64>((0, 0), (width, height), (width, height), None) {
        Ok(buffer) => {
            trace!("Read pixel data");
            buffer
        }
        Err(e) => {
            error!("Failed to read pixel data: {}", e);
            panic!("Failed to read pixel data");
        }
    };

    // Normalize data to fit into [0, 1] r
    let min_val = buffer.data().iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = buffer
        .data()
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    trace!("Min value: {}", min_val);
    trace!("Max value: {}", max_val);

    let normalized_data: Vec<f32> = buffer
        .data()
        .iter()
        .map(|&v| ((v - min_val) / (max_val - min_val)) as f32)
        .collect();

    // Debug some values from normalized_data
    debug!("Normalized data [:10]:");
    for i in 0..10 {
        debug!("{}: {}", i, normalized_data[i]);
    }

    // Create a wgpu texture
    let texture = device.create_texture(&TextureDescriptor {
        label: Some("GeoTIFF Texture"),
        size: egui_wgpu::wgpu::Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::R32Float, // Use a floating-point format
        usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        view_formats: &[],
    });
    trace!("Created GeoTIFF texture");

    // Upload data to GPU
    queue.write_texture(
        texture.as_image_copy(),
        bytemuck::cast_slice(&normalized_data), // Convert to bytes
        egui_wgpu::wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(width as u32 * std::mem::size_of::<f32>() as u32),
            rows_per_image: Some(height as u32),
        },
        egui_wgpu::wgpu::Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        },
    );
    debug!("Uploaded GeoTIFF data to GPU");

    (texture, buffer.data().to_vec())
}
