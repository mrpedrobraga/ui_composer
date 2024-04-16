use std::{error::Error, rc::Rc, sync::{Arc, Mutex}};

use ui_composer::{
    renderer::{
        formats::vertex::InstanceData,
        state::{content::to_linear_rgb, RenderingEngine},
    },
    ui::render,
};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, ScanCode, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

const INITIAL_WINDOW_TITLE: &'static str = "UI Composer App";
const INITIAL_WINDOW_SIZE: (i32, i32) = (640, 360);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run().await?;
    Ok(())
}

fn make_main_window() -> WindowBuilder {
    WindowBuilder::new()
        .with_title(INITIAL_WINDOW_TITLE)
        .with_inner_size(LogicalSize {
            width: INITIAL_WINDOW_SIZE.0,
            height: INITIAL_WINDOW_SIZE.1,
        })
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();
    let window = make_main_window().build(&event_loop)?;
    let mut render_state = RenderingEngine::new(window).await?;

    let u = Arc::new(Mutex::new(0.0));

    let uu = u.clone();
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } => {
            if window_id == render_state.window().id() {
                let _ = render_state.handle_input(event, control_flow);

                match event {
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Space),
                                ..
                            },
                        ..
                    } => {
                        let mut l = uu.lock().expect("Could not lock u mutex");
                        *l += 1.0;
                        render_state.push_raw_primitives(&get_instances(*l));
                        render_state.request_redraw(control_flow);
                    },
                    _ => {}
                }
            }
        }

        // Redraws the screen when a redraw is requested by the OS.
        // (Like, for example, when the screen is reshaped)
        Event::RedrawRequested(window_id) if window_id == render_state.window().id() => {
            render_state.request_redraw(control_flow);
        }

        _ => {}
    })
}

fn rect(position: (f32, f32), size: (f32, f32), color: [f32; 4]) -> InstanceData {
    InstanceData {
        transform: [
            [size.0, 0.0, 0.0, 0.0],
            [0.0, size.1, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [position.0, position.1, 0.0, 1.0],
        ],
        color,
    }
}

fn get_instances(u: f32) -> Vec<InstanceData> {
    vec![
        rect((0.0, 0.0), (640.0, 360.0), to_linear_rgb(0x101010)),
        rect(
            ((640.0 / 2.0) - 48.0 + 10.0 * u, 360.0 - 64.0),
            (96.0, 32.0),
            to_linear_rgb(0x00aaff),
        ),
    ]
}