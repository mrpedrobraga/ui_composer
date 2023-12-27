use std::error::Error;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

/// Wrapper responsible for holding / handling the program's user interfac
/// and broadcasting events to the underlying API.
struct ProgramState {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    window_size: winit::dpi::PhysicalSize<u32>,
    // Must be dropped *after* `self::surface`.
    // Since the surface refers to it in spite of
    // the borrow checker.
    window: Window,
}

impl ProgramState {
    async fn new(window: Window) -> Result<Self, Box<dyn Error>> {
        let window_size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Unsafe: `surface` has a reference to resources from `window`
        // such that `surface` can't be freed before `window` does.
        let surface = unsafe { instance.create_surface(&window) }?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await;
        let adapter = match adapter {
            Some(v) => v,
            None => todo!(),
        };

        dbg!(adapter.get_info());

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

        let surface_capabilities = surface.get_capabilities(&adapter);

        // Assuming sRGB for now...
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_capabilities.formats[0]);

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

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            window_size,
        })
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn resize_window(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if !(new_size.width > 0 && new_size.height > 0) {
            return;
        }

        self.window_size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config)
    }

    /// Returns 'true' if the input was handled successfully.
    fn handle_input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let render_target = self.surface.get_current_texture()?;
        let view = render_target
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut cmd_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let render_pass = cmd_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

        drop(render_pass);

        self.queue.submit(std::iter::once(cmd_encoder.finish()));
        render_target.present();

        Ok(())
    }
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Overtone")
        .build(&event_loop)?;
    let mut state = ProgramState::new(window).await?;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window().id() => {
            if !state.handle_input(event) {
                match event {
                    WindowEvent::CloseRequested {} => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => state.resize_window(*physical_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize_window(**new_inner_size)
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == state.window().id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                // Reconfigure surface if it gets lost by calling `resize`.
                Err(wgpu::SurfaceError::Lost) => state.resize_window(state.window_size),
                // Perhaps this can be better handled?
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => state.window.request_redraw(),
        _ => {}
    })
}
