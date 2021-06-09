use cgmath::{Vector3, Quaternion};

use engine::{
    scene::components::{Voxel, Transform},
    run_program,
    InitialConfig
};

use ecs::{
    DefaultWorld,
    ComponentHandler,
    EntityHandler
};

use log::info;

/// Represents a debug camera.
pub struct FlyCamera {
    pub yaw: f64,
    pub pitch: f64,
    pub direction: Vector3<f32>,
    pub right_direction: Vector3<f32>
}

impl Default for FlyCamera {
    /// Creates and returns a new `FlyCamera`.
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            direction: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            right_direction: Vector3 { x: 0.0, y: 0.0, z: 0.0 }
        }
    }
}

/// Configures the application.
///
/// # Arguments
///
/// `world` - The world used to store and handle data.
fn configure_application(world: &DefaultWorld) {
    // Adds the fly camera information.
    world.register_unique(FlyCamera::default());

    for i in 1..10 {
        for j in 1..10 {
            let transform = Transform {
                position: Vector3 {
                    x: 2.0 * (i as f32),
                    y: 0.0,
                    z: 2.0 * (j as f32)
                },
                scale: Vector3 { x: 1.0, y: 1.0, z: 1.0 },
                rotation: Quaternion { 
                    v: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                    s: 0.0
                }
            };
            world.add_entity((
                Voxel::rand_color(),
                transform
            ));
        }
    }
}

/// Executes the application logic.
///
/// # Arguments
///
/// `world` - The world used to store and handle data.
fn tick(_world: &DefaultWorld) {
    info("Tick");
}

/// Application entry point.
fn main() {
    // Trigger application main loop.
    match run_program(
        configure_application,
        tick,
        InitialConfig::default()
    ) {
        Ok(_) => return,
        Err(e) => println!("{}", e) 
    };
}   