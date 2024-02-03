struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
};

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
    @location(9) i_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_space_position: vec4<f32>,
    @location(0) world_space_position: vec3<f32>,
    @location(1) color: vec4<f32>,
};

struct FragmentInput {
    @builtin(position) fragment_position: vec4<f32>,
    @location(0) world_space_position: vec3<f32>,
    @location(1) color: vec4<f32>,
};

struct Uniforms {
    window_size: vec4<f32>
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
        idata.model_matrix_0,
        idata.model_matrix_1,
        idata.model_matrix_2,
        idata.model_matrix_3,
    );

    let wgpu_to_px = mat4x4<f32>(
        vec4(2.0 / uniforms.window_size.x, 0.0, 0.0, 0.0),
        vec4(0.0, -2.0 / uniforms.window_size.y, 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(-1.0, 1.0, 0.0, 1.0)
    );

    var screen_position = wgpu_to_px * transform * vec4<f32>(in.position, 1.0);
    out.clip_space_position = screen_position;
    out.color = idata.i_color;

    return out;
}

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color.rgba);
}
