use crate::priority::Priority;

/// A trait to execute a work load or task.
pub trait Executable {
    fn execute(&self);
}

/// Defines a wrapper which contains the callback and some useful 
/// debug information. 
pub struct Task<T> where 
    T: Fn() + Sized + Sync
{
    /// A variable which contains the priority of the task.
    priority: Priority,
    
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
    pub fn new(priority: Priority, callback: T) -> Self {
        Self {
            priority,
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
        priority: Priority,
        callback: T,
        name: String) -> Self {
        
        Self {
            priority,
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