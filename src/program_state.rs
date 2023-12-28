use std::error::Error;

use wgpu::{util::DeviceExt, SurfaceConfiguration};
use winit::{event::WindowEvent, window::Window};

use crate::shaders::formats::Vertex;

pub const MAIN_SHADER: &'static str = include_str!("./shaders/main.wgsl");

/// Wrapper responsible for holding / handling the program's user interfac
/// and broadcasting events to the underlying API.
pub struct ProgramState {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,

    // Must be dropped *after* `self::surface`.
    // Since the surface refers to it in spite of
    // the borrow checker.
    pub window: Window,
    pub window_size: winit::dpi::PhysicalSize<u32>,
}

impl ProgramState {
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

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Main Shader"),
            source: wgpu::ShaderSource::Wgsl(MAIN_SHADER.into()),
        });
        let render_pipeline = create_main_render_pipeline(&device, shader, &config);

        const VERTICES: &[Vertex] = &[];
        let vertex_buffer = create_vertex_buffer(VERTICES, &device);

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            window_size,
            render_pipeline,
            vertex_buffer,
        })
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize_window(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if !(new_size.width > 0 && new_size.height > 0) {
            return;
        }

        self.window_size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config)
    }

    /// Returns 'true' if the input was handled successfully.
    pub fn handle_input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

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
        render_pass.draw(0..3, 0..1);

        drop(render_pass);

        self.queue.submit(std::iter::once(cmd_encoder.finish()));
        render_target.present();

        Ok(())
    }
}

fn create_vertex_buffer(vertices: &[Vertex], device: &wgpu::Device) -> wgpu::Buffer {
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Main Vertex Buffer"),
        contents: bytemuck::cast_slice(vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });
    vertex_buffer
}

fn create_main_render_pipeline(
    device: &wgpu::Device,
    shader: wgpu::ShaderModule,
    config: &SurfaceConfiguration,
) -> wgpu::RenderPipeline {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Main Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Main Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
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

fn get_default_surface_configuration(
    surface_format: wgpu::TextureFormat,
    window_size: winit::dpi::PhysicalSize<u32>,
    surface_capabilities: wgpu::SurfaceCapabilities,
) -> SurfaceConfiguration {
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        // INFO: `width` and `height` can never be 0, otherwise the program
        // might crash unexpectedly.
        width: window_size.width,
        height: window_size.height,
        // TODO: This will be choosable by the user futurely.
        present_mode: surface_capabilities.present_modes[0],
        alpha_mode: surface_capabilities.alpha_modes[0],
        view_formats: Vec::new(),
    };
    config
}

fn get_surface_format(surface_capabilities: &wgpu::SurfaceCapabilities) -> wgpu::TextureFormat {
    let surface_format = surface_capabilities
        .formats
        .iter()
        .copied()
        .filter(|f| f.is_srgb())
        .next()
        .unwrap_or(surface_capabilities.formats[0]);
    surface_format
}

async fn get_device(
    adapter: &wgpu::Adapter,
) -> Result<(wgpu::Device, wgpu::Queue), Box<dyn Error>> {
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        )
        .await?;
    Ok((device, queue))
}

async fn get_adapter(instance: wgpu::Instance, surface: &wgpu::Surface) -> wgpu::Adapter {
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(surface),
        })
        .await;
    let adapter = match adapter {
        Some(v) => v,
        None => todo!(),
    };
    adapter
}

fn create_surface(
    instance: &wgpu::Instance,
    window: &Window,
) -> Result<wgpu::Surface, Box<dyn Error>> {
    let surface = unsafe { instance.create_surface(window) }?;
    Ok(surface)
}

fn create_instance() -> wgpu::Instance {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });
    instance
}
