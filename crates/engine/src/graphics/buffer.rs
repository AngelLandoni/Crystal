use bytemuck::{Pod, Zeroable};
use wgpu::Buffer;

use types::Bytes;

/// Represents a buffer that could be submited to the GPU. 
pub trait RawBufferRepresentable {
    /// Should return the raw data for the 
    fn get_raw<'a>(&'a self) -> Bytes<'a>;
}

/// Implement `RawBufferRepresentable` for all the Vectors.
impl<T: bytemuck::Pod> RawBufferRepresentable for Vec<T> {
    /// Maps the content of a Vec to an array of Bytes.
    fn get_raw<'a>(&'a self) -> Bytes<'a> {
        Bytes(bytemuck::cast_slice(self))
    } 
}

/// Represents a resource that could be submited to the GPU.
///
/// TODO: Add uniforms and some other BufferUsage.
/// https://docs.rs/wgpu/0.6.0/wgpu/struct.BufferUsage.html
pub trait BufferCreator {
    /// Should create and return the created vertex buffer.
    fn create_vertex<T: RawBufferRepresentable>(&self, data: T) -> Buffer; 
   
    /// Should create a new buffer with the size provided.
    ///
    /// This should only receive and hold data, could not be used as source.
    fn create_vertex_with_size(&self, size: u64) -> Buffer;

    /// Should create and return the created index buffer.
    fn create_index<T: RawBufferRepresentable>(&self, data: T) -> Buffer;
    
    /// Should create and return a new uniform buffer.
    fn create_uniform<T: RawBufferRepresentable>(&self, data: T) -> Buffer;
}

/// Represents an instance that could manipulate GPU buffers.
pub trait BufferManipulator {
    /// Should copy the data to the buffer provided.
    fn copy_to_buffer(&self, buffer: &Buffer, data: &[u8]);
}
