use std::rc::Rc;

use crate::{
    dispatch::Dispatcher,
    task::Executable
};

/// Defines the characteristics of the `ThreadPool`.
#[derive(Debug)]
pub struct WorkersDescriptor {
    /// Contains the number of workers needed.
    pub amount: usize,
    /// A name used to identify the `ThreadPool`
    pub name: Option<String>
}

/// Defines a `ThreadPool`.
/// 
/// This allows execute tasks in a pool of threads (workers)
#[derive(Debug)]
pub struct Workers {
    /// Contains the information about the workers. 
    descriptor: WorkersDescriptor,
}

/// Provides defaults constructors for `Workers`.
impl Workers {
    /// Creates and returns a new `Workers` using the provided 
    /// descriptor.
    pub fn new(descriptor: WorkersDescriptor) -> Self {
        Self {
            descriptor
        }
    }
}

/// Provides a default constructor for `Workers`.Workers
/// 
/// The amount of workers will be calculated based on the number
/// of CPU that the host provides (number of cores * 2).
impl Default for Workers {
    /// Creates and returns a new `Worker` based on the default
    /// configuration.
    fn default() -> Self {
        // Create the `Workers`.
        Self {
            descriptor: WorkersDescriptor {
                // Get the number of CPUs and calculate the amount of 
                // workers needed.
                amount: num_cpus::get() * 2,
                name: None
            }
        }
    }
}

/// Allow `Workers` to behave as a `Dispatche`.
impl Dispatcher for Workers {
    /// Executes the provided task as soon as possible.
    ///
    /// # Arguments
    /// 
    /// `task` - The task to be executed.
    fn execute<T: Executable>(&self, task: T) {
        // For now the task is executed but it must be 
        // dispatched into the concurrent queue.
        task.execute();
    }

    /// Executes the provided task by dynamic dispatching as soon as
    /// possible.
    /// 
    /// # Arguments
    /// 
    /// `task` -The task to be executed.
    fn execute_dyn(&self, task: Rc<dyn Executable>) {
        task.execute();
    }

    /// Executes the provided tasks by dynamic dispatching as soon as
    ///  possible.
    ///
    /// # Arguments
    /// 
    /// `task` -The task to be executed 
    fn execute_batch(&self, tasks: Vec<Rc<dyn Executable>>) {
        tasks.iter().for_each(|task| {
            task.execute();
        });
    }
}