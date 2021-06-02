use winit::{
    event_loop::{EventLoop}
};

use types::Size;

use crate::{
    basics::window::{Window, CustomEvent},
    helpers::errors::InitError
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