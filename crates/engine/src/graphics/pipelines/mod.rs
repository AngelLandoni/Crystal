pub mod bind_groups;
pub mod voxel_render_pipeline;
pub mod wireframe_voxel_render_pipeline;
pub mod sky_render_pipeline;

use ecs::{DefaultWorld, ComponentHandler};

use crate::{
	graphics::{
		gpu::Gpu,
		pipelines::{
			voxel_render_pipeline::VoxelRenderPipeline,
			wireframe_voxel_render_pipeline::WireframeVoxelRenderPipeline,
			sky_render_pipeline::SkyRenderPipeline
		}
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
	world.register_unique(SkyRenderPipeline::new(gpu, world));
	world.register_unique(WireframeVoxelRenderPipeline::new(gpu, world));
}