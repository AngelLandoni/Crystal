use std::sync::{Arc, Mutex};

use winit::{
    event::Event,
    event_loop::EventLoopProxy
};

use epi::{RepaintSignal, Frame};
use egui::{FontDefinitions, CtxRef};
use egui_wgpu_backend::{RenderPass};
use egui_winit_platform::{Platform, PlatformDescriptor};

use ecs::{DefaultWorld, UniqueRead, UniqueWrite, ComponentHandler};

use crate::{
    basics::window::CustomEvent,
    graphics::gpu::Gpu,
    basics::window::Window,
};

/// Represents a reference to the gui context
pub struct DevGui(pub Option<CtxRef>);

pub struct InternalRepaintSignal(Mutex<EventLoopProxy<CustomEvent>>);

impl RepaintSignal for InternalRepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(CustomEvent::RequestRedraw).ok();
    }
}

/// Used to alert EGui that should be repainted.
pub struct EGuiRepaintSignal(pub Arc<InternalRepaintSignal>);

/// Represtns the EGui instance.
///
/// Contains all the needed resources to control egui.
pub struct EGui {
    /// Contains the EGui context.
    pub platform: Platform,

    /// EGui's render pass.
    pub render_pass: RenderPass
}

/// Initializes and sets the EGui instance into the world.
///
/// # Arguments
///
/// `Gpu` - The Gpu used to extract the device.
/// `window` - The main window.
/// `world` - The world where EGui will be added.
pub fn initialize_egui(
    gpu: &Gpu,
    window: &Window,
    world: &DefaultWorld,
    e_loop_proxy: EventLoopProxy<CustomEvent>) {
    // Create the EGui platform.
    let platform: Platform = Platform::new(PlatformDescriptor {
        physical_width: window.size.width as u32,
        physical_height: window.size.height as u32,
        scale_factor: window.native_window.scale_factor(),
        font_definitions: FontDefinitions::default(),
        style: Default::default(),
    });

    // Create the render back-end.
    let render_pass: RenderPass = RenderPass::new(
        &gpu.device,
        gpu.swap_chain_descriptor.format
    );

    // Setup the new egui instance.
    let egui: EGui = EGui {
        platform,
        render_pass
    };

    // Create the signal.
    let repaint_signal = EGuiRepaintSignal(
        Arc::new(InternalRepaintSignal(Mutex::new(e_loop_proxy)))
    );

    // Add the demo app just temporal.
    world.register_unique(DevGui(None));
    // Add the repaint signal for egui.
    world.register_unique(repaint_signal);
    // Add the egui instance to the world.
    world.register_unique(egui);
}

/// Propagate winit events to egui.
pub fn mantain_egui_events_system(
    event: &Event<CustomEvent>,
    egui: UniqueWrite<EGui>) {
    // Send the event to egui.
    egui.write().platform.handle_event(event);
}

/// Generates the EGui context and inserts that into the world.
pub fn mantain_egui_context_system(
    egui: UniqueWrite<EGui>,
    dev_gui: UniqueWrite<DevGui>) {
    let mut egui_w = egui.write();
    let mut dev_gui_w = dev_gui.write();

    // Tell egui we want a new frame.
    egui_w.platform.begin_frame(); 
    // Create the context and inject that into the world to be used 
    // in client side.
    let context = egui_w.platform.context();
    // Send the context to the world.
    dev_gui_w.0 = Some(context);
}