use std::error::Error;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::renderer::state::ProgramRenderingState;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Overtone")
        .with_inner_size(LogicalSize {
            width: 640,
            height: 360,
        })
        .build(&event_loop)?;
    let mut render_state = ProgramRenderingState::new(window).await?;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == render_state.window().id() => {
            // Remove this: Render state shouldn't handle input.
            if !render_state.handle_input(event) {
                match event {
                    WindowEvent::CloseRequested {} => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        render_state.resize_window(*physical_size)
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        render_state.resize_window(**new_inner_size)
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == render_state.window().id() => {
            //state.update();
            match render_state.render() {
                Ok(_) => {}
                // Reconfigure surface if it gets lost by calling `resize`.
                Err(wgpu::SurfaceError::Lost) => {
                    render_state.resize_window(render_state.window_size)
                }
                // Perhaps this can be better handled?
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => render_state.window.request_redraw(),
        _ => {}
    })
}
