use crate::task::Task;

/// Defines how a dispatcher should behave.
/// 
/// Normaly it is used to aftract a ThreadPool or something 
/// used to execute work in an specific order o way.
pub trait Dispatcher {
    /// Defines a entry point for the Dispatcher.
    fn start(&mut self);
 
    /// Defines a task exection by dynamic dispatching.
    fn execute_dyn(&self, task: Task);

    /// Defines a tasks execution by dynamic dispatching. 
    fn execute_batch(&self, tasks: Vec<Task>);
}