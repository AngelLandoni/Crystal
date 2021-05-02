use std::sync::{Arc, RwLock};

use ecs::{DefaultWorld, EntityHandler, ComponentHandler, Entity};
use tasks::{Workers, Task, Dispatcher, Executable};

struct Health(u32);
struct Commander;
struct IsPlayer;
struct IsEnemy;

fn main() {
    let world = DefaultWorld::default();

    world.register::<IsPlayer>();
    world.register::<Commander>();
    world.register::<Health>();
    world.register::<IsEnemy>();

    for _ in 1..1000 {
        world.add_entity((IsPlayer, Health(123)));
        world.add_entity((Commander, IsEnemy, Health(333)));
    }

    world.remove_entity(Entity::new(3));

    world.add_entity((IsEnemy,));
}
   
