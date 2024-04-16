#![allow(unused, dead_code, non_snake_case)]

use std::{error::Error, sync::{Arc, Mutex}};
use ui_composer::{app::{UIApp, UIAppCreateDescriptor}, renderer::formats::vertex::InstanceData, ui::render::PrimitiveRenderModule};
use wgpu::Color;

struct MyState {
    pub counter: i32,
}

const TEST_FONT: &[u8; 273900] = include_bytes!("../assets/fonts/JetBrainsMono-Regular.ttf");
const TEST_FONT2: &[u8; 15920] = include_bytes!("../assets/fonts/Nayten Sans.ttf");

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let initial_state = MyState { counter: 0 };

    let mut ui_app = UIApp::new(
        initial_state,
        UIAppCreateDescriptor {
            initial_window_title: "Counter",
            initial_window_size: (320, 480),
        },
    )
    .await
    .expect("Could not build app for whatever reason.");

    let mut primitive_module = Box::new(PrimitiveRenderModule::new(&ui_app.get_render_engine().device));
    primitive_module.push_raw_primitives(&ui_app.get_render_engine().queue, &vec![
        InstanceData {
            transform: [
                [10.0, 0.0, 0.0, 0.0],
                [0.0, 10.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            color: [0.0, 0.6, 1.0, 1.0],
        }
    ]);
    ui_app.add_render_module(primitive_module);

    //ui_app.add_text_rendering_engine();
    //ui_app.load_font_data( TEST_FONT.into() );

    ui_app.run().await?;

    Ok(())
}
