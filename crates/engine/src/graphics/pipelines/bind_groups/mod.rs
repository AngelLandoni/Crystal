pub mod locals_bind_group;

use wgpu::{
    BindGroup,
    BindGroupDescriptor,
    BindGroupLayout,
    BindGroupLayoutDescriptor,
};

/// Describes how a bing group generator should behave.
pub trait BindGroupGenerator {
    fn create_bind_group_layout(
        &self,
        descriptor: &BindGroupLayoutDescriptor) -> BindGroupLayout;

    fn create_bind_group(&self, descriptor: &BindGroupDescriptor) -> BindGroup;
}