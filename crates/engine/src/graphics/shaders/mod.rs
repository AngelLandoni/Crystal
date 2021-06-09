use wgpu::ShaderModule;

/// Defins the possibles shader sources.
pub enum ShaderProvider {
    /// Default WGPU shader language.
    Wgsl(String),

    /// OpenGL shader language.
    Glsl(String),
}

pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute
}

/// Defines the possible actions for a shader generator.
pub trait ShaderGenerator {
    /// Should create a new shader using the provided source.
    fn create_shader(&self, source: &ShaderProvider) -> ShaderModule;
}