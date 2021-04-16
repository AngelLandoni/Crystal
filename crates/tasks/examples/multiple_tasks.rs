
use tasks::{Workers, Task, Dispatcher, Executable};

fn main() {
    // Create a new worker pool.
    let mut workers: Workers = Workers::default();
    workers.start();

    print!("{:?}", workers);

    loop {
        let mut vec: Vec<Box<dyn Executable + Send>> = Vec::new();
        for i in 1..10000 {
            vec.push(Box::new(Task::new(move || {
                println!("Job: {}", i);
            })));
        }
        workers.execute_batch(vec);
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}