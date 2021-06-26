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
            bind_groups::locals_bind_group::LocalsLayout
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
    locals_layout: UniqueRead<LocalsLayout>,
    depth_texture: UniqueRead<DepthTexture>,
    // Components
    sky: Read<Sky>) {

    // Create the command enconder descriptor.
    let e_descriptor = CommandEncoderDescriptor {
        label: None
    };
    
    // Create a new enconder.
    let mut encoder = gpu.read().device.create_command_encoder(&e_descriptor);

    let frame = current_frame.read();
    if let Some(output) = &frame.0 {
        let depth_texture_read = depth_texture.read();
        let depth_texture_attachment = &depth_texture_read.0.view;

        let rp_descriptor = RenderPassDescriptor {
            label: Some("Sky render pass"),
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

        let sky_pipeline_read = sky_pipeline.read();

        // Create the render pass.
        let mut rpass = encoder.begin_render_pass(&rp_descriptor);
        rpass.set_pipeline(&sky_pipeline_read.pipeline);
        rpass.draw(0..3, 0..1);
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
