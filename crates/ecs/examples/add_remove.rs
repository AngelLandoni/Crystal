use std::sync::Arc;

use ecs::{DefaultWorld, EntityHandler};
use tasks::{Workers, Task, Dispatcher, Executable};

struct Health(u32);
struct IsPlayer;

fn main() {
    let world = Arc::new(DefaultWorld::default());

    // Create a new worker pool.
    let mut workers: Workers = Workers::default();
    workers.start();
    
    print!("{:?}", workers);

    loop {
        let mut vec: Vec<Box<dyn Executable + Send>> = Vec::new();
        for i in 1..50 {
            let c_world = world.clone();
            vec.push(Box::new(Task::new(move || {
                c_world.add_entity((IsPlayer, ));
            })));
        }
        workers.execute_batch(vec);

        println!("Debug: {:?}", world);
        std::thread::sleep(std::time::Duration::from_millis(1600));
    }
}