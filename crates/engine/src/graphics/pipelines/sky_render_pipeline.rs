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
    scene::components::{Sky, Transform},
};

pub struct SkyRenderPipeline {
	/// Contains the render pipeline for the sky.
	pub pipeline: RenderPipeline,

    /// Contains a reference to all the vertices in the Gpu.
    pub vertex_buffer: Buffer,

    /// Contains a reference to the indices in the Gpu
    pub index_buffer: Buffer,

    /// Contains the number of indices in the index buffer.
    pub index_len: u32,
}

impl SkyRenderPipeline {
    /// Creates and returns a new voxel renderder pipeline.
    ///
    /// # Arguments
    ///
    /// * `gpu` - The gpu used to create the pipeline.
    pub fn new(gpu: &Gpu, world: &DefaultWorld) -> Self {
        info("Creating SkyRenderPipeline");

         // Generate the needed vertices and indices. 
        let vertices = create_voxel_vertices();
        let indices = create_voxel_indices();
        let indices_len = indices.len();

        // Create the basic needed buffers on GPU.
        let vertices_buffer: Buffer = gpu.create_vertex(vertices);
        let indices_buffer: Buffer = gpu.create_index(indices);

        // Generates the shader.
        let shader_module = create_shader(&gpu);

        info("{VoxelRenderPipeline} Crearing pipeline layout");

        let sky_layout = world.get::<UniqueRead<SkyUniformLayout>>();

        // Creates the pipeline layout.
        let pipeline_layout = gpu.device.create_pipeline_layout(
            &PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    // Creates the layout for the locals.
                    &sky_layout.read().layout
                ],
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
                    buffers: &[
                        // This thing is used not for the uniforms but the vertex thing
                        //create_style_layout()
                    ]
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
            pipeline: render_pipeline,
            vertex_buffer: vertices_buffer,
            index_buffer: indices_buffer,
            index_len: indices_len as u32,
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

/// Creates and returns the style layout, this is used to know the how the 
/// GPU should align the memory sent by the CPU.
///
/// This is useful to send the per voxel style.
fn create_style_layout<'a>() -> VertexBufferLayout<'a> {
    VertexBufferLayout {
        // The size of the Voxel content.
        array_stride: std::mem::size_of::<Sky>() as BufferAddress,
        // We want data per instance.
        step_mode: InputStepMode::Instance,
        // Defines the specific layout for each style instance.
        attributes: &[
            // Describes the position of the `color`.
            VertexAttribute {
                // The size of the data, in this case we take care only 
                // of RGB so we need 3 floats.
                format: VertexFormat::Float3,
                // Starting from the initial place.
                offset: 0,
                // Set the shader location.
                shader_location: 0
            },
            VertexAttribute {
                // The size of the data, in this case we take care only 
                // of RGB so we need 3 floats.
                format: VertexFormat::Float3,
                // Starting from the initial place.
                offset: 0,
                // Set the shader location.
                shader_location: 1
            }
        ]
    }
}

/// Creates and returns the needed vertices.
pub(crate) fn create_voxel_vertices() -> Vec<Vertex> {
    [
        // Top face.
        Vertex::new(Vector3 { x: -1.0, y: -1.0, z: 1.0 }, [0.0, 0.0]),
        Vertex::new(Vector3 { x: 1.0, y: -1.0, z: 1.0 }, [1.0, 0.0]),
        Vertex::new(Vector3 { x: 1.0, y: 1.0, z: 1.0 }, [1.0, 1.0]),
        Vertex::new(Vector3 { x: -1.0, y: 1.0, z: 1.0 }, [0.0, 1.0]),
        // Bottom face.
        Vertex::new(Vector3 { x: -1.0, y: 1.0, z: -1.0 }, [1.0, 0.0]),
        Vertex::new(Vector3 { x: 1.0, y: 1.0, z: -1.0 }, [0.0, 0.0]),
        Vertex::new(Vector3 { x: 1.0, y: -1.0, z: -1.0 }, [0.0, 1.0]),
        Vertex::new(Vector3 { x: -1.0, y: -1.0, z: -1.0 }, [1.0, 1.0]),
        // Right face.
        Vertex::new(Vector3 { x: 1.0, y: -1.0, z: -1.0 }, [0.0, 0.0]),
        Vertex::new(Vector3 { x: 1.0, y: 1.0, z: -1.0 }, [1.0, 0.0]),
        Vertex::new(Vector3 { x: 1.0, y: 1.0, z: 1.0 }, [1.0, 1.0]),
        Vertex::new(Vector3 { x: 1.0, y: -1.0, z: 1.0 }, [0.0, 1.0]),
        // Left face.
        Vertex::new(Vector3 { x: -1.0, y: -1.0, z: 1.0 }, [1.0, 0.0]),
        Vertex::new(Vector3 { x: -1.0, y: 1.0, z: 1.0 }, [0.0, 0.0]),
        Vertex::new(Vector3 { x: -1.0, y: 1.0, z: -1.0 }, [0.0, 1.0]),
        Vertex::new(Vector3 { x: -1.0, y: -1.0, z: -1.0 }, [1.0, 1.0]),
        // Front face.
        Vertex::new(Vector3 { x: 1.0, y: 1.0, z: -1.0 }, [1.0, 0.0]),
        Vertex::new(Vector3 { x: -1.0, y: 1.0, z: -1.0 }, [0.0, 0.0]),
        Vertex::new(Vector3 { x: -1.0, y: 1.0, z: 1.0 }, [0.0, 1.0]),
        Vertex::new(Vector3 { x: 1.0, y: 1.0, z: 1.0 }, [1.0, 1.0]),
        // Back face.
        Vertex::new(Vector3 { x: 1.0, y: -1.0, z: 1.0 }, [0.0, 0.0]),
        Vertex::new(Vector3 { x: -1.0, y: -1.0, z: 1.0 }, [1.0, 0.0]),
        Vertex::new(Vector3 { x: -1.0, y: -1.0, z: -1.0 }, [1.0, 1.0]),
        Vertex::new(Vector3 { x: 1.0, y: -1.0, z: -1.0 }, [0.0, 1.0])
    ].to_vec()
}

/// Creates and returns the needed indices.
pub(crate) fn create_voxel_indices() -> Vec<u16> {
    let index_data: &[u16] = &[
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    index_data.to_vec()
}
