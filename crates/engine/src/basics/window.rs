
/*use winit::{
    event_loop::{EventLoop},
    window::WindowBuilder,
    dpi::LogicalPosition
};

use types::Size;

use crate::{
    helpers::errors::InitError,
    scane::input::MousePosition
};

pub enum CustomEvent {
    RequestRedraw
}

/// Represents the window which contains the necessary information to render
/// over the user's screen.
pub struct Window {
    /// The size of the screen in user screen space.
    pub size: Size<u32>,

    /// The window provided by winit.
    pub native_window: winit::window::Window,
}

impl Window {
    /// Creates and returns a new instance of `Window`.
    ///
    /// # Arguments
    ///
    /// `title` - The window title.
    /// `size` - The window initial size.
    pub fn new(title: &str, size: Size<u32>) 
        -> Result<(Self, EventLoop<CustomEvent>), InitError> {
        // Create the event loop.
        let event_loop: EventLoop<CustomEvent> = EventLoop::with_user_event();
        
        // Create the new window.
        let native_window = match WindowBuilder::new()
            .with_title(title)
            .build(&event_loop) {
            Ok(w) => w,
            Err(_) => return Err(InitError::Window)
        };

        // Return the result window.
        Ok((Window { size, native_window }, event_loop))
    }
}

impl Window {
    /// Sets the positon of the cursor.
    ///
    /// # Arguments
    ///
    /// `position` - The new position for the cursor.
    pub fn set_cursor_position(&self, position: &MousePosition) {
        self.native_window.set_cursor_position(LogicalPosition::new(
            position.x,
            position.y
        )).unwrap();
    }
}*/
