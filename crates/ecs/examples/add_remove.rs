use std::sync::{Arc, RwLock};

use ecs::{DefaultWorld, EntityHandler, ComponentHandler};
use tasks::{Workers, Task, Dispatcher, Executable};

struct Health(u32);
struct Commander;
struct IsPlayer;

fn main() {
    let mut world = DefaultWorld::default();

    world.register::<IsPlayer>();
    world.register::<Commander>();
    world.register::<Health>();

    let shared_world = Arc::new(world);

    // Create a new worker pool.
    let mut workers: Workers = Workers::default();
    workers.start();
    
    print!("{:?}", workers);

    loop {
        let mut vec: Vec<Box<dyn Executable + Send>> = Vec::new();
        for i in 1..50 {
            let c_world = shared_world.clone();
            vec.push(Box::new(Task::new(move || {
                c_world.add_entity((IsPlayer, Health(123)));
            })));
        }
        workers.execute_batch(vec);

        println!("Debug: {:?}", shared_world);
        std::thread::sleep(std::time::Duration::from_millis(1600));
    }
}
