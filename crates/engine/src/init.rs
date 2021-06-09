use winit::{
    event_loop::{EventLoop, EventLoopProxy}
};

use types::Size;
use ecs::{DefaultWorld, ComponentHandler};
use log::info;

use crate::{
    basics::window::{Window, CustomEvent},
    helpers::errors::InitError,
    graphics::{
        gpu::Gpu,
        pipelines::bind_groups::locals_bind_group::initialize_locals
    }
};

/// Creates and returns a new Window and EventLoop.
///
/// # Arguments
///
/// `name` - The window name.
/// `size` - The window initial size.
pub fn initialize_window(name: &str, size: Size<u32>)
    -> Result<(Window, EventLoop<CustomEvent>), InitError> {
    Window::new(name, size)
}


/// Creates and returns a new instance of World.
///
/// The function setups all the necessary resources and components.
/// It moves the arguments, so after calling this function the only possibility
/// to access them is using the world and ask for that resource.
///
/// # Arguments 
///
/// `gpu` - The gpu to be setted as a resource in the world.
/// `window` - The main window used which contains the attached surface. 
pub fn initialize_world(
    gpu: Gpu,
    window: Window,
    e_loop_proxy: EventLoopProxy<CustomEvent>) -> DefaultWorld {
    info("Initializing world");

    // Creates a mutable wo =rld.
    let world: DefaultWorld = DefaultWorld::default();

    // initialize all the locals, this should be performed before the pipelines
    // due the pipelines will need the locals buffer.
    initialize_locals(&gpu, &world);

    // Initialize basic pipelines.
    /*initialize_pipelines(&gpu, &world);

    // Initialize egui.
    initialize_egui(&gpu, &window, &world, e_loop_proxy);

    // Create and set the depth texture.
    let depth_texture: Texture = gpu.create_depth_texture();
    world.add_unique(DepthTexture(depth_texture)).unwrap();*/
    
    // Register all the unique resources.
    world.register_unique(gpu);
    world.register_unique(window);
    
    // Register the CommandBufferQueue which is used to send all the commands
    // that are generated from the different renderers.
    /*world.add_unique(CBQ::new(MAX_NUMBER_OF_COMMANDS_PER_CALL)).unwrap();
    
    // Creates an empty swap chain buffer only to register the needed component
    // and allow the system to update it in the future (first frame ever).
    // There is a way to avoid this using new_uninit but it is a unstable 
    // feature as soon as the feature is release we could replace this.
    world.add_unique(CurrentSwapChainOutput::new(None)).unwrap();
    
    // Registers the camera.
    world.add_unique(Camera::default()).unwrap();
    
    // Create a new default input, this contains the actual input state, which
    // keys are pressed.
    world.add_unique(Input::default()).unwrap();
    
    // Create a new MousePosition this contains the actual mouse position.
    world.add_unique(MousePosition::default()).unwrap();

    // Register all the work loads.
    create_workloads(&world);*/

    info("World initialized");

    world
}