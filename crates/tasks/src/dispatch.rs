use std::rc::Rc;

use crate::task::Executable;

/// Defines how a dispatcher should behave.
/// 
/// Normaly it is used to aftract a ThreadPool or something 
/// used to execute work in an specific order o way.
pub trait Dispatcher {
    /// Defines a task execution.
    fn execute<T: Executable>(&self, task: T);

    /// Defines a task exection by dynamic dispatching.
    fn execute_dyn(&self, task: Rc<dyn Executable>);

    /// Defines a tasks execution by dynamic dispatching. 
    fn execute_batch(&self, tasks: Vec<Rc<dyn Executable>>);
}