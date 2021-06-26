use rand::Rng;

use cgmath::{
    Vector3,
    Matrix4,
    Quaternion,
    Deg,
    Rad,
    conv::array4x4,
    conv::array3
};

use types::Color;

#[derive(Clone, Copy)]
pub struct Voxel {
    pub color: Vector3<f32>
}

impl Default for Voxel {
    /// Creates and returns a new instance of `Voxel`.
    fn default() -> Self {
        Self {
            color: Vector3 { x: 1.0, y: 1.0, z: 1.0 }
        }
    }
}

impl Voxel {
    /// Returns the size of `Voxel` in number of bytes.
    pub fn size() -> u32 {
        std::mem::size_of::<Self>() as u32
    }
}

impl Voxel {
    /// Creates and returns a new instnace of `Voxel`
    ///
    /// # Arguments
    ///
    /// `color` - The color for the Voxel.
    pub fn color(r: f32, g: f32, b: f32) -> Self {
        Self {
            color: Vector3 { x: r, y: g, z: b }
        }
    }

    /// Creates and returns a new instance of 'Voxel' using a random
    /// color.
    pub fn rand_color() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            color: Vector3 {
                x: rng.gen_range(0.0..1.0),
                y: rng.gen_range(0.0..1.0),
                z: rng.gen_range(0.0..1.0)
            }
        }
    }
}

impl Voxel {
    /// Creates and returns the a new 3 elements array which contains the color.
    pub fn color_as_array(&self) -> [f32; 3] {
       array3(self.color)
    }
}

#[derive(Clone, Copy)]
pub struct WireframeVoxel {
    pub color: Vector3<f32>
}

impl Default for WireframeVoxel {
    fn default() -> Self {
        Self {
            color: Vector3 { x: 1.0, y: 1.0, z: 1.0 }
        }
    }
}

impl WireframeVoxel {
    /// Returns the size of `Voxel` in number of bytes.
    pub fn size() -> u32 {
        std::mem::size_of::<Self>() as u32
    }
}

impl WireframeVoxel {
    /// Creates and returns the a new 3 elements array which contains the color.
    pub fn color_as_array(&self) -> [f32; 3] {
       array3(self.color)
    }
}

/// Represents a trasnformation component.
///
/// This is used to transform one specif entity in the `World`.
pub struct Transform {
    pub position: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotation: Quaternion<f32>
}

impl Transform {
    /// Returns the size of `Transform` in number of bytes.
    pub fn size() -> u32 {
        std::mem::size_of::<Self>() as u32
    }
}

impl Transform {
    /// Creates and returns a new 4x4 matrix which contains the position, 
    /// rotation and scale.
    pub fn as_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position) *
        Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z) *
        Matrix4::from_axis_angle(self.rotation.v, Rad::from(Deg(self.rotation.s)))
    }

    /// Creates and returns a new 4x4 matrix and returns it in an array form.
    pub fn as_matrix_array(&self) -> [[f32; 4]; 4] {
        array4x4(self.as_matrix())
    }
}

#[derive(Clone, Copy)]
pub struct Sky {
    start_color: Color<f32>,
    end_color: Color<f32>
}