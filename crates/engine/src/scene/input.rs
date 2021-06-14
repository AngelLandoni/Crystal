use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    fmt
};
use ecs::{
    DefaultWorld,
    UniqueRead,
    UniqueWrite,
    ComponentHandler
};
use log::warning;

use crate::basics::window::Window;

/// Defines all the possible directions in a 2D world.
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Top,
    Bottom
}

/// Provides to the `Direction` the ability to print the content on the shell.
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dir = match self {
            Direction::Left => "Left",
            Direction::Right => "Right",
            Direction::Top => "Top",
            Direction::Bottom => "Bottom"
        };
        write!(f, "{}", dir)
    }
}

/// Defines all the initializers available for `Motion`.
impl Direction {
    /// Creates and returns a new `Direction` from raw data.
    ///
    /// The `axis` parameter should be 0(X) or 1(Y) and the direction should
    /// be negative or positive.
    ///
    /// # Arguments
    ///
    /// `axis` - The X or Y direction in integer form.
    /// `direction` - The direction for X or Y (negative or positive).
    pub fn from_raw(axis: u32, direction: f64) -> Self {
        // 1 means Y axis.
        if axis == 1 {
            if direction < 0.0 {
                return Direction::Top;
            } else {
                return Direction::Bottom;
            }
        } else if axis == 0 {
            if direction > 0.0 {
                return Direction::Right;
            } else {
                return Direction::Left;
            }
        }
        warning("Direction could not be constructed due axis does not exists");
        panic!();
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Motion(pub f64);

impl Hash for Motion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{}", self.0).hash(state);
    }
}

impl Eq for Motion {}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum InputEvent {
    KeyDown(KeyCode),
    KeyUp(KeyCode),
    MouseMotion(Direction, Motion)
}

pub struct InputSystem {
    pub key_event: InputEvent,
}

/// Update the input key state in the world using the event passed by parameter.
pub fn update_input_system(event: InputEvent, input: UniqueWrite<Input>) {

    // Check the type of event and insert that to the current input state.
    match event {
        InputEvent::KeyDown(key_code) => {
            // Inserts the sate of the key in the in the down.
            input.write().keys_down.insert(key_code);
        }

        InputEvent::KeyUp(key_code) => {
            // If the key is up remove it form the do
            input.write().keys_down.remove(&key_code);
        }

        // We only going to take care of the keyboard input.
        _ => {}
    }
}

/// The mouse position in the world.
pub struct MousePosition {
    pub x: f64,
    pub y: f64
}

impl Default for MousePosition {
    /// Creates a new instance of `MousePosition` setting the position to 0.
    fn default() -> MousePosition {
        Self {
            x: 0.0,
            y: 0.0
        }
    }
}

/// Upadtes the mouse position in the world.
pub fn update_mouse_position_system(
    (x, y): (f64, f64),
    mouse_position: UniqueWrite<MousePosition> ) {
    let mut mouse_position_write = mouse_position.write();
    // Set the position.
    mouse_position_write.x = x;
    mouse_position_write.y = y;
}

pub enum WInitInputEvent {
    KeyDown(winit::event::VirtualKeyCode),
    KeyUp(winit::event::VirtualKeyCode),
}

#[derive(Default, Clone)]
pub struct Input {
    pub keys_down: HashSet<KeyCode>,
}

pub fn map_input_event(virtual_key_code: WInitInputEvent) -> InputEvent {
    match virtual_key_code {
        WInitInputEvent::KeyDown(virtual_key) =>
            InputEvent::KeyDown(virtual_key_to_keycode(virtual_key)),
        WInitInputEvent::KeyUp(virtual_key) =>
            InputEvent::KeyUp(virtual_key_to_keycode(virtual_key)),
    }
}

/// Contains the range of the position of the letters in the winit environment.
const WINIT_KEYCODE_LETTERS_RANGE: std::ops::Range<u32> = 10..35;
/// Contains the range of the position of the arrows in the winit environment.
const WINIT_KEYCODE_ARROWS_RANGE: std::ops::Range<u32> = 70..74;

/// Contains the range of the position of the letters in mystic environment.
const MYSTIC_KEYCODE_LETTERS_DIFF_OFFSET: u32 = 10;

#[repr(u32)]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum KeyCode {
    // Letters
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Arrow keys
    Left,
    Up,
    Right,
    Down,

    Unknown,
}

impl KeyCode {
    fn from_u32(value: u32) -> Self {
        // Checks if the value is out of range.
        if value >= KeyCode::Unknown as u32 { return KeyCode::Unknown; }
        // Get the enum representation of the number and return it.
        unsafe { std::mem::transmute(value) }
    }
}

/// Coverts from winit key code to mystic key code.
pub fn virtual_key_to_keycode(virutal_key: winit::event::VirtualKeyCode) -> KeyCode {
    let key_value: u32 = virutal_key as u32;
    // Check if it is on the letters range.
    if WINIT_KEYCODE_LETTERS_RANGE.contains(&key_value) {
        return KeyCode::from_u32(virutal_key as u32 - MYSTIC_KEYCODE_LETTERS_DIFF_OFFSET);
    }
    // Return the default key code.
    KeyCode::Unknown
}

/// Hides the cursor.
///
/// # Arguments
///
/// `world` - The world where the data is stored.
pub fn hide_cursor(world: &DefaultWorld) {
    // Get the window from the world.
    let w: UniqueRead<Window> = world.get::<UniqueRead<Window>>(); 
    // Hide the cursor.
    w.read().native_window.set_cursor_visible(false);
}

/// Shows the cursor.
///
/// # Arguments
///
/// `world` - The world where the data is stored.
pub fn show_cursor(world: &DefaultWorld) {
    // Get the window from the world.
    let w: UniqueRead<Window> = world.get::<UniqueRead<Window>>(); 
    // Hide the cursor.
    w.read().native_window.set_cursor_visible(true);
}