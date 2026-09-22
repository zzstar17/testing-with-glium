use glium::{Display, VertexBuffer};

use crate::shaders::common::Vertex2d;

pub struct MainFramebufferShader {
    pub vertex_buffer: VertexBuffer<Vertex2d>,
    pub index_buffer: glium::index::NoIndices,
}

impl MainFramebufferShader {
    pub fn new(display: &Display) -> Self {
        Self {
            vertex_buffer: Self::create_vertex_buffer(display),
            index_buffer: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        }
    }

    fn create_vertex_buffer(display: &Display) -> VertexBuffer<Vertex2d> {
        let shape = [
            Vertex2d {
                position: [-1.0, 1.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex2d {
                position: [-1.0, -1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex2d {
                position: [1.0, -1.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex2d {
                position: [-1.0, 1.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex2d {
                position: [1.0, -1.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex2d {
                position: [1.0, 1.0],
                tex_coords: [1.0, 1.0],
            },
        ];
        VertexBuffer::new(display, &shape).unwrap()
    }
}
