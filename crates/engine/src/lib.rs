mod basics;
mod helpers;
mod graphics;

mod init;

use futures::executor::block_on;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use ecs::DefaultWorld;
use types::Size;
use log::{Log, Console, info};

use crate::{
    basics::window::Window,
    graphics::gpu::Gpu,
    init::{initialize_window, initialize_world}
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
const DEFAULT_WIDTH_SIZE: u32 = 1024;
const DEFAULT_HEIGHT_SIZE: u32 = 768;

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
    let mut world = initialize_world(gpu, window, event_loop.create_proxy());

    info("Entering main run loop");
    // Trigger the main run loop.
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            // Redraw
            Event::RedrawRequested(_) => {
                // Send the flow to game lands.
                //tick(&world);    
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
                   tick: TickFn,
                   app_config: InitialConfig) -> Result<(), String> {
    // Initialize the log only on debug mode.
    if cfg!(debug_assertions) || app_config.force_log {
        initializes_log();
    }

    // Run the engine and lock the program there.
    block_on(run(config, tick, app_config))
}