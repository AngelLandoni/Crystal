pub mod bind_groups;
pub mod voxel_render_pipeline;

use ecs::{DefaultWorld, ComponentHandler};

use crate::{
	graphics::{
		gpu::Gpu,
		pipelines::voxel_render_pipeline::VoxelRenderPipeline
	}
};

/// Inits all the default pipelines available in the engine.
///
/// # Arguments
///
/// `world` - The world where the pipelines will be stored.
pub fn initialize_pipelines(gpu: &Gpu, world: &DefaultWorld) {
	// Create and set the voxel pipeline.
	world.register_unique(VoxelRenderPipeline::new(gpu, world));
}