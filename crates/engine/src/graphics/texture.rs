use wgpu::{TextureView, Sampler, TextureFormat};

/// Represents an engine texture, it contains the reference to the view, the 
/// texture id in gpu and the sampler.
pub struct Texture {
    /// Contains the reference to the texture in memory.
    pub raw_texture: wgpu::Texture,
    
    /// Contains the information needed to tell the render pass and bing 
    /// group how the texture should be used, AKA metadata.
    pub view: TextureView,

    /// Contains the information that the pipeline needs to pick information 
    /// from the `TextureView`, this defines wrapping mode and other stuff.
    pub sampler: Sampler
}

/// Provides the needed symbols used to generate textures. 
pub trait TextureGenerator {
    /// Should generate a new depth texture.
    fn create_depth_texture(&self) -> Texture;
}

/// Defines the depth format.
pub const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;

/// Represents an aftraction of a depth texture.
/// This is wrapping a simple texture due Shipyard dinstinguish the components
/// by the type.
pub struct DepthTexture(pub Texture);
