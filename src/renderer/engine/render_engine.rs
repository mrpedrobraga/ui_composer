use std::error::Error;

use wgpu::{RenderPassDepthStencilAttachment, Texture, TextureUsages, TextureViewDescriptor};
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
    pub depth_buffer: wgpu::Texture,
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
        let surface_config =
            get_default_surface_configuration(surface_format, window_size, surface_capabilities);
        let render_modules = Vec::new();
        let depth_buffer = RenderingEngine::create_depth_texture(&device, &surface_config);

        Ok(Self {
            gpu: RenderingEngineGPU {
                window,
                surface,
                device,
                queue,
                surface_config,
                window_size,
                depth_buffer,
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

    pub fn create_depth_texture(device: &wgpu::Device, surface_config: &wgpu::SurfaceConfiguration) -> wgpu::Texture {
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Buffer"),
            size: wgpu::Extent3d {
                width: surface_config.width,
                height: surface_config.height,
                depth_or_array_layers: 1
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            view_formats: &[],
            usage: TextureUsages::RENDER_ATTACHMENT
        })
    }

    pub fn resize_window(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if !(new_size.width > 0 && new_size.height > 0) {
            return;
        }

        self.gpu.window_size = new_size;
        self.gpu.surface_config.width = new_size.width;
        self.gpu.surface_config.height = new_size.height;

        self.reconfigure_surface();
        self.gpu.depth_buffer = RenderingEngine::create_depth_texture(&self.gpu.device, &self.gpu.surface_config)
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
        let main_texture_view = render_target
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let depth_texture_view = self.gpu.depth_buffer.create_view(&wgpu::TextureViewDescriptor::default());

        let mut cmd_encoder = self.gpu.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = cmd_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &main_texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store
                }),
                stencil_ops: None
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        for module in self.render_modules.iter_mut() {
            module.prepare_to_render(&self.gpu);
            module.commit_render(&mut render_pass);
            break;
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
