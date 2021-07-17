use glium::{Display, IndexBuffer, VertexBuffer};

use crate::common::Vertex;

pub trait ShaderObject {
    fn new(display: &Display) -> Self;
    fn create_vertex_buffer(display: &Display) -> VertexBuffer<Vertex>;
    fn create_index_buffer(display: &Display) -> IndexBuffer<u8>;
}