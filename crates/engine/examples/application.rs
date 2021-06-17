use std::f64::consts::PI;

use engine::{
    scene::components::{Voxel, Transform},
    run_program,
    InitialConfig,
    KeyCode,
    Direction,
    Input,
    InputEvent,
    Camera,
    DevGui,
    cgmath::{Vector3, Point3, Rad, Angle, Quaternion},
    egui::{menu, Button, Color32, Label, TextStyle, TopPanel}
};

use ecs::{
    DefaultWorld,
    ComponentHandler,
    EntityHandler,
    UniqueRead,
    UniqueWrite,
    SystemHandler
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

pub fn top_bar_renderer_system(dev_gui: UniqueRead<DevGui>) {
    let dev_gui_r = dev_gui.read();
    if let Some(ctx) = &dev_gui_r.0 {
        // Add a menu bar at the top.
        TopPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                // Add the logo in the top left corner.
                ui.add(
                    Label::new("[Crystal]")
                        .text_color(Color32::from_rgb(255, 102, 0))
                        .text_style(TextStyle::Heading)
                );

                // Create 'File' menu.
                menu::menu(ui, "File", |ui| {
                    
                    ui.separator();

                    // Add the `Exit button`.
                    if ui.add(
                        Button::new("❌ Exit")
                            .text_color(Color32::RED)
                    ).clicked {
                        println!("Exit clicked");
                    }
                });
            });
        });
    }
}


const MOVEMENT_SPEED: f32 = 0.1;
const MOUSE_SENSIBILITY: f64 = 0.01;

pub fn input_camera_system(input: UniqueRead<Input>,
                           camera: UniqueWrite<Camera>,
                           fly_camera: UniqueRead<FlyCamera>) {
    let input_r = input.read();
    let fly_camera_r = fly_camera.read();
    let mut camera_w = camera.write();

    
  
    if input_r.keys_down.contains(&KeyCode::A) {
        let movement = fly_camera_r.right_direction * MOVEMENT_SPEED;
        camera_w.eye -= movement;
        camera_w.target -= movement;
    }

    if input_r.keys_down.contains(&KeyCode::D) {
        let movement = fly_camera_r.right_direction * MOVEMENT_SPEED;
        camera_w.eye += movement;
        camera_w.target += movement;
    }

    if input_r.keys_down.contains(&KeyCode::W) {
        let movement = fly_camera_r.direction * MOVEMENT_SPEED;
        camera_w.eye += movement; 
        camera_w.target += movement;
    }

    if input_r.keys_down.contains(&KeyCode::S) {
        let movement = fly_camera_r.direction * MOVEMENT_SPEED;
        camera_w.eye -= movement;
        camera_w.target -= movement; 
    }
}

pub fn calculate_input_fly_camera(data: (Direction, f64),
                                  input: UniqueRead<Input>,
                                  fly_camera: UniqueWrite<FlyCamera>,
                                  camera: UniqueWrite<Camera>) {

    let input_r = input.read();
    let mut fly_camera_w = fly_camera.write();
    let mut camera_w = camera.write();

    // Ignore mouse if there is not position movement.
    if !input_r.keys_down.contains(&KeyCode::A) &&
        !input_r.keys_down.contains(&KeyCode::D) &&
        !input_r.keys_down.contains(&KeyCode::W) &&
        !input_r.keys_down.contains(&KeyCode::S) {
        return;
    }

    // Check the direction of the rotation.
    match data.0 {
        Direction::Left => fly_camera_w.yaw += data.1 * 0.01,
        Direction::Right => fly_camera_w.yaw -= data.1 * 0.01,
        Direction::Top => {
            // Avoid rotation over 90 degs.
            if fly_camera_w.pitch < PI.floor() / 2.0 {
                fly_camera_w.pitch += data.1 * MOUSE_SENSIBILITY;
            } else {
                fly_camera_w.pitch = PI.floor() / 2.0;
            }
        }
        Direction::Bottom => {
            // Avoid rotation below 240 ges.
            if fly_camera_w.pitch > -PI.floor() / 2.0 { 
                fly_camera_w.pitch -= data.1 * MOUSE_SENSIBILITY;
            } else {
                fly_camera_w.pitch = -PI.floor() / 2.0;
            }
        }
    }

    let yaw_radians = Rad(fly_camera_w.yaw as f32);
    let pitch_radians = Rad(fly_camera_w.pitch as f32);

    let direction = Vector3 {
        x: Rad::sin(yaw_radians) * Rad::cos(pitch_radians),
        y: Rad::sin(pitch_radians),
        z: Rad::cos(yaw_radians) * Rad::cos(pitch_radians)
    };
 
    fly_camera_w.direction = direction;

    // Move the camera target.
    camera_w.target = Point3 { 
        x: direction.x + camera_w.eye.x,
        y: direction.y + camera_w.eye.y,
        z: direction.z + camera_w.eye.z
    }; 

    // Calculate the horizontal parallel direction.

    let parallel_direction = Vector3 {
        x: Rad::sin(Rad(fly_camera_w.yaw - PI / 2.0)) as f32,
        y: 0.0,
        z: Rad::cos(Rad(fly_camera_w.yaw - PI / 2.0)) as f32
    };

    fly_camera_w.right_direction = parallel_direction;
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

/// Handles all the input events.
///
/// # Arguments
///
/// `world` - The world used to store and handle data.
fn input(event: &InputEvent, world: &DefaultWorld) {
    match event {
            InputEvent::MouseMotion(direction, value) => {
                world.run_with_data(
                    calculate_input_fly_camera,
                    (direction.clone(), value.0)
                );
        }
        _ => ()
    }
}

/// Executes the application logic.
///
/// # Arguments
///
/// `world` - The world used to store and handle data.
fn tick(world: &DefaultWorld) {
    world.run(input_camera_system);
    world.run(top_bar_renderer_system);
}

/// Application entry point.
fn main() {
    // Trigger application main loop.
    match run_program(
        configure_application,
        input,
        tick,
        InitialConfig::default()
    ) {
        Ok(_) => return,
        Err(e) => println!("{}", e) 
    };
}   