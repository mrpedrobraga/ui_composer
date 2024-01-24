use crate::renderer::formats::Vertex;


pub fn get_vertices() -> (&'static [Vertex], &'static [u16]) {
    const VERTICES: &[Vertex] = &[
        Vertex {
            position: [-0.5, 0.5, 0.0],
            color: [0.0, 0.0, 0.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.0],
            color: [0.0, 1.0, 0.0, 1.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
            color: [1.0, 1.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.5, 0.5, 0.0],
            color: [1.0, 0.0, 0.0, 0.0],
        },
    ];

    const INDICES: &[u16] = &[0, 1, 2, 3, 0, 2];

    (VERTICES, INDICES)
}
