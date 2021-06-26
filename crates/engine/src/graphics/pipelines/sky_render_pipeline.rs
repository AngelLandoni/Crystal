use cgmath::{Vector3, Matrix4};

use wgpu::{
    RenderPipeline,
    RenderPipelineDescriptor,
    Buffer,
    PipelineLayoutDescriptor,
    VertexState,
    FragmentState,
    ShaderModule,
    PrimitiveState,
    VertexBufferLayout,
    BufferAddress,
    InputStepMode,
    VertexAttribute,
    VertexFormat,
    DepthStencilState,
    CompareFunction,
    StencilState,
    DepthBiasState
};

use ecs::{DefaultWorld, UniqueRead, ComponentHandler};
use log::info;

use crate::{
    graphics::{ 
        gpu::Gpu,
        vertex::Vertex,
        buffer::BufferCreator,
        shaders::{ShaderProvider, ShaderGenerator},
        pipelines::{
        	bind_groups::{
        		sky_bind_group::SkyUniformLayout,
        		locals_bind_group::LocalsLayout
        	},
        },
        texture::DEPTH_FORMAT
    },
    scene::components::{Voxel, Transform},
};

pub struct SkyRenderPipeline {
	/// Contains the render pipeline for the sky.
	pub pipeline: RenderPipeline
}

impl SkyRenderPipeline {
    /// Creates and returns a new voxel renderder pipeline.
    ///
    /// # Arguments
    ///
    /// * `gpu` - The gpu used to create the pipeline.
    pub fn new(gpu: &Gpu, world: &DefaultWorld) -> Self {
        info("Creating SkyRenderPipeline");

        // Generates the shader.
        let shader_module = create_shader(&gpu);

        info("{VoxelRenderPipeline} Crearing pipeline layout");

        // Creates the pipeline layout.
        let pipeline_layout = gpu.device.create_pipeline_layout(
            &PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[]
            }
        );

        info("{SkyRenderPipeline} Finish creating pipeline layout");

        // Get the swap chain format.
        let swapchain_format = gpu.swap_chain_format();

        info("{SkyRenderPipeline} Crearing render pipeline");

        let render_pipeline: RenderPipeline = gpu.create_render_pipeline(
            &RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &shader_module,
                    entry_point: "vs_main",
                    buffers: &[]
                },
                fragment: Some(FragmentState {
                    module: &shader_module,
                    entry_point: "fs_main",
                    targets: &[swapchain_format.into()],
                }),
                primitive: PrimitiveState {
                    cull_mode: wgpu::CullMode::Back,
                    ..Default::default()
                },
                depth_stencil: Some(
                    DepthStencilState {
                        format: DEPTH_FORMAT,
                        depth_write_enabled: true,
                        depth_compare: CompareFunction::Less,
                        stencil: StencilState::default(),
                        bias: DepthBiasState::default(),
                        clamp_depth: false
                    } 
                ),
                multisample: wgpu::MultisampleState::default(),
            }
        );

        info("{SkyRenderPipeline} Voxel pipeline created");

        Self {
            pipeline: render_pipeline
        }
    }
}

/// Creates and returns the shader module for the Voxel render pipeline.
///
/// # Arguments
///
/// * `gpu` - The gpu used to create the shader.
fn create_shader(gpu: &Gpu) -> ShaderModule {
    // Generate a string shader from the static string and create
    // the shader provieder using wgsl.
    let provider: ShaderProvider = ShaderProvider::Wgsl(
        String::from(include_str!("../shaders/sky_shader.wgsl"))
    );
    
    // Call the gpu in order to create the shader.
    gpu.create_shader(&provider)
}
