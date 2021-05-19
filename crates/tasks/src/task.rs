use std::{
    sync::Arc,
    fmt::{Debug, Result, Formatter}
};

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

/// A trait to execute a work load or task.
pub trait Executable {
    fn execute(&self);
}

pub type Task = Box<dyn FnOnce() + Send + 'static>;
/*
/// Defines a wrapper which contains the callback and some useful 
/// debug information. 
pub struct Task {
    /// A variable which contains the callback to be executed.
    callback: Box<dyn CbFunc + Send>,

    /// A variable which conatins 
    name: Option<String>
}

/// Provides constructors for `Task`. 
impl Task {
    /// Creates and returns a new `Task` with the provided closure.
    /// 
    /// # Arguments
    /// 
    /// `priority` - The priority of the task.
    /// `callback` - The work to get done
    pub fn new(callback: dyn CbFunc + Send) -> Self {
        Self {
            callback: Box::new(callback),
            name: None
        }
    }

    /// Creates and returns a new `Task` with the provided paremters.
    /// 
    /// # Arguments
    /// 
    /// `priority` - The priority of the task.
    /// `callback` - The work to get done.
    /// `name` - The debug name used to track the task.
    pub fn new_debug(
        callback: dyn CbFunc + Send,
        name: String) -> Self {
        
        Self {
            callback: Box::new(callback),
            name: Some(name)    
        }
    }
}

/// Provides execution to the `Task`.
impl Executable for Task {
    /// Executes the current `Task`.
    fn execute(&self) {
        self.callback.call_box();
    }
}

/// Provide a debug function for the `Task`.
impl Debug for Task {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, r#"
[+] Worker:
    [*] id: {}
        "#,
        self.name.as_ref().unwrap_or(&"".to_string()))
    }
}*/