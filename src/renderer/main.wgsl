struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
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
    val: f32
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex fn vs_main(
    in: VertexInput
) -> VertexOutput {
    var out: VertexOutput;

    out.clip_space_position = vec4(in.position + uniforms.val * 0.4, 1.0);
    out.color = in.color;

    return out;
}

@fragment fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color.rgba);
}
