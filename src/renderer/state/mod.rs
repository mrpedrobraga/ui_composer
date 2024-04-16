pub mod content;
pub mod helpers;

use content::*;
use helpers::*;
use std::error::Error;

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

/// Wrapper responsible for holding / handling the program's user interfac
/// and broadcasting events to the underlying API.
pub struct RenderingEngine {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub render_pipeline: wgpu::RenderPipeline,
    pub global_uniforms: ProgramUniforms,
    pub global_uniform_bind_group: wgpu::BindGroup,
    pub global_uniform_buffer: wgpu::Buffer,
    pub prim_vertex_buffer: wgpu::Buffer,
    pub prim_index_buffer: wgpu::Buffer,
    pub prim_instance_buffer: wgpu::Buffer,
    pub prim_instances: Vec<InstanceData>,
    pub prim_mesh: (&'static [Vertex], &'static [u16]),

    // Text Renderer module for rendering rich text.
    // TODO: Inline layout might use text? Idk?
    pub text_renderer: TextRenderer,

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
        let primitive_mesh = get_quad_mesh();
        let (vertex_buffer, index_buffer, instance_buffer) =
            create_primitive_mesh_buffers(&primitive_mesh, &device);
        let render_pipeline =
            create_main_render_pipeline(&device, shader, &config, uniform_bind_group_layout);

        let text_renderer = TextRenderer::new(&device, &queue, surface_format);

        let prim_instances = Vec::new();

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
            prim_vertex_buffer: vertex_buffer,
            prim_index_buffer: index_buffer,
            prim_instance_buffer: instance_buffer,
            prim_mesh: primitive_mesh,
            text_renderer,
            prim_instances
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
            WindowEvent::CloseRequested {} => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => self.resize_window(*physical_size),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.resize_window(**new_inner_size)
            }
            _ => return false,
        }

        true
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

    pub fn push_raw_primitives(&mut self, primitive_instances: &Vec<InstanceData>) {
        self.prim_instances.clear();
        self.prim_instances.clone_from(&primitive_instances);
        
        self.queue.write_buffer(&self.prim_instance_buffer, 0, bytemuck::cast_slice(&self.prim_instances[..]));
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

        /* Render all the basic primitives on a single draw call. */
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.global_uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.prim_vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.prim_instance_buffer.slice(..));
        render_pass.set_index_buffer(self.prim_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        // TODO: Render spans of primitives instead of all of them.
        render_pass.draw_indexed(0..(self.prim_mesh.1.len() as _), 0, 0..(self.prim_instances.len() as _));

        /* Here, many other things can be plugged in to draw */
        // TODO: Handle text rendering error!
        // CLIP

        self.text_renderer.prepare(
            &self.queue,
            &self.device,
            &self.surface_config,
            self.window_size.width, self.window_size.height
        ).expect("Could not prepare text???");
        self.text_renderer.render(&mut render_pass).expect("Could not render text???");
        /* Pluging space end */

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
