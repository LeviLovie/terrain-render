// Vertex shader

struct Camera {
    view_proj: mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}





// Fragment shader

struct Dimensions {
    width: f32,
    height: f32,
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;
@group(0) @binding(2)
var geo_texture: texture_2d<f32>;
@group(0) @binding(3)
var geo_sampler: sampler;
@group(0) @binding(4)
var<uniform> u_dimensions: Dimensions;

fn smoothTintBlend(height: f32) -> vec3<f32> {
    return mix(vec3<f32>(1.0, 0.0, 0.0),
               vec3<f32>(0.0, 1.0, 0.0),
               height);
}

fn bilinearSample(texture: texture_2d<f32>, smp: sampler, uv: vec2<f32>) -> f32 {
    let tex_size = textureDimensions(texture);
    let texel_size = 1.0 / vec2<f32>(tex_size);

    let uv_texel = uv * vec2<f32>(tex_size);
    let i_uv = floor(uv_texel);
    let f_uv = fract(uv_texel);

    let tl = textureSample(texture, smp, (i_uv + vec2<f32>(0.0, 0.0)) * texel_size).r;
    let tr = textureSample(texture, smp, (i_uv + vec2<f32>(1.0, 0.0)) * texel_size).r;
    let bl = textureSample(texture, smp, (i_uv + vec2<f32>(0.0, 1.0)) * texel_size).r;
    let br = textureSample(texture, smp, (i_uv + vec2<f32>(1.0, 1.0)) * texel_size).r;

    let t = mix(tl, tr, f_uv.x);
    let b = mix(bl, br, f_uv.x);
    return mix(t, b, f_uv.y);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let base_color = textureSample(t_diffuse, s_diffuse, in.tex_coords).rgb;
    let height = textureSample(geo_texture, geo_sampler, in.tex_coords).r;
    let int_height = bilinearSample(geo_texture, geo_sampler, in.tex_coords);
    let tint_color = smoothTintBlend(int_height);
    let final_color = mix(base_color, tint_color, 0.1);
    return vec4<f32>(final_color, 1.0);
}

// @fragment
// fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
//     return textureSample(t_diffuse, s_diffuse, in.tex_coords);
// }
