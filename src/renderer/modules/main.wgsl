struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct InstanceInput {
    @location(5) transform_0: vec4<f32>,
    @location(6) transform_1: vec4<f32>,
    @location(7) transform_2: vec4<f32>,
    @location(8) transform_3: vec4<f32>,
    @location(9) i_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_space_position: vec4<f32>,
    @location(0) world_space_position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>, 
};

struct FragmentInput {
    @builtin(position) fragment_position: vec4<f32>,
    @location(0) world_space_position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>, 
};

struct Uniforms {
    px_to_wgpu_0: vec4<f32>,
    px_to_wgpu_1: vec4<f32>,
    px_to_wgpu_2: vec4<f32>,
    px_to_wgpu_3: vec4<f32>,
    camera_position: vec4<f32>
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(
    in: VertexInput,
    idata: InstanceInput
) -> VertexOutput {
    var out: VertexOutput;
    let transform = mat4x4<f32>(
        idata.transform_0,
        idata.transform_1,
        idata.transform_2,
        idata.transform_3,
    );
    let px_to_wgpu = mat4x4<f32>(
        uniforms.px_to_wgpu_0,
        uniforms.px_to_wgpu_1,
        uniforms.px_to_wgpu_2,
        uniforms.px_to_wgpu_3
    );
    var screen_position = px_to_wgpu * (
        transform * vec4<f32>(in.position, 1.0) - uniforms.camera_position);
    out.clip_space_position = screen_position;
    out.uv = in.uv;
    out.color = idata.i_color;
    return out;
}

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    return vec4(uv.x, uv.y, 1.0, 1.0);
}

fn SDFRect(position: vec2<f32>, halfSize: vec2<f32>, cornerRadius: f32) -> f32 {
   let p = abs(position) - halfSize + cornerRadius;
   return length(vec2(max(p.x, 0.0), max(p.y, 0.0))) + min(max(p.x, p.y), 0.0) - cornerRadius;
}