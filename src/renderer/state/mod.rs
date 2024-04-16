pub mod content;
pub mod render_module;
pub mod helpers;

use content::*;
use helpers::*;
use std::{borrow::Borrow, error::Error, sync::{Arc, Mutex}};

use crate::ui::render::{calc_px_to_wgpu_matrix, create_main_render_pipeline, create_uniform_bind_group, create_uniform_bind_group_layout, create_uniform_buffer};

use self::render_module::RenderModule;

use super::{
    device::{
        create_instance, create_surface, get_adapter, get_default_surface_configuration,
        get_device, get_surface_format,
    }, formats::vertex::{InstanceData, Vertex}, main_shader::{get_main_shader, ProgramUniforms}, text::TextRenderer
};
use winit::{
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

pub type SharedRenderModule = Box<dyn RenderModule>;

/// Wrapper responsible for holding/handling the program's user interface primitives
/// and broadcasting events to the underlying rendering API.
pub struct RenderingEngine {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub render_pipeline: wgpu::RenderPipeline,
    pub global_uniforms: ProgramUniforms,
    pub global_uniform_bind_group: wgpu::BindGroup,
    pub global_uniform_buffer: wgpu::Buffer,

    pub render_modules: Vec<SharedRenderModule>,

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
        let shader = device.create_shader_module(get_main_shader());
        let uniforms = ProgramUniforms::default();
        let uniform_buffer = create_uniform_buffer(&uniforms, &device);
        let uniform_bind_group_layout = create_uniform_bind_group_layout(&device);
        let uniform_bind_group =
            create_uniform_bind_group(&uniform_bind_group_layout, &uniform_buffer, &device);
        
        let render_pipeline =
            create_main_render_pipeline(&device, shader, &config, uniform_bind_group_layout);

        let render_modules = Vec::new();

        Ok(Self {
            window,
            surface,
            device,
            queue,
            surface_config: config,
            window_size,
            render_pipeline,
            global_uniforms: uniforms,
            global_uniform_bind_group: uniform_bind_group,
            global_uniform_buffer: uniform_buffer,
            render_modules,
        })
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn reconfigure_surface(&mut self) {
        self.surface.configure(&self.device, &self.surface_config)
    }

    pub fn resize_window(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if !(new_size.width > 0 && new_size.height > 0) {
            return;
        }

        self.window_size = new_size;
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;

        self.reconfigure_surface()
    }

    pub fn request_window_redraw(&mut self) {
        self.window.request_redraw()
    }

    pub fn handle_input(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) -> bool {
        match event {
            
            _ => return false,
        }

        true
    }

    pub fn add_render_module (&mut self, render_module: SharedRenderModule) {
        self.render_modules.push(render_module);
    }

    /** Sends the uniform to the GPU. */
    fn send_uniforms(&mut self) {
        self.queue.write_buffer(
            &self.global_uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.global_uniforms]),
        );
    }

    /** Updates the engine state and rerenders it to screen. */
    pub fn update(&mut self) {
        let _ = self.render();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // Update all the render-related uniforms.
        self.global_uniforms.window_size = calc_px_to_wgpu_matrix(
            self.window_size.width as f32,
            self.window_size.height as f32
        );
        self.send_uniforms();

        // Render to the current texture.
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
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.global_uniform_bind_group, &[]);
        
        for module in &self.render_modules {
            module.render(&mut render_pass);
        }

        drop(render_pass);
        self.queue.submit(std::iter::once(cmd_encoder.finish()));

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
