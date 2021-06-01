mod basics;

use futures::executor::block_on;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use ecs::DefaultWorld;
use types::Size;

/// Defines the initial configuration for the application.
pub struct InitialConfig {
    /// The size of the window.
    window_size: Size<u32>,

    /// Contains a flag defining if the application should run in full screen
    /// or not.
    full_screen: bool,
}

/// Defines the constants values for the window.
const DEFAULT_WIDTH_SIZE: u32 = 1024;
const DEFAULT_HEIGHT_SIZE: u32 = 768;

/// Defines the default constructor for `InitialConfig`.
impl Default for InitialConfig {
    fn default() -> Self {
        Self {
            window_size: Size::new(DEFAULT_WIDTH_SIZE, DEFAULT_HEIGHT_SIZE),
            full_screen: false
        }
    }
}

/// Defines the callback for the configuration.
pub type ConfigFn = fn(&DefaultWorld);

/// Defines the callback for the run per frame.
pub type TickFn = fn(&DefaultWorld);

/// Configures the resources and executes the engine main loop.
async fn run(config: ConfigFn,
             tick: TickFn,
             app_config: InitialConfig) -> Result<(), String> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
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
    // Run the engine and lock the program there.
    block_on(run(config, tick, app_config))
}