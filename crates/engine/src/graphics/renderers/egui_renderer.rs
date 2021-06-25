use chrono::Timelike;
use egui_wgpu_backend::ScreenDescriptor;

use wgpu::CommandEncoderDescriptor;

use epi::{
    backend::{FrameBuilder, AppOutput},
    App
};

use ecs::{UniqueRead, UniqueWrite};
use log::{info, warning};

use crate::{
    basics::window::Window,
    graphics::{
        CommandBufferQueue,
        OrderedCommandBuffer,
        gpu::Gpu,
        renderers::{RenderOrder, CurrentSwapChainOutput},
        egui::{EGui, EGuiRepaintSignal, DevGui}
    },
};

pub fn egui_renderer_system(
    gpu: UniqueRead<Gpu>,
    window: UniqueRead<Window>,
    egui: UniqueWrite<EGui>,
    dev_gui: UniqueRead<DevGui>,
    repaint_signal: UniqueRead<EGuiRepaintSignal>,
    current_frame: UniqueRead<CurrentSwapChainOutput>,
    command_buffer: UniqueRead<CommandBufferQueue>) {
    let gpu_r = gpu.read();
    let mut egui_w = egui.write();

    let dev_gui_r = dev_gui.read();
    let window_r = window.read();
    let repaint_signal_r = repaint_signal.read();
    let current_frame_r = current_frame.read();
    let command_buffer_r = command_buffer.read();

    if let Some(context) = &dev_gui_r.0 {
        // Tell egui we are staring a new frame.
        //egui.platform.begin_frame();

        // Create the backend output.
        let mut app_output: AppOutput = AppOutput::default();

        let mut frame = FrameBuilder {
            info: epi::IntegrationInfo {
                web_info: None,
                cpu_usage: None,
                seconds_since_midnight: Some(seconds_since_midnight()),
                native_pixels_per_point: Some(
                    window_r.native_window.scale_factor() as _
                ),
            },
            tex_allocator: Some(&mut egui_w.render_pass),
            output: &mut app_output,
            repaint_signal: repaint_signal_r.0.clone(),
        }.build();

        // End the UI frame. We could now handle the output and draw the UI 
        // with the backend.
        let (_output, paint_commands) = egui_w.platform.end_frame();
        let paint_jobs = egui_w.platform.context().tessellate(paint_commands);

        if let Some(output) = &current_frame_r.0 {
            let mut encoder = gpu_r.device.create_command_encoder(
                &CommandEncoderDescriptor {
                    label: Some("encoder"),
                }
            );

            // Upload all resources for the GPU.
            let screen_descriptor = ScreenDescriptor {
                physical_width: window_r.size.width,//gpu_r.swap_chain_descriptor.width,
                physical_height: window_r.size.height,//gpu_r.swap_chain_descriptor.height,
                scale_factor: window_r.native_window.scale_factor() as f32,
            };
        
            let tex = context.texture();
            egui_w.render_pass.update_texture(&gpu_r.device, &gpu_r.queue, &tex);
            egui_w.render_pass.update_user_textures(&gpu_r.device, &gpu_r.queue);
            egui_w.render_pass.update_buffers(
                &gpu_r.device,
                &gpu_r.queue,
                &paint_jobs,
                &screen_descriptor
            );
                
            // Record all render passes.
            egui_w.render_pass.execute(
                &mut encoder,
                &output.view,
                &paint_jobs,
                &screen_descriptor,
                None,
            );

            // Send the commander buffer
            match command_buffer_r.push(
                OrderedCommandBuffer {
                    label: Some("Voxel_Render_System".to_string()),
                    order: RenderOrder::DebugGui.as_index(),
                    command: encoder.finish()
                }   
            ) {
                Ok(_) => {
                    //info("[EGui] Render pass finished correclty");
                },
                Err(_) => {
                    warning("[EGui] Render pass error");
                }
            }
        }
    }
}

/// Time of day as seconds since midnight. Used for clock in demo app.
pub fn seconds_since_midnight() -> f64 {
    let time = chrono::Local::now().time();
    time.num_seconds_from_midnight() as f64 + 1e-9 * (time.nanosecond() as f64)
}
