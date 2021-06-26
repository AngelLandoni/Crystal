pub mod voxel_renderer;
pub mod sky_renderer;
pub mod wireframe_voxel_renderer;
pub mod egui_renderer;

use wgpu::{CommandBuffer, SwapChainTexture};

use ecs::{UniqueRead, UniqueWrite};
use log::warning;

use crate::{
    graphics::{
        gpu::Gpu,
        CommandBufferQueue,
        OrderedCommandBuffer
    }
};

/// Represents the current active swap chain output.
// TODO(Angel): Bug in the ECS why it does not allow me to update the value without a 
// wrapper.
pub type CurrentSwapChainOutput = (Option<SwapChainTexture>, );

/// Provides the rendering order.
///
/// This is needed due the render systems are executed in parallel so the
/// rendering order is not guaranteed.
///
/// The position on the enum describes the order for submition.
/// TODO(Angel): Now the render order is inverted fix that.
#[derive(Copy, Clone)]
pub enum RenderOrder {
    /// Render the Sky
    Sky,
    /// Render EGui.
    DebugGui,
    /// Wireframe voxel rendering.
    WireframeVoxel,
    /// Voxel rendering order.
    Voxel
}

impl RenderOrder {
    /// Convert the enum from a option set representation to a number.
    fn as_index(&self) -> usize {
        *self as usize
    }
}

/// Updates the current frame texture with a new one. In order to work as a multi thread engine.
pub fn maintain_swap_chain_output_system(
    gpu: UniqueRead<Gpu>,
    sc_output: UniqueWrite<CurrentSwapChainOutput>) {
    // Checks if we can get a new swap chain output.
    if let Ok(frame) = gpu.read().swap_chain.get_current_frame() {
        // Update the active swap chain with the current one, this does not
        // affect the old ones due the swap chain output is under an RC layer,
        // if this is not used correct could potentially leak memory.
        sc_output.write().0 = Some(frame.output);
    } else {
        // If this line is executed means that something went wrong and we 
        // could not get the next frame buffer.
        warning("Seems like we can not get the next frame.");
    }
}

/// Clear all the data used, this must only be called with the frame ends.
pub fn clean_and_drop_system(
    output: UniqueWrite<CurrentSwapChainOutput>) {
    // Remove the current reference that the system has, with this we should 
    // reduce the counter of the swap chain output to 0 to delete it.
    output.write().0 = None;
}

/// Submits all the commands to the GPU.
/// TODO: Make it better, it is copying all over the place to order.
pub fn submit_commands_system(
    gpu: UniqueRead<Gpu>,
    commnad_buffer_queue: UniqueRead<CommandBufferQueue>) {
    let commmand_buffer_queue_read = commnad_buffer_queue.read();

    // Get the number of all the commands.
    let array_size = commmand_buffer_queue_read.len();
    // Create a new fixed size array to send all the commands.
    let mut all_commands = Vec::<OrderedCommandBuffer>::with_capacity(array_size);

    // Map all the commands in correct order.
    while let Some(c) = commmand_buffer_queue_read.pop() {
        all_commands.push(c);
    }

    // Short the commands.
    all_commands.sort_by_key(|c| c.order);

    // Extract the commands from the other vector.
    let mut order_commands = Vec::<CommandBuffer>::new();
    while let Some(c) = all_commands.pop() {
        order_commands.push(c.command);
    }

    // Submit all.
    gpu.read().queue.submit(order_commands);
}