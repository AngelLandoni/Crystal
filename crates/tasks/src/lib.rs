mod dispatch;
pub use dispatch::Dispatcher;

mod task;
pub use task::{Executable, Task};

mod workers;
pub use workers::{Workers, WorkersDescriptor};

extern crate num_cpus;