pub mod buffer;
pub mod gpu;
pub mod pipelines;
pub mod renderers;
pub mod shaders;
pub mod texture;
pub mod vertex;

extern crate crossbeam_queue;

use wgpu::CommandBuffer;
use crossbeam_queue::ArrayQueue;

/// Defines the maximun number of commands per wgpu draw call (command 
/// submition).
pub const MAX_NUMBER_OF_COMMANDS_PER_CALL: usize = 50; 

/// Defines a render ordered command buffer.
///
/// This struct only wraps an Wgpu CommandBuffer along with a order integer
/// to know which position should take in the commander buffer submition 
/// process.
pub struct OrderedCommandBuffer {
    /// Contains a handy label used for debugging. 
    label: Option<String>,

    /// Contains the position in which the command buffer will be 
    /// sent to the WGPU CommandQueue, usefull to order an unordered queue.
    /// 
    /// The order is limited to an u32 size, this should not be a problem
    /// due the CommandQueue is reased every frame.
    order: usize,

    /// The command to send to the GPU.
    command: CommandBuffer
}

/// A type alias of a thread shafe queue.
pub type CommandBufferQueue = ArrayQueue<OrderedCommandBuffer>;

/// Defines the FPS limits.
pub const FPS_LIMIT: f64 = 60.0;
