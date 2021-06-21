use wgpu::{
    CommandEncoderDescriptor,
    RenderPassDescriptor,
    RenderPassDepthStencilAttachmentDescriptor,
    Operations,
    LoadOp
};

use ecs::{
    UniqueRead,
    Read,
    Searchable
};

use log::{info, warning};

use crate::{
    graphics::{
        CommandBufferQueue,
        OrderedCommandBuffer,
        gpu::Gpu,
        pipelines::{ 
            wireframe_voxel_render_pipeline::WireframeVoxelRenderPipeline,
            bind_groups::locals_bind_group::LocalsLayout
        },
        renderers::{RenderOrder, CurrentSwapChainOutput},
        buffer::{BufferManipulator},
        texture::DepthTexture
    },
    scene::{ 
        components::{WireframeVoxel, Transform}
    }
};

// /// Reprsets a system voxel renderer.


/// Runs the system, executed by the world.
pub fn wireframe_voxel_renderer_system(
    gpu: UniqueRead<Gpu>,
    voxel_pipeline: UniqueRead<WireframeVoxelRenderPipeline>,
    command_buffer: UniqueRead<CommandBufferQueue>,
    current_frame: UniqueRead<CurrentSwapChainOutput>,
    locals_layout: UniqueRead<LocalsLayout>,
    depth_texture: UniqueRead<DepthTexture>,
    // Components
    voxels: Read<WireframeVoxel>,
    transformations: Read<Transform>) {

    // Create the command enconder descriptor.
    let e_descriptor = CommandEncoderDescriptor {
        label: None
    };
    
    // Create a new enconder.
    let mut encoder = gpu.read().device.create_command_encoder(&e_descriptor);

    // Create a buffer for all the transformations, at this point we should have
    // a cache system so if there are not changes on the items we could avoid 
    // this part.
    let mut raw_transforms: Vec<u8> = Vec::<u8>::new();

    // Creates a buffer for all the colors, we should implement a cache system
    // for this.
    let mut raw_colors: Vec<u8> = Vec::<u8>::new();

    // Generate the transformation buffer.
    // TODO(Angel): Limit this loop due the pipeline only supports 200000 of 
    // them.
    // TODO(Angel): Improve this using multithreading.
    (voxels.iter(), transformations.iter())
        .query()
        .for_each(|(voxel, transfrom)| {
            // Get the raw transformation.
            let raw_transform = transfrom.read().as_matrix_array();
            // Transform the raw information to a binary array.
            let data = bytemuck::cast_slice(&raw_transform);
            // Append that to the vector.
            raw_transforms.append(&mut Vec::from(data));

            // Get the raw color.
            let raw_color: [f32; 3] = voxel.read().color_as_array();
            // Get a representation of the color in bytes.
            let data: &[u8] = bytemuck::bytes_of(&raw_color);
            // Conver the array into a vector and append that vector to the 
            // colors vector.
            raw_colors.append(&mut Vec::from(data));
        });

    let frame = current_frame.read();
    if let Some(output) = &frame.0 {
        let depth_texture_read = depth_texture.read();
        let depth_texture_attachment = &depth_texture_read.0.view;

        let rp_descriptor = RenderPassDescriptor {
            label: Some("WireframeVoxel render pass"),
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &output.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: Some(
                RenderPassDepthStencilAttachmentDescriptor {
                    attachment: depth_texture_attachment,
                    depth_ops: Some(
                        Operations {
                            load: LoadOp::Clear(1.0),
                            store: true
                        }
                    ),
                    stencil_ops: None
                }
            ),
        };

        // If it has transformations it means there are some entities to be 
        // rendererd.
        if !raw_transforms.is_empty() {
            let layout_read = locals_layout.read();
            let group = &layout_read.group;

            // Get the number of instances.
            let num_inst: u32 = transformations.len() as u32;
            
            // Copy data to the buffer
            let gpu_read = gpu.read();
            let voxel_pipeline_read = voxel_pipeline.read();

            gpu_read.copy_to_buffer(
                &voxel_pipeline_read.transformations_buffer,
                &raw_transforms);
            gpu_read.copy_to_buffer( 
                &voxel_pipeline_read.voxels_buffer,
                &raw_colors
            );

            // Create the render pass.
            let mut rpass = encoder.begin_render_pass(&rp_descriptor);
            rpass.set_pipeline(&voxel_pipeline_read.pipeline);
            // Bind the locals bind group to the group 0. 
            rpass.set_bind_group(0, group, &[]);
            // Set the vertex buffer.
            rpass.set_index_buffer(
                voxel_pipeline_read.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16
            );
            // Set the vertex buffer.
            rpass.set_vertex_buffer(0, voxel_pipeline_read.vertex_buffer.slice(..));
            rpass.set_vertex_buffer(1, voxel_pipeline_read.voxels_buffer.slice(..));
            rpass.set_vertex_buffer(2, voxel_pipeline_read.transformations_buffer.slice(..));
            rpass.draw_indexed(0..voxel_pipeline_read.index_len, 0, 0..num_inst);
        }
    }

    // Send the commander buffer
    match command_buffer.read().push(
        OrderedCommandBuffer {
            label: Some("WireframeVoxel_Render_System".to_string()),
            order: RenderOrder::WireframeVoxel.as_index(),
            command: encoder.finish()
        }   
    ) {
        Ok(_) => {
            //info("{VoxelRenderer} Render pass finished correclty");
        },
        Err(_) => {
            warning("{WireframeVoxelRenderer} Render pass error");
        }
    }
}
