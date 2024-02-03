use wgpu::{util::DeviceExt, SurfaceConfiguration};

use crate::renderer::{
    formats::vertex::{InstanceData, Vertex},
    main_shader::ProgramUniforms,
};

pub fn create_uniform_bind_group(
    layout: &wgpu::BindGroupLayout,
    buffer: &wgpu::Buffer,
    device: &wgpu::Device,
) -> wgpu::BindGroup {
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
        label: Some("Main Uniform Bind Group"),
    });

    bind_group
}

pub fn create_uniform_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("Main Uniform Bind Group Layout"),
    });

    layout
}

pub fn create_uniform_buffer(uniforms: &ProgramUniforms, device: &wgpu::Device) -> wgpu::Buffer {
    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Main Index Buffer"),
        contents: bytemuck::cast_slice(&[*uniforms]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    uniform_buffer
}

pub fn create_test_buffers(
    data: (&[Vertex], &[u16], &[InstanceData]),
    device: &wgpu::Device,
) -> (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer) {
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Main Vertex Buffer"),
        contents: bytemuck::cast_slice(data.0),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Main Index Buffer"),
        contents: bytemuck::cast_slice(data.1),
        usage: wgpu::BufferUsages::INDEX,
    });

    let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Main Instance Buffer"),
        contents: bytemuck::cast_slice(data.2),
        usage: wgpu::BufferUsages::VERTEX,
    });

    (vertex_buffer, index_buffer, instance_buffer)
}

pub fn create_main_render_pipeline(
    device: &wgpu::Device,
    shader: wgpu::ShaderModule,
    config: &SurfaceConfiguration,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Main Render Pipeline Layout"),
        bind_group_layouts: &[&uniform_bind_group_layout],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Main Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[Vertex::descriptor(), InstanceData::descriptor()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Cw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0, /*All masks*/
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });
    render_pipeline
}
