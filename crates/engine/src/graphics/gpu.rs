use std::borrow::Cow;

use wgpu::{
    Device,
    Queue,
    SwapChain,
    SwapChainDescriptor,
    Instance,
    Surface,
    BackendBit,
    RequestAdapterOptions,
    PowerPreference,
    Limits,
    Features,
    TextureUsage,
    ShaderModule,
    ShaderSource,
    Buffer,
    BufferUsage,
    Adapter,
    TextureFormat,
    RenderPipeline,
    RenderPipelineDescriptor,
    BindGroup,
    BindGroupLayoutDescriptor,
    BindGroupLayout,
    BindGroupDescriptor,
    BufferDescriptor,
    BufferAddress,
    Extent3d,
    TextureDescriptor,
    TextureDimension,
    TextureView,
    TextureViewDescriptor,
    Sampler,
    SamplerDescriptor,
    COPY_BUFFER_ALIGNMENT,

    util::{DeviceExt, BufferInitDescriptor}
};

use ecs::{UniqueRead, UniqueWrite};
use log::{info, error};
use types::Size;

use crate::{
    basics::window::Window,
    helpers::errors::InitError,
    graphics::{
        shaders::{ShaderGenerator, ShaderProvider},
        buffer::{BufferCreator, RawBufferRepresentable, BufferManipulator},
        pipelines::bind_groups::BindGroupGenerator,
        texture::{Texture, TextureGenerator, DepthTexture, DEPTH_FORMAT},
    },
};

/// Contains all the configurations needed to create a Gpu.
#[derive(Copy, Clone)]
pub struct GpuOptions {
    /// Contains a flag to indicate if the GPU should use the main 
    /// graphics backend (Metal, Vulkan, DX12) or an alternative one
    /// (OpenGL, DX11).
    pub use_alternative_backend: bool,

    /// Contains a flag to indicate which pyshical GPU should be used. If it is
    /// false the high end graphics card will be used otherwise the low end
    /// will be used.
    ///
    /// If the platform / device contains only one GPU no matter wich option
    /// is setted that will be used.
    pub use_low_end_graphics_card: bool
}

impl Default for GpuOptions {
    /// Creates and returns a new instance of GpuOptions.
    ///
    /// By default it is configured with the highest end graphics card and
    /// the main graphics API.
    fn default() -> GpuOptions {
        GpuOptions {
            use_alternative_backend: false,
            use_low_end_graphics_card: false
        }
    }
}

/// Contains all the necesary information to interact with the GPU. 
pub struct Gpu {
    /// Contains the wgpu surface.
    /// This represents the match between wgpu and the window system.
    pub surface: Surface,

    /// Contains the wgpu adapter.
    pub adapter: Adapter,

    /// Contains an abstraction of the GPU, allowing to create rendering
    /// and compute resources.
    pub device: Device,

    /// Used to send commands to be executed in the GPU.
    pub queue: Queue,

    /// Represents the image or series of images that will be presented to
    /// a surface.
    pub swap_chain: SwapChain,

    /// Contains the swap chain description.
    pub swap_chain_descriptor: SwapChainDescriptor
}

impl Gpu {
    /// Creates and returns a new instance of a Gpu representation.
    ///
    /// # Arguments
    ///
    /// * `window` - the window used to extract the surface target.
    pub async fn default(window: &Window) -> Result<Self, InitError> {
        info("Generating GPU");
        let gpu = Gpu::new(window, GpuOptions::default()).await;
        info("GPU generated successfully");
        gpu
    }

