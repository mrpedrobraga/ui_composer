use wgpu::vertex_attr_array;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

const VERTEX_SIZE: wgpu::BufferAddress = std::mem::size_of::<Vertex>() as wgpu::BufferAddress;

const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 2] = vertex_attr_array![
    0 => Float32x3,
    1 => Float32x4
];

impl Vertex {
    pub fn descriptor() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: VERTEX_SIZE,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &VERTEX_ATTRIBUTES,
        }
    }
}
