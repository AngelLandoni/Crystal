use std::{
    rc::Rc,
    sync::Arc,
    thread::{JoinHandle, spawn}
};

use lockfree::queue::Queue;

use crate::{
    dispatch::Dispatcher,
    task::Executable
};

/// Just a handy wrapper of the task queue so we do not deal with 
/// large data types.
type TaskQueue = Arc<Queue<Arc<dyn Executable + Send + Sync>>>;

/// Defines a worker.
struct Worker {
    /// A variable used to control the worker (Thread).
    handle: JoinHandle<()>,

    /// An id that identify the worker.
    id: usize
}

/// Defines the characteristics of the `ThreadPool`.
pub struct WorkersDescriptor {
    /// Contains the number of workers needed.
    pub amount: usize,
    /// A name used to identify the `ThreadPool`
    pub name: Option<String>
}

/// Defines a `ThreadPool`.
/// 
/// This allows execute tasks in a pool of threads (workers)
pub struct Workers {
    /// Contains the information about the workers. 
    descriptor: WorkersDescriptor,

    /// Contains all the spawned threads. We do not need any return
    /// from the execution so avoiding that.
    workers: Vec<Worker>,

    /// The task queue shared across threads.
    /// 
    /// TODO(Angel): We could use a Box insted of a Rc and drop it 
    /// when the Workers is destroyed because the threads should be
    /// stoped before the Workers deletion.
    queue: TaskQueue 
}

/// Provides defaults constructors for `Workers`.
impl Workers {
    /// Creates and returns a new `Workers` using the provided 
    /// descriptor.
    pub fn new(descriptor: WorkersDescriptor) -> Self {
        Self {
            descriptor,
            workers: Vec::new(),
            queue: Arc::new(Queue::new())
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
            },
            workers: Vec::new(),
            queue: Arc::new(Queue::new())
        }
    }
}

/// Useful functions.
impl Workers {
    fn spawn_workers(&mut self) {
        // Copy the number of workers needed. 
        let number_of_workers = self.descriptor.amount;
        // Spawn all the workers.
        for i in 0..number_of_workers {
            // Get a clone of the reference to the queue to move that
            // into the thread. 
            let queue_ref: TaskQueue = self.queue.clone();
            // Create a new worker.
            let new_worker: Worker = Worker {
                handle: worker_loop(queue_ref), 
                id: i
            };
            // Send the worker to the pool.
            self.workers.push(new_worker);
        }
    }
}

/// Allow `Workers` to behave as a `Dispatche`.
impl Dispatcher for Workers {
    /// Create and deploy all the workers needed.
    fn start(&mut self) {
        self.spawn_workers();
    }

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

/// Generates and returns the worker main loop.
/// 
/// # Arguments
/// 
/// `task_queue` - The task queue referece to be moved into the loop.
fn worker_loop(task_queue: TaskQueue) -> JoinHandle<()> {
    spawn(move || {
        // Force move ownership.
        let t_queue = task_queue;

        loop {
            // Get a task from the queue, if there are not tasks to
            // do go to sleep.
            if let Some(task) = t_queue.pop() {
                task.execute();
            } else {
                // Go to sleep here.
            } 
        }
    })
}