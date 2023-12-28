struct VertexOutput {
    @builtin(position) clip_space_position: vec4<f32>,
    @location(0) world_space_position: vec3<f32>,
};

struct FragmentInput {
    @builtin(position) fragment_position: vec4<f32>,
    @location(0) world_space_position: vec3<f32>,
};

@vertex fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32
) -> VertexOutput {
    var out: VertexOutput;

    // Define the vertices of the triangle
    let vertex0 = vec2<f32>(-2.0, -1.0);
    let vertex1 = vec2<f32>(2.0, -1.0);
    let vertex2 = vec2<f32>(0.0, 4.0);

    // Set the clip space position based on the vertex index
    if (in_vertex_index == 0u) {
        out.clip_space_position = vec4<f32>(vertex0, 0.0, 1.0);
    } else if (in_vertex_index == 1u) {
        out.clip_space_position = vec4<f32>(vertex1, 0.0, 1.0);
    } else {
        out.clip_space_position = vec4<f32>(vertex2, 0.0, 1.0);
    }

    // Set the world space position
    out.world_space_position = out.clip_space_position.xyz;

    return out;
}

const PI = 3.14159265359;
const TAU = 6.28318530718;
const MAX_STEPS = 1000.0;
const EPSILON = 0.01;

fn raymarch (ro: vec3<f32>, rd: vec3<f32>) -> vec4<f32> {
    var dist = 0.0;
    var photon = ro;
    var col = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    for(var step: f32; step < MAX_STEPS; step = step + 1.0) {
        dist += distance(photon, vec3(0.0, 0.0, 1.0)) - 0.5;
        
        if (dist < EPSILON) {
            col = vec4(1.0, 0.3, 0.6, 1.0);
            break;
        }
        
        photon = ro + rd * dist;
    }
    return col;
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return a * t + b * (1.0 - t);
}

@fragment fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    // Normalize clip space position for display
    let uv = vec2<f32>(in.fragment_position.x / 640.0 - 0.5, in.fragment_position.y / 360.0 - 0.5) * 2.0;

    let fov = vec2(0.4 * TAU, 0.4 * TAU * 9.0/16.0);
    let ray = normalize(vec3<f32>(
        lerp(-fov.x, fov.x, uv.x),
        lerp(-fov.y, fov.y, uv.y),
        1.0
    ));

    let col = raymarch(
        vec3<f32>(0.0, 0.0, 0.0),
        vec3<f32>(ray)
    );


    return vec4<f32>(col);
}
