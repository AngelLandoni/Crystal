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
            bind_groups::locals_bind_group::LocalsLayout,
            voxel_render_pipeline::{
                create_voxel_vertices,
                create_voxel_indices,
                allocate_gpu_buffers
            }
        },
        texture::DEPTH_FORMAT
    },
    scene::components::{WireframeVoxel, Transform},
};

/// TODO: Rename this to pipeline the module already defines context and Rust is
/// super nice and we can use them as namespaces.
pub struct WireframeVoxelRenderPipeline {
    /// Contains the Wgpu pipeline.
    pub pipeline: RenderPipeline,

    /// Contains a reference to all the vertices in the Gpu.
    pub vertex_buffer: Buffer,

    /// Contains a reference to the indices in the Gpu
    pub index_buffer: Buffer,

    /// Contains the number of indices in the index buffer.
    pub index_len: u32,

    /// Contains the buffer which contains all the transformations.
    pub transformations_buffer: Buffer,
    
    /// Contains the buffer which conatins all the colors.
    pub voxels_buffer: Buffer
}

impl WireframeVoxelRenderPipeline {
    /// Creates and returns a new voxel renderder pipeline.
    ///
    /// # Arguments
    ///
    /// * `gpu` - The gpu used to create the pipeline.
    pub fn new(gpu: &Gpu, world: &DefaultWorld) -> Self {
        info("Creating VoxelRenderPipeline");

        // Generate the needed vertices and indices. 
        let vertices = create_voxel_vertices();
        let indices = create_voxel_indices();
        let indices_len = indices.len();

        // Generates the shader.
        let shader_module = create_shader(&gpu);

        // Create the basic needed buffers on GPU.
        let vertices_buffer: Buffer = gpu.create_vertex(vertices);
        let indices_buffer: Buffer = gpu.create_index(indices);

        let locals_layout = world.get::<UniqueRead<LocalsLayout>>();

        info("{VoxelRenderPipeline} Crearing pipeline layout");

        // Creates the pipeline layout.
        let pipeline_layout = gpu.device.create_pipeline_layout(
            &PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    // Creates the layout for the locals.
                    &locals_layout.read().layout
                ],
                push_constant_ranges: &[]
            }
        );

        info("{WireframeVoxelRenderPipeline} Finish creating pipeline layout");

        // Get the swap chain format.
        let swapchain_format = gpu.swap_chain_format();

        info("{WireframeVoxelRenderPipeline} Crearing render pipeline");

        let render_pipeline: RenderPipeline = gpu.create_render_pipeline(
            &RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &shader_module,
                    entry_point: "vs_main",
                    buffers: &[
                        create_vertex_layout(),
                        create_style_layout(),
                        create_transformation_layout()
                    ]
                },
                fragment: Some(FragmentState {
                    module: &shader_module,
                    entry_point: "fs_main",
                    targets: &[swapchain_format.into()],
                }),
                primitive: PrimitiveState {
                    topology: wgpu::PrimitiveTopology::LineStrip,
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

        info("{VoxelRenderPipeline} Voxel pipeline created");

        let (transformations_buffer, voxels_buffer) = allocate_gpu_buffers(&gpu);

        Self {
            pipeline: render_pipeline,
            vertex_buffer: vertices_buffer,
            index_buffer: indices_buffer,
            index_len: indices_len as u32,
            transformations_buffer,
            voxels_buffer
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
        String::from(include_str!("../shaders/voxel_shader.wgsl"))
    );
    
    // Call the gpu in order to create the shader.
    gpu.create_shader(&provider)
}

/// Creates and returns the vertex layout, this is used to know how the
/// GPU should align the memory sent by the CPU.
/// 
/// In the shader the structure for the vertex can be complex not just the +
/// position of it but the color and usefull information, that is why 
/// we need this layout.
///
/// We can send the data to the GPU using the set_vertex_buffer function.
fn create_vertex_layout<'a>() -> VertexBufferLayout<'a> {
    VertexBufferLayout {
        // How long is the data that we want to send.
        array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
        // We want the data for each vertex.
        step_mode: InputStepMode::Vertex,
        // Defines the specific layout of `Vertex` (Each of the fields).
        attributes: &[
            // Describes the position of the `Vertex`.
            VertexAttribute {
                // The size of the data in GPU.
                format: VertexFormat::Float4, 
                // Position on the memory sent by the CPU.
                offset: 0,
                // Where it should map the data in the shader.
                shader_location: 0
            },
            // TODO(Angel): Add the rest of the parameters like UV etc.
        ]
    }
}

/// Creates and returns the style layout, this is used to know the how the 
/// GPU should align the memory sent by the CPU.
///
/// This is useful to send the per voxel style.
fn create_style_layout<'a>() -> VertexBufferLayout<'a> {
    VertexBufferLayout {
        // The size of the Voxel content.
        array_stride: std::mem::size_of::<WireframeVoxel>() as BufferAddress,
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
                shader_location: 1
            }
        ]
    }
}

/// Creates and returns the transformation layout, this is used to translate the
/// vertex on the GPU side.
fn create_transformation_layout<'a>() -> VertexBufferLayout<'a> {
    // Take the size of the internal type.
    const UNIT_SIZE: usize = std::mem::size_of::<f32>();
    const ROW_SIZE: u64 = UNIT_SIZE as u64 * 4;

    VertexBufferLayout {
        // How long is the data we want to send.
        array_stride: std::mem::size_of::<Matrix4<f32>>() as BufferAddress,
        // Iterate over the data per instance.
        step_mode: InputStepMode::Instance,
        // Defines the specific layout for the data.
        attributes: &[
            // Defines the matrix 4x4. As WGPU(0.7) does not have Float4x4 we
            // have to craft it using 4 Float4.
            VertexAttribute {
                format: VertexFormat::Float4,
                // The position of the data.
                offset: ROW_SIZE * 0,
                // The location on the shader
                shader_location: 2
            },
            VertexAttribute {
                format: VertexFormat::Float4,
                // The position of the data.
                offset: ROW_SIZE * 1,
                // The location on the shader
                shader_location: 3
            },
            VertexAttribute {
                format: VertexFormat::Float4,
                // The position of the data.
                offset: ROW_SIZE * 2,
                // The location on the shader
                shader_location: 4
            },
            VertexAttribute {
                format: VertexFormat::Float4,
                // The position of the data.
                offset: ROW_SIZE * 3,
                // The location on the shader
                shader_location: 5
            }
        ]
    }
}
