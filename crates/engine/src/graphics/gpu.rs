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

    util::{ DeviceExt, BufferInitDescriptor }
};

use log::{info, error};

use crate::{
    basics::window::Window,
    helpers::errors::InitError
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