        /// Creates and returns a new instance of a Gpu representation.
    ///
    /// # Arguments
    ///
    /// * `window` - The window used to extract the surface target.
    /// * `use_alternative_backend` - Indicates if it should use OpenGL or Dx11
    /// instead of the main one
    pub async fn new(window: &Window, options: GpuOptions) 
        -> Result<Self, InitError> {
        
        // Defines which backend should be used.
        let backend: BackendBit = match options.use_alternative_backend {
            true => BackendBit::SECONDARY,
            false => BackendBit::PRIMARY
        };

        // Creates a new WGPU instance. 
        let instance: Instance = Instance::new(backend);

        // Creates a new instance of a wgpu surface.
        let native_surface: Surface = unsafe {
            instance.create_surface(&window.native_window)
        };

        let graphics_card = match options.use_low_end_graphics_card {
            true => {
                info("Using low power graphics card");
                PowerPreference::LowPower
            },
            false => {
                info("Using high performnace graphics card");
                PowerPreference::HighPerformance
            }
        };

        // Generate the adapter options. This represents the the default GPU
        // to be used, for example in case of a MacBook pro this will take 
        // the dedicated GPU if exits, in case of an iPhone there is only
        // one option.
        let adapter_options: RequestAdapterOptions = RequestAdapterOptions {
            power_preference: graphics_card, 
            compatible_surface: Some(&native_surface)
        };

        // Get the adapter. The adapter allows a connection with the host 
        // system in order to get the device.
        let adapter = match instance.request_adapter(&adapter_options).await {
            Some(a) => a,
            None => {
                error("Error getting the adapter.");
                return Err(InitError::Gpu);
            }
        };

        let device_descriptor = wgpu::DeviceDescriptor {
            label: None,
            features: Features::empty(),
            limits: Limits::default(),
        };

        let (device, queue) 
            = match adapter.request_device(&device_descriptor, None).await {
            Ok((device, queue)) => (device, queue),
            Err(_) => {
                error("Error creating the device.");
                panic!();
            }
        };

        // Define the format of the image to write to.
        let swap_chain_descriptor = SwapChainDescriptor {
            usage: TextureUsage::RENDER_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width: window.size.width,
            height: window.size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&native_surface,
                                                  &swap_chain_descriptor);

        Ok(Self {
            surface: native_surface,
            adapter,
            device,
            queue,
            swap_chain,
            swap_chain_descriptor
        })
    }
}

impl Gpu {
    /// Returns the swap chain preferred format.
    pub fn swap_chain_format(&self) -> TextureFormat {
        self.adapter.get_swap_chain_preferred_format(&self.surface)
    }

    /// Creates and returns ¡a new render pipeline.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor used to create the pipeline.
    pub fn create_render_pipeline(&self,
        descriptor: &RenderPipelineDescriptor) -> RenderPipeline {
        self.device.create_render_pipeline(descriptor) 
    }
}

/// Provides to the Gpu aftraction the hability to handle bing groups.
impl BindGroupGenerator for Gpu {
    fn create_bind_group_layout(
        &self,
        descriptor: &BindGroupLayoutDescriptor) -> BindGroupLayout {
        self.device.create_bind_group_layout(descriptor)
    }

    fn create_bind_group(
        &self,
        descriptor: &BindGroupDescriptor) -> BindGroup {
       self.device.create_bind_group(descriptor) 
    }
}

/// Provides to the Gpu aftraction the hability to create uniforms / buffers.
///
/// TODO(Angel): Avoid copy the data just pass a reference.
impl BufferCreator for Gpu {
    /// Creates and returns a new vertex buffer after submit to Gpu.
    ///
    /// # Arguments
    ///
    /// * `data` - The raw buffer representable data used to create the buffer.
    fn create_vertex<T: RawBufferRepresentable>(&self, data: T) -> Buffer {
        self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: data.get_raw().content(),
            usage: BufferUsage::VERTEX
        })
    }

    /// Creates and returns a buffer of the specific size provided.
    fn create_vertex_with_size(&self, size: u64) -> Buffer {
        // Convert the size from the provided one into one that WGPU handles.
        let unpadded_size: BufferAddress = size as BufferAddress;
        // Make sure the size is 4 bytes aligned.
        let padding: BufferAddress = 
            COPY_BUFFER_ALIGNMENT -
            unpadded_size %
            COPY_BUFFER_ALIGNMENT; 
        
        // Final padding, the size now is memory aligned.
        let padded_size: BufferAddress = unpadded_size + padding;

        // Define the descriptor that contains all the information neeeded
        // to allocate the buffer.
        let descriptor: BufferDescriptor = BufferDescriptor {
            label: None,
            size: padded_size,
            usage: BufferUsage::VERTEX | BufferUsage::COPY_DST,
            mapped_at_creation: true
        };

        let buffer: Buffer = self.device.create_buffer(&descriptor);

        {
            let mut slice = buffer.slice(..).get_mapped_range_mut();
            for i in 0..padded_size {
                slice[i as usize] = 0;
            }
        }

        buffer.unmap();
        buffer
    }

    /// Creates and returns a new index buffer after submit to GPU.
    ///
    /// # Arguments
    ///
    /// * `data` - The raw buffer representable used to create the buffer.
    fn create_index<T: RawBufferRepresentable>(&self, data: T) -> Buffer {
        self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index buffer"),
            contents: data.get_raw().content(),
            usage: BufferUsage::INDEX
        })
    }

    /// Creates and returns a new uniform buffer.
    ///
    /// TODO(Angel): Add the usage, for not it is only copy dst so we cannot
    /// read from there just save.
    fn create_uniform<T: RawBufferRepresentable>(&self, data: T) -> Buffer {
        self.device.create_buffer_init(&BufferInitDescriptor{
            label: None,
            contents: data.get_raw().content(),
            usage: BufferUsage::UNIFORM | BufferUsage::COPY_DST
        })
    }
}

