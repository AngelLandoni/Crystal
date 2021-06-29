use cgmath::{Vector2, Vector3, Vector4};

use bytemuck::{Pod, Zeroable};

/// Represents a vertex in a 3D space.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    /// Position of the Vertex in the 3D space.
    pub position: Vector4<f32>,

    /// Color of the Vertex.
    pub color: Vector4<f32>,

    /// Position of the UV coordinate in 2D space.
    pub uv: Vector2<f32>
}

/// Contains all the basic functions available for the Vertex.
impl Vertex {
    /// Creates a new Vertex.
    pub fn new(position: Vector3<f32>, color: Vector4<f32>, uv: Vector2<f32>) -> Vertex {
        Vertex {
            position: Vector4 {
                x: position.x,
                y: position.y,
                z: position.z,
                w: 1.0
            },
            color,
            uv,
        }
    }
}

unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}