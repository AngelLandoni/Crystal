mod dispatch;
pub use dispatch::Dispatcher;

mod priority;
pub use priority::Priority;

mod task;
pub use task::{Executable, Task};

mod workers;
pub use workers::{Workers, WorkersDescriptor};

extern crate num_cpus;

/*fn testing() {
    // Creates a new thread pool of 4 threads.
    let thread_pool = :new(4);

    let task = Task::new(|| {
        println!("Something in background in some thread.");
    });

    thread_pool.execute(task);    
}*/