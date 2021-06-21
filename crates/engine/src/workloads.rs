use ecs::{DefaultWorld, TaskWaitable, SystemHandler};

use crate::{
    graphics::{
        renderers::{
            voxel_renderer::voxel_renderer_system,
            wireframe_voxel_renderer::wireframe_voxel_renderer_system,
            egui_renderer::egui_renderer_system,
            maintain_swap_chain_output_system,
            submit_commands_system,
            clean_and_drop_system
        },
        egui::mantain_egui_context_system
    },
    scene::camera::mantain_camera_buffer_system
};

/// Represents all the available workloads in the engine.
/// Find a better name for this.
pub enum Workloads {
    Start,
    Synchronize,
    Render,
    Commit,
    End
}

/// Runs the provided workload in the provided world.
///
/// # Arguments
///
/// `workload` - The workload to be executed.
/// `world` - The world where the workload will be executed.
pub fn run_workload(workload: Workloads, world: &DefaultWorld) {
    // Match the workload with the actual work to do.
    match workload {
        Workloads::Start => run_start_workload(world),
        Workloads::Synchronize => run_synchronize_workload(world),
        Workloads::Render => run_render_workload(world),
        Workloads::Commit => run_commit_workload(world),
        Workloads::End => run_end_workload(world)
    }
}

/// Generates and executes the start workload.
///
/// All the tasks inside the start will be executed in parallel using the 
/// `Tasks` create.
///
/// # Arguments
///
/// `world` - The world which contains all the resources.
fn run_start_workload(world: &DefaultWorld) {
    (
        world.run(maintain_swap_chain_output_system),
        world.run(mantain_egui_context_system)
    ).wait();
}

/// Generates and executes the synchronize workload.
///
/// All the tasks inside the synchronize will be executed in parallel using the 
/// `Tasks` create.
///
/// # Arguments
///
/// `world` - The world which contains all the resources.
fn run_synchronize_workload(world: &DefaultWorld) {
    (
        world.run(mantain_camera_buffer_system),
    ).wait();
}

/// Generates and executes the render workload.
///
/// All the tasks inside the render will be executed in parallel using the 
/// `Tasks` create.
///
/// # Arguments
///
/// `world` - The world which contains all the resources.
fn run_render_workload(world: &DefaultWorld) {
    (
        world.run(voxel_renderer_system),
        world.run(wireframe_voxel_renderer_system),
        world.run(egui_renderer_system)
    ).wait();
}

/// Generates and executes the commit workload.
///
/// All the tasks inside the commit will be executed in parallel using the 
/// `Tasks` create.
///
/// # Arguments
///
/// `world` - The world which contains all the resources.
fn run_commit_workload(world: &DefaultWorld) {
    (
        world.run(submit_commands_system),
    ).wait();
}

/// Generates and executes the end workload.
///
/// All the tasks inside the end will be executed in parallel using the 
/// `Tasks` create.
///
/// # Arguments
///
/// `world` - The world which contains all the resources.
fn run_end_workload(world: &DefaultWorld) {
    (
        world.run(clean_and_drop_system),
    ).wait();
}