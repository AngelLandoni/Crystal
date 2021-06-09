use cgmath::{Vector3, Vector4};

use bytemuck::{Pod, Zeroable};

/// Represents a vertex in a 3D space.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    /// Position of the Vertex in the 3D space.
    pub pos: Vector4<f32>,

    // /// Position of the UV coordinate in 2D space.
    // pub uv: [f32; 2]
}

/// Contains all the basic functions available for the Vertex.
impl Vertex {
    /// Creates a new Vertex.
    pub fn new(pos: Vector3<f32>, uv: [f32; 2]) -> Vertex {
        Vertex {
            pos: Vector4 {
                x: pos.x,
                y: pos.y,
                z: pos.z,
                w: 1.0
            }
        }
    }
}

unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}