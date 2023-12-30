use std::error::Error;

use wgpu::{util::DeviceExt, SurfaceConfiguration};
use winit::{
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use crate::renderer::formats::Vertex;

use super::{
    device::{
        create_instance, create_surface, get_adapter, get_default_surface_configuration,
        get_device, get_surface_format,
    },
    main_shader::{get_main_shader, ProgramUniforms},
    text::{create_brush, get_example_test_section},
};

/// Wrapper responsible for holding / handling the program's user interfac
/// and broadcasting events to the underlying API.
pub struct ProgramRenderingState {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub render_pipeline: wgpu::RenderPipeline,
    pub text_brush: wgpu_text::TextBrush<wgpu_text::glyph_brush::ab_glyph::FontArc>,
    pub uniforms: ProgramUniforms,
    pub uniform_bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub vertex_count: u32,
    pub index_count: u32,

    // Must be dropped *after* `self::surface`.
    // Since the surface refers to it in spite of
    // the borrow checker.
    pub window: Window,
    pub window_size: winit::dpi::PhysicalSize<u32>,
}

impl ProgramRenderingState {
    pub async fn new(window: Window) -> Result<Self, Box<dyn Error>> {
        let window_size = window.inner_size();
        let instance = create_instance();
        // Unsafe: `surface` has a reference to resources from `window`
        // such that `surface` can't be freed before `window` does.
        let surface = create_surface(&instance, &window)?;
        let adapter = get_adapter(instance, &surface).await;
        let (device, queue) = get_device(&adapter).await?;
        let surface_capabilities = surface.get_capabilities(&adapter);
        // Assuming sRGB for now...
        let surface_format = get_surface_format(&surface_capabilities);
        let config =
            get_default_surface_configuration(surface_format, window_size, surface_capabilities);

        let shader = device.create_shader_module(get_main_shader());

        let uniforms = ProgramUniforms { val: 1.0 };
        let uniform_buffer = create_uniform_buffer(&uniforms, &device);
        let uniform_bind_group_layout = create_uniform_bind_group_layout(&device);
        let uniform_bind_group =
            create_uniform_bind_group(&uniform_bind_group_layout, &uniform_buffer, &device);

        let (vertex_buffer, index_buffer) =
            create_vertex_and_index_buffers(get_vertices(), &device);
        let vertex_count = get_vertices().0.len() as u32;
        let index_count = get_vertices().1.len() as u32;

        let render_pipeline =
            create_main_render_pipeline(&device, shader, &config, uniform_bind_group_layout);

        // TODO: Text brushes won't be created upon project start,
        // but when a new font is loaded.
        let text_brush = create_brush(&device, &config).unwrap();

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            window_size,
            render_pipeline,
            text_brush,
            uniforms,
            uniform_bind_group,
            uniform_buffer,
            vertex_buffer,
            index_buffer,
            vertex_count,
            index_count,
        })
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn reconfigure_surface(&mut self) {
        self.surface.configure(&self.device, &self.config)
    }

    pub fn resize_window(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if !(new_size.width > 0 && new_size.height > 0) {
            return;
        }

        self.window_size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;

        self.text_brush
            .resize_view(new_size.width as f32, new_size.height as f32, &self.queue);

        self.reconfigure_surface()
    }

    pub fn request_window_redraw(&mut self) {
        self.window.request_redraw()
    }

    /// Returns 'true' if the input was handled successfully.
    pub fn handle_input(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) -> bool {
        match event {
            WindowEvent::CloseRequested {} => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => self.resize_window(*physical_size),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.resize_window(**new_inner_size)
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    },
                ..
            } => self.update(),
            _ => return false,
        }

        true
    }

    pub fn update(&mut self) {
        self.uniforms.val = 1.0 - self.uniforms.val;
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        )
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let render_target = self.surface.get_current_texture()?;
        let view = render_target
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut cmd_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = cmd_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.3,
                        b: 0.6,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..(self.index_count as u32), 0, 0..1);

        // TODO: Remove this
        let text_section = get_example_test_section();

        self.text_brush
            .queue(&self.device, &self.queue, vec![&text_section])
            .unwrap();

        self.text_brush.draw(&mut render_pass);

        drop(render_pass);

        self.queue.submit(std::iter::once(cmd_encoder.finish()));

        render_target.present();

        Ok(())
    }
}

fn get_vertices() -> (&'static [Vertex], &'static [u16]) {
    const VERTICES: &[Vertex] = &[
        Vertex {
            position: [-0.5, 0.5, 0.0],
            color: [0.0, 0.0, 0.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.0],
            color: [0.0, 1.0, 0.0, 1.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
            color: [1.0, 1.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.5, 0.5, 0.0],
            color: [1.0, 0.0, 0.0, 0.0],
        },
    ];

    const INDICES: &[u16] = &[0, 1, 2, 3, 0, 2];

    (VERTICES, INDICES)
}

fn create_uniform_bind_group(
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

fn create_uniform_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
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

fn create_uniform_buffer(uniforms: &ProgramUniforms, device: &wgpu::Device) -> wgpu::Buffer {
    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Main Index Buffer"),
        contents: bytemuck::cast_slice(&[*uniforms]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    uniform_buffer
}

fn create_vertex_and_index_buffers(
    data: (&[Vertex], &[u16]),
    device: &wgpu::Device,
) -> (wgpu::Buffer, wgpu::Buffer) {
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

    (vertex_buffer, index_buffer)
}

fn create_main_render_pipeline(
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
            buffers: &[Vertex::descriptor()],
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
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
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
