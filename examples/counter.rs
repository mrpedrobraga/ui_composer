#![allow(dead_code, non_snake_case)]

use std::error::Error;
use ui_composer::prelude::*;
use ui_composer::renderer::modules::ui::to_linear_rgb;
use ui_composer::renderer::{
    formats::vertex::InstanceData,
    modules::ui::PrimitiveRenderModule,
};

struct MyState {
    pub counter: i32,
}

const TEST_FONT: &[u8; 273900] = include_bytes!("../assets/fonts/JetBrainsMono-Regular.ttf");
const TEST_FONT2: &[u8; 15920] = include_bytes!("../assets/fonts/Nayten Sans.ttf");

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let initial_state = MyState { counter: 0 };

    let mut app = UIAppBuilder::new(initial_state)
        .with_window_title("My Window".to_owned())
        .with_window_size((300, 300))
        .build()
        .await?;

    let mut primitive_module = Box::new(PrimitiveRenderModule::new(&app));

    // TODO: It should be possible to communicate with this module 
    // from some higher level API.
    primitive_module.push_raw_primitives(
        &app.get_render_engine().gpu,
        &get_test_instance_data()
    );
    
    app.add_render_module(primitive_module);

    app.run().await?;

    Ok(())
}

fn get_test_instance_data() -> Vec<InstanceData> {
    vec![
        InstanceData {
            transform: rect([0.0, 0.0, 0.999], [300.0, 300.0]),
            color: to_linear_rgb(0xdedede),
        },
        InstanceData {
            transform: rect([(300.0-96.0-4.0)/2.0, 300.0-32.0-16.0+4.0, 0.91], [96.0+4.0, 32.0]),
            color: to_linear_rgb(0xa0a0a0),
        },
        InstanceData {
            transform: rect([(300.0-96.0)/2.0, 300.0-32.0-16.0, 0.9], [96.0, 32.0]),
            color: to_linear_rgb(0xee2244),
        },
    ]
}

fn rect(position: [f32; 3], size: [f32; 2]) -> [[f32; 4]; 4] {
    [
        [size[0], 0.0, 0.0, 0.0],
        [0.0, size[1], 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [position[0], position[1], position[2], 1.0]
    ]
}
