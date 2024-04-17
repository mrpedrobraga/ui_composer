use std::error::Error;

use winit::{
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use crate::renderer::device::*;

use super::render_module::RenderModule;

pub type SharedRenderModule = Box<dyn RenderModule>;

/// Wrapper responsible for holding/handling the program's user interface primitives
/// and broadcasting events to the underlying rendering API.
pub struct RenderingEngine {
    pub render_modules: Vec<SharedRenderModule>,
    pub gpu: RenderingEngineGPU,
}

pub struct RenderingEngineGPU {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface,
    pub surface_config: wgpu::SurfaceConfiguration,
    // Must be dropped *after* `self::surface`.
    // Since the surface refers to it in spite of
    // the borrow checker.
    pub window: Window,
    pub window_size: winit::dpi::PhysicalSize<u32>,
}

impl RenderingEngine {
    pub async fn new(window: Window) -> Result<Self, Box<dyn Error>> {
        let window_size = window.inner_size();
        let instance = create_instance();
        let surface = create_surface(&instance, &window)?;
        let adapter = get_adapter(instance, &surface).await;
        let (device, queue) = get_device(&adapter).await?;
        let surface_capabilities = surface.get_capabilities(&adapter);
        // Assuming sRGB for now...
        let surface_format = get_surface_format(&surface_capabilities);
        let config =
            get_default_surface_configuration(surface_format, window_size, surface_capabilities);
        let render_modules = Vec::new();

        Ok(Self {
            gpu: RenderingEngineGPU {
                window,
                surface,
                device,
                queue,
                surface_config: config,
                window_size,
            },
            render_modules,
        })
    }

    pub fn window(&self) -> &Window {
        &self.gpu.window
    }

    pub fn reconfigure_surface(&mut self) {
        self.gpu.surface.configure(&self.gpu.device, &self.gpu.surface_config)
    }

    pub fn resize_window(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if !(new_size.width > 0 && new_size.height > 0) {
            return;
        }

        self.gpu.window_size = new_size;
        self.gpu.surface_config.width = new_size.width;
        self.gpu.surface_config.height = new_size.height;

        self.reconfigure_surface()
    }

    pub fn request_window_redraw(&mut self) {
        self.gpu.window.request_redraw()
    }

    pub fn handle_input(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) -> bool {
        match event {
            _ => return false,
        }

        true
    }

    pub fn add_render_module(&mut self, render_module: SharedRenderModule) {
        self.render_modules.push(render_module);
    }

    /** Updates the engine state and rerenders it to screen. */
    pub fn update(&mut self) {
        let _ = self.render();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // Render to the current texture.
        let render_target = self.gpu.surface.get_current_texture()?;
        let view = render_target
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut cmd_encoder = self.gpu.device
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
                        r: 0.02,
                        g: 0.02,
                        b: 0.02,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        // /* Render all the basic primitives on a single draw call. */
        // render_pass.set_pipeline(&self.render_pipeline);
        // render_pass.set_bind_group(0, &self.global_uniform_bind_group, &[]);

        for module in self.render_modules.iter_mut() {
            module.prepare(&self.gpu);
            module.render(&mut render_pass);
        }

        drop(render_pass);
        self.gpu.queue.submit(std::iter::once(cmd_encoder.finish()));

        // Present the final result to the screen.
        // TODO: Maybe in case of partial rendering it won't present to the screen.
        render_target.present();
        Ok(())
    }

    pub fn request_redraw(&mut self, control_flow: &mut ControlFlow) {
        match self.render() {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost) => self.reconfigure_surface(),
            // Perhaps this can be better handled?
            Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