/// Provides to the Gpu aftraction the hability to handle shaders.
impl ShaderGenerator for Gpu {
    /// Creates and returns a new shader module.
    ///
    /// # Arguments
    ///
    /// * `source` - The Shader source to be compiled.
    fn create_shader(&self, source: &ShaderProvider) -> ShaderModule {
        let w_source: wgpu::ShaderSource = match source { 
            ShaderProvider::Wgsl(s) => ShaderSource::Wgsl(Cow::Borrowed(&s)),
            ShaderProvider::Glsl(_) => {
                panic!("Not implemeneted yet");
            },
        };

        self.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: w_source,
            flags: wgpu::ShaderFlags::all()
        }) 
    }  
}

/// Provides to the Gpu the aftraction to manipulate Gpu buffers.
impl BufferManipulator for Gpu {
    /// Copy the provided data into the GPU buffer.
    ///
    /// # Arguments
    ///
    /// `buffer` - The dest where to paste the data.
    /// `data` - The data to be copied.
    ///
    /// TODO(Angel): Create a more complex data structure to write an specific
    /// chunk of the buffer.
    fn copy_to_buffer(&self, buffer: &Buffer, data: &[u8]) {
        self.queue.write_buffer(buffer, 0, data);
    }
}

/// Provides to the Gpu the aftraction to manipualte textures on GPU.
impl TextureGenerator for Gpu {
    /// Creates and returns a new depth texture.
    fn create_depth_texture(&self) -> Texture {
        // Defines the size of the depth texture, in this case it should be
        // of the size of the entire screen or the swap chain.
        let size: Extent3d = Extent3d {
            width: self.swap_chain_descriptor.width,
            height: self.swap_chain_descriptor.height,
            depth: 1
        };

        // Create the wgpu texture descriptor. 
        let descriptor: TextureDescriptor = TextureDescriptor {
            label: None,
            // The size of the texture.
            size,
            // We only need 1 texture mip level (texture could have different
            // resolutions in order to be used at different distances).
            mip_level_count: 1,
            // No idea.
            sample_count: 1,
            // The texture is 2D.
            dimension: TextureDimension::D2,
            // We want a depth format.
            format: DEPTH_FORMAT,
            // We need render to the texture so RENDER_ATTACHMEN comes in 
            // place, sampled due the data could be extracted using a sampler.
            usage: TextureUsage::RENDER_ATTACHMENT | TextureUsage::SAMPLED
        };

        // Create the gpu texture.
        let raw_texture: wgpu::Texture = self.device.create_texture(&descriptor);
    
        // Create the view for the texture.
        let view: TextureView = raw_texture.create_view(
            &TextureViewDescriptor::default()
        );

        let sampler: Sampler = self.device.create_sampler(
            &SamplerDescriptor { // 4.
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                compare: Some(wgpu::CompareFunction::LessEqual), // 5.
                lod_min_clamp: -100.0,
                lod_max_clamp: 100.0,
                ..Default::default()
            }
        );

        Texture {
            raw_texture,
            view,
            sampler
        }
    }
}

/// Updates the Gpu with the new size provided.
pub fn update_gpu_with_new_size_system(
    size: Size<u32>,
    gpu: UniqueWrite<Gpu>,
    depth_texture: UniqueWrite<DepthTexture>) {
    
    let mut gpu_w = gpu.write();

    gpu_w.swap_chain_descriptor.width = size.width;
    gpu_w.swap_chain_descriptor.height = size.height;

    gpu_w.swap_chain = gpu_w.device.create_swap_chain(
        &gpu_w.surface,
        &gpu_w.swap_chain_descriptor
    );

    depth_texture.write().0 = gpu_w.create_depth_texture();
}