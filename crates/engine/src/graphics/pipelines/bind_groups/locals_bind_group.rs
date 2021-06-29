use cgmath::Matrix4;
use bytemuck::{Pod, Zeroable};

use wgpu::{
    BindGroupLayoutEntry,
    ShaderStage,
    BindingType,
    BufferBindingType,
    BufferSize,
    BindGroup,
    BindGroupDescriptor,
    BindGroupLayout,
    BindGroupLayoutDescriptor,
    BindGroupEntry,
    Buffer
};

use ecs::{DefaultWorld, ComponentHandler};
use types::Bytes;

use crate::{
    graphics::{
        gpu::Gpu,
        buffer::{BufferCreator, RawBufferRepresentable},
        pipelines::bind_groups::BindGroupGenerator,
    },
    scene::camera::Camera
};

/// Define where the locals with be placed in the shader.
pub const LOCAL_BINDING_POSITION: u32 = 0;

/// Represetns all the locals uniforms to be sent to the GPU.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Locals {
    /// The point of view of the camera.
    view: Matrix4<f32>,
    
    /// The projection of the camera.
    projection: Matrix4<f32>
}

impl Locals {
    /// Creates and returns a new `Locals` using the default camera projection.
    pub fn new() -> Self {
        let default_camera = Camera::default();

        Locals {
            view: default_camera.view(),
            projection: default_camera.projection()
        }
    }
}

/// Implements `RawBufferRepresentable` for the Locals.
impl RawBufferRepresentable for Locals {
    /// Maps the content of Locals to an array of Bytes.
    fn get_raw<'a>(&'a self) -> Bytes<'a> {
        let vec_bytes: &[u8] = bytemuck::bytes_of(self);
        Bytes(vec_bytes)
    }    
}

unsafe impl Pod for Locals {}
unsafe impl Zeroable for Locals {}

/// Contains the initialization of the locals in Gpu.
///
/// TODO(Angel): Find a better name for this.
pub struct LocalsLayout {
    pub group: BindGroup,
    pub layout: BindGroupLayout
}

/// Wrapps the locals buffer pointer.
/// This is be exposed in the world as a resource.
pub struct LocalsBuffer(pub Buffer);

/// Creates and return the locals bind group layout.
///
/// This defines the layout of the memory for the camera transformation and
/// some other information not yet defined.
pub fn create_locals_bind_group_layout_entry() -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        // The position in the shader.
        //
        // Normaly in wgsl that is extracted using the [[group(0), binding(0))]]
        // where 0 is the position.
        binding: LOCAL_BINDING_POSITION,
        // Where the information is visible, in this case it is only visible
        // for the vertex stage, we do not need the camera transformation 
        // in the frag for now.
        visibility: ShaderStage::VERTEX, 
        // Defines the type of allocation that is needed, in this case is just
        // a camera so a normal buffer is ok, also we can send images if needed
        // if we need to do some specific task and extract information form a
        // big two dimentional matrix.
        ty: BindingType::Buffer {
            // Not sure the difference but we need an Uniform, TODO(Angel)
            // research more about this.
            ty: BufferBindingType::Uniform,
            // Not sure why we need this, TODO(Angel) research a little bit more.
            has_dynamic_offset: false,
            // Not sure why TODO(Angel) research.
            min_binding_size: None,
        },
        // Not sure why TODO(Angel) research.
        count: None
    }
}

/// Creates and returns the local bind group layout
///
/// # Arguments
///
/// `gpu` - The gpu used to generate the bing group.
fn create_locals_bind_group_layout(gpu: &Gpu) -> BindGroupLayout {
    gpu.create_bind_group_layout(
        &BindGroupLayoutDescriptor {
            entries: &[
                create_locals_bind_group_layout_entry()
            ],
            label: None
        }
    )
}

/// Creates and returns a new locals bind group.
///
/// # Arguments
///
/// `gpu` - The gpu used to create the bind group.
/// `buffer` - The buffer address which conatins the information.
fn create_locals_bind_group(gpu: &Gpu,
                            buffer: &Buffer) -> (BindGroup, BindGroupLayout) {
    // Create the layout for the bind group.
    let layout = create_locals_bind_group_layout(gpu);
    // Create the bind group.
    let bind_group = gpu.create_bind_group(&BindGroupDescriptor {
        layout: &layout,
        entries: &[
            BindGroupEntry {
                binding: LOCAL_BINDING_POSITION,
                resource: buffer.as_entire_binding() 
            }
        ],
        label: None
    });

    (bind_group, layout)
}

/// Initialize the locals.
///
/// # Arguments
///
/// `gpu` - The gpu to be used to generate the buffers and layouts.
/// `world` - The world used to register the resources.
pub fn initialize_locals(gpu: &Gpu, world: &DefaultWorld) { 
    // Create a new locals in order to get memory layout and default data.
    let locals: Locals = Locals::new();

    // Allocate space in GPU for locals data and get the reference.
    let locals_buffer: Buffer = gpu.create_uniform(locals);

    // Create the locals bind group.
    let (l_bind_group, l_bind_group_layout) = create_locals_bind_group(
        gpu,
        &locals_buffer
    );

    // Register the resource in the world.
    world.register_unique(LocalsBuffer(locals_buffer));
    // Reguster the locals layout.
    world.register_unique(LocalsLayout {
        group: l_bind_group,
        layout: l_bind_group_layout
    });
}