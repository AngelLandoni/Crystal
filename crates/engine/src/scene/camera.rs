use cgmath::{Matrix4, Vector3, Point3, conv::array4x4};

use wgpu::{
    CommandEncoder,
    CommandEncoderDescriptor,
    Buffer,
    util::{DeviceExt, BufferInitDescriptor}
};

use ecs::{UniqueRead, UniqueWrite};

use crate::graphics::{ 
    pipelines::bind_groups::locals_bind_group::LocalsBuffer,
    gpu::Gpu
};

/// OpenGL matrix 
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[derive(Clone, Copy)]
pub struct Camera {
    /// Contains the position of the camera in the world.
    pub eye: Point3<f32>,
    
    /// The point where the camera is looking at.
    pub target: Point3<f32>,
    
    /// Contains the up direction of the world.
    pub up: Vector3<f32>,

    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Default for Camera {
    /// Creates and returns a new default camera.
    fn default() -> Self {
        Camera {
            eye: (0.0, 0.0, 0.0).into(),
            target: (1.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: 1.0,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }
}

impl Camera {
    /// Adds a translation to the camera.
    ///
    /// # Arguments
    ///
    /// `direction` - The direction of translation and the magnitude.
    pub fn add_translation(
        &mut self,
        direction: Vector3<f32>, amount: f32) {
        
        self.eye.x += direction.x * amount;
        self.eye.y += direction.y * amount;
        self.eye.z += direction.z * amount;
    }

    /// Add a translation to the target point (where the camera looks at).
    ///
    /// # Arguments
    ///
    /// `direction` - The direction of translation and the magnitude.
    pub fn add_target_translation(
        &mut self,
        direction: Vector3<f32>, amount: f32) {
        
        self.target.x += direction.x * amount;
        self.target.y += direction.y * amount;
        self.target.z += direction.z * amount;
    }

    /// Returns the view projection of the camera. 
    pub fn view_projection(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at(self.eye, self.target, self.up);
        let projection = cgmath::perspective(
            cgmath::Deg(self.fovy),
            self.aspect, self.znear,
            self.zfar);
        OPENGL_TO_WGPU_MATRIX * projection * view
    }
}

/// Mantains the locals buffer with respect to the camera.
///
/// If the camera changes that should be reflected into the locals. 
pub fn mantain_camera_buffer_system(
    gpu: UniqueRead<Gpu>,
    camera: UniqueRead<Camera>,
    locals_buffer: UniqueRead<LocalsBuffer>) {
    // Create a new enconder.
    let view_projection: [[f32; 4]; 4] = array4x4(camera.read().view_projection());
    let view_projection_bytes: &[u8] = bytemuck::cast_slice(&view_projection);

    gpu
        .read()
        .queue
        .write_buffer(&locals_buffer.read().0, 0, view_projection_bytes);
}

/// Updates the camera aspect.
pub fn update_camera_resize_system(
    new_aspect: f32,
    mut camera: UniqueWrite<Camera>) {
    // Access to the camera resource and updates the aspect.
    camera.write().aspect = new_aspect;
}