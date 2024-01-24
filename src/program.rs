use std::error::Error;
use winit::{dpi::LogicalSize, event::Event, event_loop::EventLoop, window::WindowBuilder};

use crate::renderer::state::ProgramRenderingState;

const DEFAULT_WINDOW_TITLE: &'static str = "";
const DEFAULT_WINDOW_SIZE: (i32, i32) = (640, 360);

pub async fn run() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();

    /* Render State */
    let window = make_main_window().build(&event_loop)?;
    let mut render_state = ProgramRenderingState::new(window).await?;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } => {
            if window_id == render_state.window().id() {
                let _ = render_state.handle_input(event, control_flow);
            }
        }

        Event::RedrawRequested(window_id) if window_id == render_state.window().id() => {
            render_state.request_redraw(control_flow);
        }

        //Event::MainEventsCleared => render_state.request_window_redraw(),
        _ => {}
    })
}

fn make_main_window() -> WindowBuilder {
    WindowBuilder::new()
        .with_title(DEFAULT_WINDOW_TITLE)
        .with_inner_size(LogicalSize {
            width: DEFAULT_WINDOW_SIZE.0,
            height: DEFAULT_WINDOW_SIZE.1,
        })
}
