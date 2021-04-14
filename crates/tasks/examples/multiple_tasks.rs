use std::rc::Rc;

use tasks::{Workers, Task, Dispatcher, Priority, Executable};

fn main() {
    // Create a new worker pool.
    let workers: Workers = Workers::default();

    let task1 = Task::new(Priority::High, || {
        println!("Working!!!!!!");
    });
    
    let task2 = Task::new(Priority::Medium, || {
        println!("Working2!!!!");
    });

    let task3 = Task::new(Priority::Low, || {
        println!("Working3!!!!");
    });

    let mut vec: Vec<Rc<dyn Executable>> = Vec::new();

    vec.push(Rc::new(task1));
    vec.push(Rc::new(task2));
    vec.push(Rc::new(task3));

    workers.execute_batch(vec);

}