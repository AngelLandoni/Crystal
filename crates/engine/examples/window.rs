use cgmath::Vector3;

use engine::{
    run_program,
    InitialConfig
};

use ecs::{DefaultWorld, ComponentHandler};

use log::{
    Log,
    Console,
    info
};

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
}

/// Executes the application logic.
///
/// # Arguments
///
/// `world` - The world used to store and handle data.
fn tick(world: &DefaultWorld) {
    info("Tick");
}

/// Application entry point.
fn main() {
    Log::init();
    Console::init();
    info("Running engine");

    // Trigger application main loop.
    match run_program(
        configure_application,
        tick,
        InitialConfig::default()
    ) {
        Ok(_) => return,
        Err(e) => println!("{}", e) 
    };

    info("Stopped engine");
}   