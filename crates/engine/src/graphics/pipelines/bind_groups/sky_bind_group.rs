use bytemuck::{Pod, Zeroable};
use types::Color;

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
		pipelines::bind_groups::BindGroupGenerator
	}
};

/// Define where the sky with be placed in the shader.
const SKY_BINDING_POSITION: u32 = 0;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SkyUniform {
	start_color: Color<f32>,
	end_color: Color<f32>
}

impl SkyUniform {
	fn new() -> Self {
		SkyUniform {
			start_color: Color {
				r: 1.0,
				g: 1.0,
				b: 0.0
			},
			end_color: Color {
				r: 0.0,
				g: 1.0,
				b: 0.0
			}
		}
	}
}

impl RawBufferRepresentable for SkyUniform {
	/// Maps the content of SkyUniform to an array of bytes.
	fn get_raw<'a>(&'a self) -> Bytes<'a> {
		let vec_bytes: &[u8] = bytemuck::bytes_of(self);
		Bytes(vec_bytes)
	}
}

unsafe impl Pod for SkyUniform {}
unsafe impl Zeroable for SkyUniform {}

/// Contains the layout and bing group of the Sky.
pub struct SkyUniformLayout {
	pub group: BindGroup,
	pub layout: BindGroupLayout
}

/// Wrapes the SkyUniform buffer into a type to be used
/// in the ecs.
pub struct SkyUniformBuffer(pub Buffer);

/// Creates and return the locals bind group layout.
///
/// This defines the layout of the memory for the camera transformation and
/// some other information not yet defined.
fn create_sky_bind_group_layout_entry() -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        // The position in the shader.
        //
        // Normaly in wgsl that is extracted using the [[group(0), binding(0))]]
        // where 0 is the position.
        binding: SKY_BINDING_POSITION,
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
fn create_sky_bind_group_layout(gpu: &Gpu) -> BindGroupLayout {
    gpu.create_bind_group_layout(
        &BindGroupLayoutDescriptor {
            entries: &[
                create_sky_bind_group_layout_entry()
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
fn create_sky_bind_group(
	gpu: &Gpu,
	sky_buffer: &Buffer
) -> (BindGroup, BindGroupLayout) {
    // Create the layout for the bind group.
    let layout = create_sky_bind_group_layout(gpu);
    // Create the bind group.
    let bind_group = gpu.create_bind_group(&BindGroupDescriptor {
        layout: &layout,
        entries: &[
            BindGroupEntry {
                binding: SKY_BINDING_POSITION,
                resource: sky_buffer.as_entire_binding() 
            }
        ],
        label: None
    });

    (bind_group, layout)
}

/// Initialize the sky.
///
/// # Arguments
///
/// `gpu` - The gpu to be used to generate the buffers and layouts.
/// `world` - The world used to register the resources.
pub fn initialize_sky(gpu: &Gpu, world: &DefaultWorld) { 
    // Create a new sky in order to get memory layout and default data.
    let sky: SkyUniform = SkyUniform::new();

    // Allocate space in GPU for the sky data and get a reference to that.
    let sky_buffer: Buffer = gpu.create_uniform(sky);

    // Create the locals bind group.
    let (l_bind_group, l_bind_group_layout) = create_sky_bind_group(
        gpu,
        &sky_buffer
    );

    // Register the resource in the world.
    world.register_unique(SkyUniformBuffer(sky_buffer));
    // Reguster the locals layout.
    world.register_unique(SkyUniformLayout {
        group: l_bind_group,
        layout: l_bind_group_layout
    });
}
