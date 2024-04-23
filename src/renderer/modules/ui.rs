use crate::{app::UIApp, renderer::{
    engine::{self, render_engine::{RenderingEngine, RenderingEngineGPU}, render_module::RenderModule},
    formats::vertex::{InstanceData, Vertex},
}};
use wgpu::{util::DeviceExt, SurfaceConfiguration};

pub struct PrimitiveRenderModule {
    render_pipeline: wgpu::RenderPipeline,
    primitive_mesh: Mesh2D<'static>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    instances: Vec<InstanceData>,
    uniform_bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
    uniforms: PrimitiveRenderModuleUniforms,
}

impl PrimitiveRenderModule {
    pub fn new<T>(app: &UIApp<T>) -> Self {
        let gpu = &app.get_render_engine().gpu;
        let primitive_mesh = get_quad_mesh();
        let (vertex_buffer, index_buffer, instance_buffer) =
            create_primitive_mesh_buffers(&primitive_mesh, &gpu.device);
        let instances = Vec::new();
        let uniforms = PrimitiveRenderModuleUniforms::default();
        let uniform_buffer = create_uniform_buffer(&uniforms, &gpu.device);
        let uniform_bind_group_layout = create_uniform_bind_group_layout(&gpu.device);
        let uniform_bind_group =
            create_uniform_bind_group(&uniform_bind_group_layout, &uniform_buffer, &gpu.device);
        let shader_descriptor = get_main_shader();
        let shader = gpu.device.create_shader_module(get_main_shader());
        let render_pipeline =
            create_main_render_pipeline(&gpu.device, shader, &gpu.surface_config, uniform_bind_group_layout);

        Self {
            render_pipeline,
            primitive_mesh,
            vertex_buffer,
            index_buffer,
            instance_buffer,
            instances,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    pub fn push_raw_primitives(
        &mut self,
        gpu: &RenderingEngineGPU,
        primitive_instances: &Vec<InstanceData>,
    ) {
        self.instances.clear();
        self.instances.clone_from(&primitive_instances);

        gpu.queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&self.instances[..]),
        );
    }
}

impl RenderModule for PrimitiveRenderModule {
    fn prepare_to_render(&mut self, engine: &RenderingEngineGPU) {
        self.uniforms.window_size = calc_px_to_wgpu_matrix(
            engine.window_size.width as f32,
            engine.window_size.height as f32
        );

        engine.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }

    fn commit_render<'pass>(
        &'pass self,
        render_pass: &mut wgpu::RenderPass<'pass>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

        render_pass.draw_indexed(
            0..(self.primitive_mesh.1.len() as _),
            0,
            0..(self.instances.len() as _),
        );

        Ok(())
    }
}

type Mesh2D<'a> = (&'a [Vertex], &'a [u16]);

pub fn get_quad_mesh() -> Mesh2D<'static> {
    const VERTICES: &[Vertex] = &[
        Vertex {
            position: [0.0, 0.0, 0.0],
            uv: [0.0, 0.0],
        },
        Vertex {
            position: [1.0, 0.0, 0.0],
            uv: [1.0, 0.0],
        },
        Vertex {
            position: [1.0, 1.0, 0.0],
            uv: [1.0, 1.0],
        },
        Vertex {
            position: [0.0, 1.0, 0.0],
            uv: [0.0, 1.0],
        },
    ];

    const INDICES: &[u16] = &[0, 1, 2, 3, 0, 2];

    (VERTICES, INDICES)
}

pub fn to_linear_rgb(c: u32) -> [f32; 4] {
    let f = |xu: u32| {
        let x = (xu & 0xFF) as f32 / 255.0;
        if x > 0.04045 {
            ((x + 0.055) / 1.055).powf(2.4)
        } else {
            x / 12.92
        }
    };
    [f(c >> 16), f(c >> 8), f(c), 1.0]
}

/** Converts from px to wgpu matrix. */
pub fn calc_px_to_wgpu_matrix(width: f32, height: f32) -> [[f32; 4]; 4] {
    return [
        [2.0 / width, 0.0, 0.0, 0.0],
        [0.0, -2.0 / height, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-1.0, 1.0, 0.0, 1.0],
    ];
}

pub fn get_main_shader() -> wgpu::ShaderModuleDescriptor<'static> {
    wgpu::ShaderModuleDescriptor {
        label: Some("Main Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("./ui_primitives.wgsl").into()),
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PrimitiveRenderModuleUniforms {
    pub window_size: [[f32; 4]; 4],
    pub camera_position: [f32; 4],
}

impl Default for PrimitiveRenderModuleUniforms {
    fn default() -> Self {
        Self {
            window_size: Default::default(),
            camera_position: Default::default(),
        }
    }
}

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
        label: Some("Primitive Uniform Bind Group"),
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
        label: Some("Primitive Uniform Bind Group Layout"),
    });

    layout
}

pub fn create_uniform_buffer(
    uniforms: &PrimitiveRenderModuleUniforms,
    device: &wgpu::Device,
) -> wgpu::Buffer {
    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Primitive Index Buffer"),
        contents: bytemuck::cast_slice(&[*uniforms]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    uniform_buffer
}

pub fn create_primitive_mesh_buffers(
    data: &Mesh2D,
    device: &wgpu::Device,
) -> (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer) {
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Primitive Vertex Buffer"),
        contents: bytemuck::cast_slice(data.0),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Primitive Index Buffer"),
        contents: bytemuck::cast_slice(data.1),
        usage: wgpu::BufferUsages::INDEX,
    });

    let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Primitive Instance Buffer"),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        size: 1048576,
        mapped_at_creation: false,
    });

    //.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //    label: Some("Primitive Instance Buffer"),
    //    contents: bytemuck::cast_slice::<InstanceData, _>(&[]),
    //    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    //});

    (vertex_buffer, index_buffer, instance_buffer)
}

pub fn create_main_render_pipeline(
    device: &wgpu::Device,
    shader: wgpu::ShaderModule,
    config: &SurfaceConfiguration,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Primitive Render Pipeline Layout"),
        bind_group_layouts: &[&uniform_bind_group_layout],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Primitive Render Pipeline"),
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
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default()
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0, /*All masks*/
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });
    render_pipeline
}
