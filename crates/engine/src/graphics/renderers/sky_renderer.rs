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
            sky_render_pipeline::SkyRenderPipeline,
            bind_groups::sky_bind_group::SkyUniformLayout
        },
        renderers::{RenderOrder, CurrentSwapChainOutput},
        buffer::{BufferManipulator},
        texture::DepthTexture
    },
    scene::{ 
        components::{Sky, Transform}
    }
};

// /// Reprsets a system voxel renderer.


/// Runs the system, executed by the world.
pub fn sky_renderer_system(
    gpu: UniqueRead<Gpu>,
    sky_pipeline: UniqueRead<SkyRenderPipeline>,
    command_buffer: UniqueRead<CommandBufferQueue>,
    current_frame: UniqueRead<CurrentSwapChainOutput>,
    sky_layout: UniqueRead<SkyUniformLayout>,
    depth_texture: UniqueRead<DepthTexture>) {

    // Create the command enconder descriptor.
    let e_descriptor = CommandEncoderDescriptor {
        label: None
    };
    
    // Create a new enconder.
    let mut encoder = gpu.read().device.create_command_encoder(&e_descriptor);

    let frame = current_frame.read();
    if let Some(output) = &frame.0 {
        let sky_read = sky_layout.read();
        let group = &sky_read.group;

        let depth_texture_read = depth_texture.read();
        let depth_texture_attachment = &depth_texture_read.0.view;

        let rp_descriptor = RenderPassDescriptor {
            label: Some("Sky render pass"),
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &output.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
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

        let sky_pipeline_read = sky_pipeline.read();

        // Create the render pass.
        let mut rpass = encoder.begin_render_pass(&rp_descriptor);
        rpass.set_pipeline(&sky_pipeline_read.pipeline);
        // Bind the locals bind group to the group 0. 
        rpass.set_bind_group(0, group, &[]);
        // Set the vertex buffer.
        rpass.set_index_buffer(
            sky_pipeline_read.index_buffer.slice(..),
            wgpu::IndexFormat::Uint16
        );
        // Set the vertex buffer.
        rpass.set_vertex_buffer(0, sky_pipeline_read.vertex_buffer.slice(..));
        rpass.draw_indexed(0..sky_pipeline_read.index_len, 0, 0..1);
    }

    // Send the commander buffer
    match command_buffer.read().push(
        OrderedCommandBuffer {
            label: Some("Sky_Render_System".to_string()),
            order: RenderOrder::Sky.as_index(),
            command: encoder.finish()
        }   
    ) {
        Ok(_) => {
            //info("{SkyRenderer} Render pass finished correclty");
        },
        Err(_) => {
            warning("{SkyRenderer} Render pass error");
        }
    }
}
