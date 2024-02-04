use glyphon::cosmic_text::rustybuzz::ttf_parser::Width;
use rand::{Rng, RngCore};

use crate::renderer::formats::vertex::{InstanceData, Vertex};

pub fn get_vertices() -> (&'static [Vertex], &'static [u16], Vec<InstanceData>) {
    const VERTICES: &[Vertex] = &[
        Vertex {
            position: [0.0, 0.0, 0.0],
            color: [0.0, 0.0, 0.0, 1.0],
        },
        Vertex {
            position: [1.0, 0.0, 0.0],
            color: [0.0, 1.0, 0.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0, 0.0],
            color: [1.0, 1.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.0, 1.0, 0.0],
            color: [1.0, 0.0, 0.0, 0.0],
        },
    ];

    const INDICES: &[u16] = &[0, 1, 2, 3, 0, 2];

    // const INSTANCES: &[InstanceData] = &[
    //     InstanceData {
    //         transform: [
    //             [640.0, 0.0, 0.0, 0.0], // coluna 1
    //             [0.0, 360.0, 0.0, 0.0], // coluna 2
    //             [0.0, 0.0, 1.0, 0.0],   // coluna 3
    //             [0.0, 0.0, 0.0, 1.0],   // coluna 4
    //         ],
    //         color: [0.99609375, 0.359375, 0.78515625, 1.0],
    //     },
    //     InstanceData {
    //         transform: [
    //             [200.0, 0.0, 0.0, 0.0],
    //             [0.0, 200.0, 0.0, 0.0],
    //             [0.0, 0.0, 16.0, 0.0],
    //             [250.0, 0.0, 0.0, 1.0],
    //         ],
    //         color: [0.2578125, 0.79296875, 0.9570312, 1.0],
    //     },
    // ];
    let instances: Vec<InstanceData> = (0..1)
        .flat_map(|_| {
            (0..10).flat_map(|i_y| {
                (0..19).map(move |i_x| {
                    let mut r = rand::thread_rng();
                    let width = 10.0;
                    let height = 10.0;
                    let x = 16.0 * i_x as f32;
                    let y = 16.0 * i_y as f32;

                    let red = r.gen::<f32>();
                    let gre = r.gen::<f32>();
                    let blu = r.gen::<f32>();

                    InstanceData {
                        transform: [
                            [width, 0.0, 0.0, 0.0],
                            [0.0, height, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [x, y, 0.0, 1.0],
                        ],
                        color: [red, gre, blu, 1.0],
                    }
                })
            })
        })
        .collect::<_>();

    (VERTICES, INDICES, instances)
}
