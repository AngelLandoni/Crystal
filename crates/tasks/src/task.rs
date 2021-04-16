use std::fmt::{Debug, Result, Formatter};

/// A trait to execute a work load or task.
pub trait Executable {
    fn execute(&self);
}

/// Defines a wrapper which contains the callback and some useful 
/// debug information. 
pub struct Task<T> where 
    T: Fn() + Sized + Sync
{
    /// A variable which contains the callback to be executed.
    callback: T,

    /// A variable which conatins 
    name: Option<String>
}

/// Provides constructors for `Task`. 
impl<T> Task<T> where
    T: Fn() + Sized + Sync
{
    /// Creates and returns a new `Task` with the provided closure.
    /// 
    /// # Arguments
    /// 
    /// `priority` - The priority of the task.
    /// `callback` - The work to get done
    pub fn new(callback: T) -> Self {
        Self {
            callback,
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
        callback: T,
        name: String) -> Self {
        
        Self {
            callback,
            name: Some(name)    
        }
    }
}

/// Provides execution to the `Task`.
impl<T> Executable for Task<T> where
    T: Fn() + Sized + Sync 
{
    /// Executes the current `Task`.
    fn execute(&self) {
       (self.callback)(); 
    }
}

/// Provide a debug function for the `Task`.
impl<T> Debug for Task<T> where
    T: Fn() + Send + Sync {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, r#"
[+] Worker:
    [*] id: {}
        "#,
        self.name.as_ref().unwrap_or(&"".to_string()))
    }
}