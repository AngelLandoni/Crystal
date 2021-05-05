use std::sync::{Arc, RwLock};

use ecs::{
    DefaultWorld,
    EntityHandler,
    ComponentHandler,
    Entity,
    SystemHandler,
    Read,
    Write
};

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

    world.run(do_nothing);
}

fn do_nothing(healths: Read<Health>) {
    println!("Doing nothing");
}
   
