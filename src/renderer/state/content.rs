use crate::renderer::formats::vertex::{InstanceData, Vertex};

pub fn get_vertices() -> (&'static [Vertex], &'static [u16], &'static [InstanceData]) {
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

    const INSTANCES: &[InstanceData] = &[
        InstanceData {
            transform: [
                [640.0, 0.0, 0.0, 0.0], // coluna 1
                [0.0, 360.0, 0.0, 0.0], // coluna 2
                [0.0, 0.0, 1.0, 0.0],   // coluna 3
                [0.0, 0.0, 0.0, 1.0],   // coluna 4
            ],
            color: [0.99609375, 0.359375, 0.78515625, 1.0],
        },
        InstanceData {
            transform: [
                [200.0, 0.0, 0.0, 0.0],
                [0.0, 200.0, 0.0, 0.0],
                [0.0, 0.0, 16.0, 0.0],
                [250.0, 0.0, 0.0, 1.0],
            ],
            color: [0.2578125, 0.79296875, 0.9570312, 1.0],
        },
    ];

    (VERTICES, INDICES, INSTANCES)
}
