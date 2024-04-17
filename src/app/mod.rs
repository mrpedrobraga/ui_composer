use crate::renderer::engine::{render_module::RenderModule, render_engine::{RenderingEngine, SharedRenderModule}};
use std::{error::Error, sync::{Arc, Mutex}};
use winit::{event::WindowEvent, event_loop::EventLoop, window::Window};

/// A user interface app, everything necessary for rendering UI from state.
pub struct UIApp<TState> {
    pub state: TState,
    event_loop: EventLoop<()>,
    render_engine: RenderingEngine,
}

/// Descriptor for creating a new UI App.
pub struct UIAppCreateDescriptor {
    pub initial_window_title: &'static str,
    pub initial_window_size: (i32, i32),
}

impl Default for UIAppCreateDescriptor {
    fn default() -> Self {
        Self {
            initial_window_title: "UI Composer App",
            initial_window_size: (640, 360),
        }
    }
}

impl<TState> UIApp<TState> {
    /// Creates a new ui App, with some initial state.
    pub async fn new(
        initial_state: TState,
        descriptor: UIAppCreateDescriptor,
    ) -> Result<Self, Box<dyn Error>> {
        let event_loop = EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title(descriptor.initial_window_title)
            .with_inner_size(winit::dpi::LogicalSize {
                width: descriptor.initial_window_size.0,
                height: descriptor.initial_window_size.1,
            })
            .build(&event_loop)?;
        let render_engine = RenderingEngine::new(window).await?;

        Ok(Self {
            state: initial_state,
            event_loop,
            render_engine,
        })
    }

    pub fn add_render_module(&mut self, primitive_module: SharedRenderModule) {
        self.render_engine.add_render_module(primitive_module);
    }

    /// Loads font data from a buffer into the text rendering engine.
    pub fn load_font_data(&mut self, bytes: Vec<u8>) {
        //TODO: Load font data into the inner font db.
        //Maybe this will end up being in TextRenderer?
    }

    pub fn get_render_engine(&self) -> &RenderingEngine {
        &self.render_engine
    }

    /// Takes ownership of the current app and runs it, listening for external input.
    ///
    /// At this stage, you can no longer directly interact with the app from the outside,
    /// so make sure to set all the input handlers and state you might want *inside* it.
    pub async fn run(mut self) -> Result<(), Box<dyn Error>> {
        self.event_loop
            .run(move |event, _, mut control_flow| match event {
                winit::event::Event::WindowEvent {
                    event: ref win_event,
                    window_id,
                } => {
                    let _ = self.render_engine.handle_input(win_event, control_flow);
                    handle_basic_window_events(&mut self.render_engine, win_event, control_flow);
                }

                winit::event::Event::RedrawRequested(window_id) => {
                    self.render_engine.request_redraw(&mut control_flow)
                }

                _ => {}
            });
    }    
}

fn handle_basic_window_events(render_engine: &mut RenderingEngine, win_event: &WindowEvent<'_>, control_flow: &mut winit::event_loop::ControlFlow) {
    match win_event {
        winit::event::WindowEvent::CloseRequested {} =>
            { *control_flow = winit::event_loop::ControlFlow::Exit }
        winit::event::WindowEvent::Resized(physical_size) =>
            { render_engine.resize_window(*physical_size) }
        winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } =>
            { render_engine.resize_window(**new_inner_size) }
        _ => {}
    }
}