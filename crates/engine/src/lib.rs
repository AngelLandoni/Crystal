mod helpers;
mod workloads;
mod init;

mod basics;
pub use basics::window::Window;
mod graphics;
pub use graphics::egui::DevGui;

pub mod scene;
pub use scene::{
    camera::Camera,
    input::{Input, Direction, KeyCode, InputEvent}
};

pub use egui;
pub use cgmath;

use futures::executor::block_on;

use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    event::{Event, WindowEvent, KeyboardInput, ElementState, DeviceEvent},
};

use ecs::{
    ComponentHandler,
    DefaultWorld,
    UniqueRead,
    SystemHandler
};
use types::Size;
use log::{Log, Console, info};

use crate::{
    basics::window::update_window_with_new_size_system,
    graphics::{
        gpu::{Gpu, update_gpu_with_new_size_system},
        egui::mantain_egui_events
    },
    init::{initialize_window, initialize_world},
    workloads::{Workloads, run_workload},
    scene::{
        input::{
            Motion,
            WInitInputEvent,
            map_input_event,
            update_input_system,
            update_mouse_position_system
        },
        camera::update_camera_resize_system
    }
};

/// Defines the initial configuration for the application.
pub struct InitialConfig {
    /// The size of the window.
    window_size: Size<u32>,

    /// Contains a flag defining if the application should run in full screen
    /// or not.
    full_screen: bool,

    /// A flag which allows force log into the console.
    force_log: bool,
}

/// Defines the constants values for the window.
const DEFAULT_WIDTH_SIZE: u32 = 2024;
const DEFAULT_HEIGHT_SIZE: u32 = 1400;

/// Defines the default constructor for `InitialConfig`.
impl Default for InitialConfig {
    fn default() -> Self {
        Self {
            window_size: Size::new(DEFAULT_WIDTH_SIZE, DEFAULT_HEIGHT_SIZE),
            full_screen: false,
            force_log: false
        }
    }
}

/// Defines the callback for the configuration.
pub type ConfigFn = fn(&DefaultWorld);

/// Defines the input event callback.
pub type InputEventFn = fn(&InputEvent, &DefaultWorld); 

/// Defines the callback for the run per frame.
pub type TickFn = fn(&DefaultWorld);

/// Initializes the log system. 
fn initializes_log() {
    Log::init();
    Console::init();
}

/// Configures the resources and executes the engine main loop.
///
/// # Arguments
///
/// `config` - The general configuration callback.
/// `tick` - The tick callback.
/// `app_config` - The app configuration.
async fn run(config: ConfigFn,
             input: InputEventFn,
             tick: TickFn,
             app_config: InitialConfig) -> Result<(), String> {
    info("Initialize window and input handlers");
    
    // Create the window.
    let window_size: Size<u32> = app_config.window_size;
    let (window, event_loop) = match initialize_window("Shiny", window_size) {
        Ok(w) => w,
        Err(e) => return Err(e.to_string())
    };

    // Create the Gpu aftraction.
    let gpu: Gpu = match Gpu::default(&window).await {
        Ok(g) => g,
        Err(e) => return Err(e.to_string())
    };

    // Create a new world an inject the basic resources.
    let world = initialize_world(gpu, window, event_loop.create_proxy());

    // Configures the user's application.
    config(&world);

    info("Entering main run loop");
    // Trigger the main run loop.
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        mantain_egui_events(&event, &world);

        match event {

            Event::MainEventsCleared => {
                let window = world.get::<UniqueRead<Window>>();
                window.read().native_window.request_redraw();
            }

            // Events
            Event::WindowEvent { event, .. } => match event {
                
                WindowEvent::Resized(size) => {
                    let new_aspect = size.width as f32 / size.height as f32;
                    let size = Size::new(size.width, size.height);
                    world.run_with_data(
                        update_camera_resize_system,
                        new_aspect
                    );
                    world.run_with_data(
                        update_window_with_new_size_system,
                        size.clone()
                    );
                    world.run_with_data(
                        update_gpu_with_new_size_system,
                        size 
                    ); 
                }

                // User input down.
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                    virtual_keycode: Some(virtual_code),
                    state: ElementState::Pressed,
                    ..
                },
                ..
                } => {
                    let event = map_input_event(WInitInputEvent::KeyDown(virtual_code));
                    world.run_with_data(update_input_system, event);
                } 

                // User input up.
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                    virtual_keycode: Some(virtual_code),
                    state: ElementState::Released,
                    ..
                },
                ..
                } => {
                    let event = map_input_event(WInitInputEvent::KeyUp(virtual_code));
                    world.run_with_data(update_input_system, event);
                }

                WindowEvent::CursorMoved { position, .. } => {
                    world.run_with_data(update_mouse_position_system, (position.x, position.y));
                }

                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }

                _ => {}
            }

            Event::DeviceEvent { event, .. } => match event {
                
                // Maps the motion event to a engine compatible event.
                DeviceEvent::Motion {axis, value} => {
                    // Map the raw event to a typed one.
                    let direction = Direction::from_raw(axis, value);
                    input(
                        &InputEvent::MouseMotion(
                            direction,
                            Motion(value.abs())
                        ),
                        &world
                    );
                }

                _ => {}
            }

            // Redraw
            Event::RedrawRequested(_) => {
                // Run the render workload.
                run_workload(Workloads::Start, &world);
                // Send the flow to game lands.
                tick(&world);    
                // Render and sync everything else.
                run_workload(Workloads::Synchronize, &world);
                run_workload(Workloads::Render, &world);
                run_workload(Workloads::Commit, &world);
                run_workload(Workloads::End, &world);
            }            

            // We do not care about the rest of events.
            _ => (),
        }
    });
}

/// Runs the given program.
///
/// # Arguments
///
/// `config` - The function used to configure the world.
/// `input_event` - The function used to react to events.
/// `tick` - The funtion executed every frame.
pub fn run_program(config: ConfigFn,
                   input: InputEventFn,
                   tick: TickFn,
                   app_config: InitialConfig) -> Result<(), String> {
    // Initialize the log only on debug mode.
    //if cfg!(debug_assertions) || app_config.force_log {
        initializes_log();
    //}

    // Run the engine and lock the program there.
    block_on(run(config, input, tick, app_config))
}